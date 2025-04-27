use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use regex::bytes::Regex;
use seq_io::fastq::Record;
use seq_io::parallel::parallel_fastq;
use std::io::Write;

mod arg;
use arg::{Cli, Commands};
mod initialise;
mod inverted;
mod output;
mod quality;
mod summarise;
mod tune;
use clap::Parser;
use initialise::{create_reader, create_writer, parse_patterns_file};
use std::io::{Error, ErrorKind};
use serde_json::json;
use regex::bytes::Regex as BytesRegex; // Alias to avoid confusion

// Main entry point for the grepq tool.
// This file handles CLI argument parsing, SQL database connection (if enabled),
// and dispatches commands to the appropriate module functions.

// adding a comment to simulate a change
// adding a comment to simulate a change
// adding a comment to simulate a change
// adding a comment to simulate a change
// adding a comment to simulate a change

fn main() {
    // Parse command line arguments using clap.
    let cli = Cli::parse();

    // Invoked as: `$ my-app --markdown-help`
    if cli.markdown_help {
        clap_markdown::print_help_markdown::<Cli>();
    }
    // Set up SQL database connection if writing SQL output and no command is given.
    let db_conn = if cli.write_sql && cli.command.is_none() {
        // If pattern file is JSON, check for qualityEncoding.
        let conn = if cli.patterns.ends_with(".json") {
            let pattern_data: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&cli.patterns).unwrap()).unwrap();
            if pattern_data["regexSet"]["qualityEncoding"].is_null() {
                output::create_sqlite_db().unwrap()
            } else {
                output::create_sqlite_db_with_quality().unwrap()
            }
        } else {
            output::create_sqlite_db().unwrap()
        };
        // Write regex patterns and file info to the database.
        output::write_regex_to_db(&conn, &cli.patterns, &cli.file).unwrap();
        Some(conn)
    } else {
        None
    };

    // Dispatch commands: tune, summarise, or inverted.
    match &cli.command {
        Some(Commands::Tune(tune)) => {
            // Execute tuning command.
            tune::run_tune(&cli, tune.num_matches, tune.include_count).unwrap();
            return;
        }
        Some(Commands::Summarise(summarise)) => {
            // Execute summarisation command.
            summarise::run_summarise(&cli, summarise.include_count).unwrap();
            return;
        }
        Some(Commands::Inverted) => {
            // Execute the inverted command.
            inverted::run_inverted(&cli);
            return;
        }
        None => {}
    }

    // Default processing: parse patterns file and prepare for record filtering.
    let (
        regex_set,
        header_regex,
        minimum_sequence_length,
        minimum_quality,
        quality_encoding,
        regex_names,
        _,
    ) = parse_patterns_file(&cli.patterns)
        .map_err(|e| Error::new(ErrorKind::Other, e))
        .unwrap();

    // Check if the pattern file is a text file.
    let is_text_file = cli.patterns.ends_with(".txt");

    // Store quality encoding as a string slice for further processing.
    let quality_encoding = quality_encoding.as_deref();

    // Ensure the number of regex patterns matches the number of regex names.
    assert_eq!(
        regex_set.patterns().len(),
        regex_names.len(),
        "The number of regex patterns and regex names must match."
    );

    // Compile the optional header regex filter.
    let header_regex = header_regex.map(|re: String| Regex::new(&re).unwrap());

    // Create input reader and output writer.
    let reader = create_reader(&cli);
    let mut writer = create_writer(&cli);

    // Extract output mode flags from CLI options.
    let with_id = cli.with_id;
    let with_full_record = cli.with_full_record;
    let with_fasta = cli.with_fasta;
    let count = cli.count;
    let bucket = cli.bucket;

    // Determine which filters to apply.
    let check_seq_len = minimum_sequence_length.is_some();
    let check_qual = minimum_quality.is_some();
    let check_header = header_regex.is_some();

    // Initialize reusable buffers.
    let mut seq_buffer = Vec::new();
    let mut qual_buffer = Vec::new();
    let mut head_buffer = Vec::new();

    if count {
        // Counting mode: Count records that match filter criteria.
        let mut match_count = 0;
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // Worker thread: Apply filter checks for sequence length, quality, header, and regex.
                *found = false;
                let seq_len_check = !check_seq_len
                    || record.seq().len() >= minimum_sequence_length.unwrap() as usize;
                let qual_check = !check_qual
                    || average_quality(
                        record.qual(),
                        quality_encoding.unwrap_or("Phred+33"),
                    ) >= minimum_quality.unwrap();
                let header_check =
                    !check_header || header_regex.as_ref().unwrap().is_match(record.head());
                let regex_check = regex_set.is_match(record.seq());

                if seq_len_check && qual_check && header_check && regex_check {
                    *found = true;
                }
            },
            |_, found| {
                // Main thread: Increment count when a record passes filters.
                if *found {
                    match_count += 1;
                }
                None::<()>
            },
        )
        .unwrap();
        // Write the final count to output.
        writeln!(writer, "{}", match_count).unwrap();
    } else {
        // Mode for writing records to files or stdout.
        let mut bucket_writers = if bucket {
            // Create separate bucket writers for each regex pattern.
            Some(
                regex_names
                    .iter()
                    .map(|name| {
                        let formatted_name = name.replace(' ', "-").replace('\'', "");
                        let suffix = if with_fasta {
                            "fasta"
                        } else if with_full_record {
                            "fastq"
                        } else {
                            ""
                        };
                        let file_name = if suffix.is_empty() {
                            formatted_name
                        } else {
                            format!("{}.{}", formatted_name, suffix)
                        };
                        let file = std::fs::File::create(file_name).unwrap();
                        (name.clone(), std::io::BufWriter::new(file))
                    })
                    .collect::<std::collections::HashMap<String, std::io::BufWriter<std::fs::File>>>(),
            )
        } else {
            None
        };

        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // Worker thread: Check the same filter criteria as in count mode.
                *found = false;
                let seq_len_check = !check_seq_len
                    || record.seq().len() >= minimum_sequence_length.unwrap() as usize;
                let qual_check = !check_qual
                    || quality::average_quality(
                        record.qual(),
                        quality_encoding.unwrap_or("Phred+33"),
                    ) >= minimum_quality.unwrap();
                let header_check =
                    !check_header || header_regex.as_ref().unwrap().is_match(record.head());
                let regex_check = regex_set.is_match(record.seq());

                if seq_len_check && qual_check && header_check && regex_check {
                    *found = true;
                }
            },
            |record, found| {
                // Main thread: Depending on flags, write the record in various formats.
                if *found {
                    if cli.write_sql && cli.command.is_none() {
                        // Process SQL write: extract match details and record quality statistics.
                        let mut matches_info = vec![];
                        for pattern in regex_set.patterns() {
                            let regex = BytesRegex::new(pattern).unwrap();
                            for matched in regex.find_iter(record.seq()) {
                                matches_info.push(json!({
                                    "pattern": pattern,
                                    "match": String::from_utf8_lossy(&record.seq()[matched.start()..matched.end()]).to_string(),
                                    "start": matched.start(),
                                    "end": matched.end()
                                }));
                            }
                        }
                        if let Some(ref db) = db_conn {
                            let avg_quality = quality_encoding
                                .map(|encoding| quality::average_quality(record.qual(), encoding))
                                .unwrap_or(0.0);
                            let (tnf, ntn) = quality::tetranucleotide_frequencies(record.seq(), cli.num_tetranucleotides);
                            let gc = quality::gc_content(record.seq());
                            let gc_int = gc.round() as i64;
                            let matches_json = serde_json::to_string(&matches_info).unwrap_or_else(|_| "[]".to_string());

                            // Insert record details into the SQLite database.
                            let insert_stmt = if quality_encoding.is_some() && !is_text_file {
                                "INSERT INTO fastq_data (header, sequence, quality, length, GC, GC_int, nTN, TNF, average_quality, variants) 
                                 VALUES (?1, ?2, ?3, ?4, ROUND(?5, 2), ?6, ?7, ?8, ROUND(?9, 2), ?10)"
                            } else {
                                "INSERT INTO fastq_data (header, sequence, quality, length, GC, GC_int, nTN, TNF, variants) 
                                 VALUES (?1, ?2, ?3, ?4, ROUND(?5, 2), ?6, ?7, ?8, ?9)"
                            };
                            db.execute(
                                insert_stmt,
                                rusqlite::params![
                                    String::from_utf8_lossy(record.head()),
                                    String::from_utf8_lossy(record.seq()),
                                    String::from_utf8_lossy(record.qual()),
                                    record.seq().len() as i64,
                                    gc,
                                    gc_int,
                                    ntn as i64,
                                    tnf,
                                    avg_quality,
                                    matches_json,
                                ],
                            ).unwrap();
                        }
                    }
                    
                    if let Some(ref mut bucket_writers) = bucket_writers {
                        // Write to separate bucket files based on matching regex patterns.
                        for (i, pattern) in regex_set.patterns().iter().enumerate() {
                            let regex = Regex::new(pattern).unwrap();
                            if regex.is_match(record.seq()) {
                                let writer = bucket_writers.get_mut(&regex_names[i]).unwrap();
                                if with_id {
                                    output::write_record_with_id(
                                        writer,
                                        &record,
                                        &mut head_buffer,
                                        &mut seq_buffer,
                                    );
                                } else if with_full_record {
                                    output::write_full_record(
                                        writer,
                                        &record,
                                        &mut head_buffer,
                                        &mut seq_buffer,
                                        &mut qual_buffer,
                                    );
                                } else if with_fasta {
                                    output::write_record_with_fasta(
                                        writer,
                                        &record,
                                        &mut head_buffer,
                                        &mut seq_buffer,
                                    );
                                } else {
                                    writer.write_all(record.seq()).unwrap();
                                    writer.write_all(b"\n").unwrap();
                                }
                            }
                        }
                    } else if with_id {
                        output::write_record_with_id(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                        );
                    } else if with_full_record {
                        output::write_full_record(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                            &mut qual_buffer,
                        );
                    } else if with_fasta {
                        output::write_record_with_fasta(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                        );
                    } else {
                        writer.write_all(record.seq()).unwrap();
                        writer.write_all(b"\n").unwrap();
                    }
                }
                None::<()>
            },
        )
        .unwrap();
    }
    
    // Ensure the database connection is properly closed, if one was established.
    if let Some(conn) = db_conn {
        conn.close().unwrap();
    }
}

// Inline function: average_quality calculates the average quality of a sequence.
#[inline(always)]
fn average_quality(qual: &[u8], encoding: &str) -> f32 {
    quality::average_quality(qual, encoding)
}
