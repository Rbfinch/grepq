// MIT License

// Copyright (c) 2024 - present Nicholas D. Crosbie

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// This module handles the summarisation command for grepq.
// It parses the patterns file, iterates over FASTQ records, applies filters,
// collects match statistics, writes to SQL if enabled, and prints summary output.

use crate::arg::Cli;
use crate::initialise::{create_reader, parse_patterns_file};
use crate::output;
use crate::quality;
use regex::bytes::Regex;
use seq_io::fastq::Record;
use serde_json::{self, json};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

pub fn run_summarise(cli: &Cli, include_count: bool) -> io::Result<()> {
    let patterns_path = &cli.patterns;

    // Parse the patterns file and extract settings.
    let (regex_set, header_regex, minimum_sequence_length, minimum_quality, quality_encoding, _, _) =
        parse_patterns_file(patterns_path).map_err(|e| io::Error::other(e.to_string()))?;

    // Compile header regex if provided.
    let header_regex = header_regex.map(|re| Regex::new(&re).unwrap());

    // Create a reader to stream input FASTQ records.
    let mut reader = create_reader(cli);

    // Initialize counters to store match counts and sub-match frequencies.
    let mut match_counts: HashMap<String, usize> = HashMap::new();
    let mut match_strings: HashMap<String, HashMap<String, usize>> = HashMap::new();

    // Initialize database connection if SQL output is enabled.
    let db_conn = if cli.write_sql {
        let conn = if cli.patterns.ends_with(".json") {
            let pattern_data: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(&cli.patterns).unwrap()).unwrap();
            if pattern_data["regexSet"]["qualityEncoding"].is_null() {
                output::create_sqlite_db().unwrap()
            } else {
                output::create_sqlite_db_with_quality().unwrap()
            }
        } else {
            output::create_sqlite_db().unwrap()
        };
        output::write_regex_to_db(&conn, &cli.patterns, &cli.file).unwrap();
        Some(conn)
    } else {
        None
    };

    // Process each FASTQ record in a loop.
    while let Some(result) = reader.next() {
        // Attempt to read a FASTQ record.
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                // Return error if file format is not supported.
                return Err(io::Error::other(format!(
                    "grepq only supports the fastq format. Check your input file.: {}",
                    e
                )));
            }
        };

        // Apply filters: sequence length, header pattern, and quality.
        let seq_len_check = minimum_sequence_length
            .map(|len| record.seq().len() >= len as usize)
            .unwrap_or(true);
        let header_check = header_regex
            .as_ref()
            .map(|re| re.is_match(record.head()))
            .unwrap_or(true);
        let qual_check = match minimum_quality {
            Some(min_q) => {
                quality::average_quality(
                    record.qual(),
                    quality_encoding.as_deref().unwrap_or("Phred+33"),
                ) >= min_q
            }
            None => true,
        };

        // If the record passes all filters, match the sequence against the regex set.
        if seq_len_check && header_check && qual_check {
            let mut matches_info = vec![];
            // Iterate over all regex matches for the sequence.
            for mat in regex_set.matches(record.seq()).into_iter() {
                // Obtain the pattern string and convert it if needed.
                let matched_pattern = regex_set.patterns()[mat].to_string();
                let converted_pattern = crate::initialise::convert_iupac_to_regex(&matched_pattern);
                *match_counts.entry(converted_pattern.clone()).or_insert(0) += 1;

                // Track sub-match frequencies for each pattern.
                let entry = match_strings.entry(converted_pattern.clone()).or_default();
                let matched = Regex::new(&regex_set.patterns()[mat])
                    .unwrap()
                    .find_iter(record.seq())
                    .next()
                    .unwrap();
                let matched_substring = &record.seq()[matched.start()..matched.end()];
                *entry
                    .entry(String::from_utf8_lossy(matched_substring).to_string())
                    .or_insert(0) += 1;

                // If SQL write is enabled, collect match info in JSON format.
                if cli.write_sql {
                    matches_info.push(json!({
                        "pattern": matched_pattern,
                        "start": matched.start(),
                        "end": matched.end(),
                        "match": String::from_utf8_lossy(matched_substring).to_string()
                    }));
                }
            }

            // Write record data and match details to the database if enabled.
            if cli.write_sql {
                let avg_quality = quality_encoding
                    .as_ref()
                    .map(|encoding| quality::average_quality(record.qual(), encoding.as_str()))
                    .unwrap_or(0.0);
                let (tnf, ntn) =
                    quality::tetranucleotide_frequencies(record.seq(), cli.num_tetranucleotides);
                let (ctnf, nctn) = quality::canonical_tetranucleotide_frequencies(
                    record.seq(),
                    cli.num_tetranucleotides,
                );
                let gc = quality::gc_content(record.seq());
                let gc_int = gc.round() as i64;

                if let Some(ref db) = db_conn {
                    let matches_json =
                        serde_json::to_string(&matches_info).unwrap_or_else(|_| "[]".to_string());

                    // Use different SQL statements based on pattern file type and quality encoding
                    if cli.patterns.ends_with(".json") {
                        let pattern_data: serde_json::Value =
                            serde_json::from_str(&std::fs::read_to_string(&cli.patterns).unwrap())
                                .unwrap();

                        if !pattern_data["regexSet"]["qualityEncoding"].is_null() {
                            // With quality encoding
                            db.execute(
                                "INSERT INTO fastq_data (header, sequence, quality, length, GC, GC_int, nTN, nCTN, TNF, CTNF, average_quality, variants) 
                                 VALUES (?1, ?2, ?3, ?4, ROUND(?5, 2), ?6, ?7, ?8, ?9, ?10, ROUND(?11, 2), ?12)",
                                rusqlite::params![
                                    String::from_utf8_lossy(record.head()),
                                    String::from_utf8_lossy(record.seq()),
                                    String::from_utf8_lossy(record.qual()),
                                    record.seq().len() as i64,
                                    gc,
                                    gc_int,
                                    ntn as i64,
                                    nctn as i64,
                                    tnf,
                                    ctnf,
                                    avg_quality,
                                    matches_json,
                                ],
                            ).unwrap();
                        } else {
                            // JSON without quality encoding
                            db.execute(
                                "INSERT INTO fastq_data (header, sequence, quality, length, GC, GC_int, nTN, nCTN, TNF, CTNF, variants) 
                                 VALUES (?1, ?2, ?3, ?4, ROUND(?5, 2), ?6, ?7, ?8, ?9, ?10, ?11)",
                                rusqlite::params![
                                    String::from_utf8_lossy(record.head()),
                                    String::from_utf8_lossy(record.seq()),
                                    String::from_utf8_lossy(record.qual()),
                                    record.seq().len() as i64,
                                    gc,
                                    gc_int,
                                    ntn as i64,
                                    nctn as i64,
                                    tnf,
                                    ctnf,
                                    matches_json,
                                ],
                            ).unwrap();
                        }
                    } else {
                        // Text file patterns - no quality encoding
                        db.execute(
                            "INSERT INTO fastq_data (header, sequence, quality, length, GC, GC_int, nTN, nCTN, TNF, CTNF, variants) 
                             VALUES (?1, ?2, ?3, ?4, ROUND(?5, 2), ?6, ?7, ?8, ?9, ?10, ?11)",
                            rusqlite::params![
                                String::from_utf8_lossy(record.head()),
                                String::from_utf8_lossy(record.seq()),
                                String::from_utf8_lossy(record.qual()),
                                record.seq().len() as i64,
                                gc,
                                gc_int,
                                ntn as i64,
                                nctn as i64,
                                tnf,
                                ctnf,
                                matches_json,
                            ],
                        ).unwrap();
                    }
                }
            }
        }
    }

    // Sort the collected match counts by descending order.
    let mut match_counts: Vec<_> = match_counts.into_iter().collect();
    match_counts.sort_by(|a, b| b.1.cmp(&a.1));

    // Process output based on the type of patterns file.
    if patterns_path.ends_with(".json") {
        // For JSON patterns, read the regex set name.
        let json: serde_json::Value = serde_json::from_reader(File::open(patterns_path)?)?;
        let regex_set_name = json["regexSet"]["regexSetName"]
            .as_str()
            .unwrap_or("Unknown");

        // JSON-specific processing for regex patterns with names and variants
        if matches!(cli.command.as_ref(), Some(crate::arg::Commands::Summarise(s)) if s.include_names)
        {
            println!("Regex Set Name: {}", regex_set_name);
        }

        let regex_array = json["regexSet"]["regex"].as_array().unwrap();
        let mut regex_matches = vec![];

        // Iterate over all regex definitions in the JSON.
        for regex in regex_array {
            let regex_string = regex["regexString"].as_str().unwrap();
            let converted_regex_string = crate::initialise::convert_iupac_to_regex(regex_string);
            let regex_name = regex["regexName"].as_str().unwrap_or("Unknown");
            let count = match_counts
                .iter()
                .find(|(pattern, _)| pattern == &converted_regex_string)
                .map(|(_, count)| count)
                .unwrap_or(&0);

            // Get the most frequent variant matches for the regex.
            let mut most_frequent_matches: Vec<_> = match_strings
                .get(&converted_regex_string)
                .map(|matches| {
                    let mut matches_vec: Vec<_> = matches.iter().collect();
                    matches_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
                    matches_vec
                })
                .unwrap_or_default();

            // Determine how many variant matches to include.
            let top_n = cli
                .command
                .as_ref()
                .and_then(|cmd| {
                    if let crate::arg::Commands::Summarise(summarise) = cmd {
                        if summarise.all_variants {
                            Some(usize::MAX) // Include all variants.
                        } else {
                            summarise.variants
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or(1);
            most_frequent_matches.truncate(top_n);

            let variants_array = regex["variants"]
                .as_array()
                .unwrap_or_else(|| Box::leak(Box::new(Vec::new())));
            let variants = variants_array;
            let most_frequent_matches_json: Vec<_> = most_frequent_matches
                .into_iter()
                .map(|(seq, count)| {
                    let variant_name = variants.iter().find_map(|variant| {
                        if variant["variantString"].as_str() == Some(seq) {
                            variant["variantName"].as_str().map(|s| s.to_string())
                        } else {
                            None
                        }
                    });
                    json!({"variant": seq, "count": count, "variantName": variant_name})
                })
                .collect();

            regex_matches.push(json!({
                "regexName": regex_name,
                "regexString": regex_string,
                "regexCount": count,
                "variants": most_frequent_matches_json
            }));

            // Print summary information for each regex.
            if matches!(cli.command.as_ref(), Some(crate::arg::Commands::Summarise(s)) if s.include_names)
            {
                if include_count {
                    println!("{} ({}): {}", regex_name, regex_string, count);
                } else {
                    println!("{} ({})", regex_name, regex_string);
                }
            } else if *count > 0 {
                if include_count {
                    println!("{}: {}", regex_string, count);
                } else {
                    println!("{}", regex_string);
                }
            }
        }

        // If JSON output is desired, write detailed match information to "matches.json".
        if let Some(crate::arg::Commands::Summarise(summarise)) = &cli.command {
            if summarise.json_matches && summarise.include_names && include_count {
                let json_output = json!({
                    "regexSet": {
                        "regexSetName": regex_set_name,
                        "regex": regex_matches
                    }
                });
                let file = File::create("matches.json")?;
                serde_json::to_writer(file, &json_output)?;
            }
        }
    } else {
        // For plain text patterns, print each pattern and its match count.
        for (pattern, count) in &match_counts {
            if count > &0 {
                if include_count {
                    println!("{}: {}", pattern, count);
                } else {
                    println!("{}", pattern);
                }
            }
        }
    }

    Ok(())
}
