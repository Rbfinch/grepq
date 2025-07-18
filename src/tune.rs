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

use crate::arg::Cli;
use crate::initialise::{create_reader, parse_patterns_file};
use crate::quality;
//use log::info;
use regex::bytes::Regex;
use seq_io::fastq::Record;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

// Main function to run the tune command
pub fn run_tune(cli: &Cli, num_matches: usize, include_count: bool) -> io::Result<()> {
    let patterns_path = &cli.patterns;

    // Parse the patterns file
    let (regex_set, header_regex, minimum_sequence_length, minimum_quality, quality_encoding, _, _) =
        parse_patterns_file(patterns_path).map_err(io::Error::other)?;

    let header_regex = header_regex.map(|re| Regex::new(&re).unwrap());
    let mut reader = create_reader(cli);

    let mut match_counts: HashMap<String, usize> = HashMap::new();
    let mut match_strings: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut total_matches = 0;

    // Iterate through each record in the reader
    let mut records_processed = 0;
    while let Some(result) = reader.next() {
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                return Err(io::Error::other(format!(
                    "grepq only supports the fastq format. Check your input file.: {}",
                    e
                )));
            }
        };

        // Check sequence length, header, and quality
        let seq_len_check = minimum_sequence_length
            .map(|len| record.seq().len() >= len as usize)
            .unwrap_or(true);
        let header_check = header_regex
            .as_ref()
            .map(|re| re.is_match(record.head()))
            .unwrap_or(true);
        let qual_check = minimum_quality
            .map(|min_q| {
                quality::average_quality(
                    record.qual(),
                    quality_encoding.as_deref().unwrap_or("Phred+33"),
                ) >= min_q
            })
            .unwrap_or(true);

        // If all checks pass, match the sequence against the regex set
        if seq_len_check && header_check && qual_check {
            for mat in regex_set.matches(record.seq()).into_iter() {
                let matched_pattern = regex_set.patterns()[mat].to_string();
                let converted_pattern = crate::initialise::convert_iupac_to_regex(&matched_pattern);
                *match_counts.entry(converted_pattern.clone()).or_insert(0) += 1;
                let entry = match_strings.entry(converted_pattern.clone()).or_default();
                let matched_substring = Regex::new(&regex_set.patterns()[mat])
                    .unwrap()
                    .find_iter(record.seq())
                    .next()
                    .unwrap();
                let matched_substring =
                    &record.seq()[matched_substring.start()..matched_substring.end()];
                *entry
                    .entry(String::from_utf8_lossy(matched_substring).to_string())
                    .or_insert(0) += 1;
                total_matches += 1;
                //  info!("Total matches: {}", total_matches);
                if total_matches >= num_matches {
                    break;
                }
            }
        }
        records_processed += 1;
        //  info!("Records processed: {}", records_processed);
        if total_matches >= num_matches || records_processed >= num_matches {
            break;
        }
    }

    let mut match_counts: Vec<_> = match_counts.into_iter().collect();
    match_counts.sort_by(|a, b| b.1.cmp(&a.1));

    // Handle JSON patterns file
    if patterns_path.ends_with(".json") {
        let json: serde_json::Value = serde_json::from_reader(std::fs::File::open(patterns_path)?)?;
        let regex_set_name = json["regexSet"]["regexSetName"]
            .as_str()
            .unwrap_or("Unknown");

        if matches!(cli.command.as_ref(), Some(crate::arg::Commands::Tune(t)) if t.include_names) {
            println!("Regex Set Name: {}", regex_set_name);
        }

        let regex_array = json["regexSet"]["regex"].as_array().unwrap();
        let mut regex_matches = vec![];

        for regex in regex_array {
            let regex_string = regex["regexString"].as_str().unwrap();
            let converted_regex_string = crate::initialise::convert_iupac_to_regex(regex_string);
            let regex_name = regex["regexName"].as_str().unwrap_or("Unknown");
            let count = match_counts
                .iter()
                .find(|(pattern, _)| pattern == &converted_regex_string)
                .map(|(_, count)| count)
                .unwrap_or(&0);

            let mut most_frequent_matches: Vec<_> = match_strings
                .get(&converted_regex_string)
                .map(|matches| {
                    let mut matches_vec: Vec<_> = matches.iter().collect();
                    matches_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
                    matches_vec
                })
                .unwrap_or_default();

            let top_n = cli
                .command
                .as_ref()
                .and_then(|cmd| {
                    if let crate::arg::Commands::Tune(tune) = cmd {
                        if tune.all_variants {
                            Some(usize::MAX) // Include all variants
                        } else {
                            tune.variants
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or(1); // Default to 1 variant

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

            if matches!(cli.command.as_ref(), Some(crate::arg::Commands::Tune(t)) if t.include_names)
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

        if let Some(crate::arg::Commands::Tune(tune)) = &cli.command {
            if tune.json_matches && tune.include_names && include_count {
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
