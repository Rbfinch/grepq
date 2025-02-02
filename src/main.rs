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
mod quality;
mod tune;
use clap::Parser;
use initialise::{create_reader, create_writer, parse_patterns_file};
use std::io::{self};

fn main() {
    let cli = Cli::parse();

    // Match the command and execute the corresponding function
    match &cli.command {
        Some(Commands::Tune(tune)) => {
            tune::run_tune(&cli, tune.num_records, tune.include_count).unwrap();
            return;
        }
        Some(Commands::Inverted) => {
            inverted::run_inverted(&cli);
            return;
        }
        None => {}
    }

    // Parse the patterns file
    let (
        regex_set,
        header_regex,
        minimum_sequence_length,
        minimum_quality,
        quality_encoding,
        regex_names,
        _,
    ) = parse_patterns_file(&cli.patterns)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .unwrap();
    assert_eq!(
        regex_set.patterns().len(),
        regex_names.len(),
        "The number of regex patterns and regex names must match."
    );
    let header_regex = header_regex.map(|re| Regex::new(&re).unwrap());
    let reader = create_reader(&cli);
    let mut writer = create_writer(&cli);

    let with_id = cli.with_id;
    let with_full_record = cli.with_full_record;
    let with_fasta = cli.with_fasta;
    let count = cli.count;
    let bucket = cli.bucket;

    let check_seq_len = minimum_sequence_length.is_some();
    let check_qual = minimum_quality.is_some();
    let check_header = header_regex.is_some();

    let mut seq_buffer = Vec::new();
    let mut qual_buffer = Vec::new();
    let mut head_buffer = Vec::new();

    if count {
        let mut match_count = 0;
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // runs in worker
                *found = false;
                let seq_len_check = !check_seq_len
                    || record.seq().len() >= minimum_sequence_length.unwrap() as usize;
                let qual_check = !check_qual
                    || average_quality(
                        record.qual(),
                        quality_encoding.as_deref().unwrap_or("Phred+33"),
                    ) >= minimum_quality.unwrap() as f32;
                let header_check =
                    !check_header || header_regex.as_ref().unwrap().is_match(record.head());
                let regex_check = regex_set.is_match(record.seq());

                if seq_len_check && qual_check && header_check && regex_check {
                    *found = true;
                }
            },
            |_, found| {
                // runs in main thread
                if *found {
                    match_count += 1;
                }
                None::<()>
            },
        )
        .unwrap();
        writeln!(writer, "{}", match_count).unwrap();
    } else {
        let mut bucket_writers = if bucket {
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
                    .collect::<std::collections::HashMap<_, _>>(),
            )
        } else {
            None
        };

        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // runs in worker
                *found = false;
                let seq_len_check = !check_seq_len
                    || record.seq().len() >= minimum_sequence_length.unwrap() as usize;
                let qual_check = !check_qual
                    || quality::average_quality(
                        record.qual(),
                        quality_encoding.as_deref().unwrap_or("Phred+33"),
                    ) >= minimum_quality.unwrap() as f32;
                let header_check =
                    !check_header || header_regex.as_ref().unwrap().is_match(record.head());
                let regex_check = regex_set.is_match(record.seq());

                if seq_len_check && qual_check && header_check && regex_check {
                    *found = true;
                }
            },
            |record, found| {
                // runs in main thread
                if *found {
                    if let Some(ref mut bucket_writers) = bucket_writers {
                        for (i, pattern) in regex_set.patterns().iter().enumerate() {
                            let regex = Regex::new(pattern).unwrap();
                            if regex.is_match(record.seq()) {
                                let writer = bucket_writers.get_mut(&regex_names[i]).unwrap();
                                if with_id {
                                    // With ID mode
                                    write_record_with_id(
                                        writer,
                                        &record,
                                        &mut head_buffer,
                                        &mut seq_buffer,
                                    );
                                } else if with_full_record {
                                    // With full record mode
                                    write_full_record(
                                        writer,
                                        &record,
                                        &mut head_buffer,
                                        &mut seq_buffer,
                                        &mut qual_buffer,
                                    );
                                } else if with_fasta {
                                    // With FASTA format
                                    write_record_with_fasta(
                                        writer,
                                        &record,
                                        &mut head_buffer,
                                        &mut seq_buffer,
                                    );
                                } else {
                                    // Default mode
                                    writer.write_all(record.seq()).unwrap();
                                    writer.write_all(b"\n").unwrap();
                                }
                            }
                        }
                    } else if with_id {
                        // With ID mode
                        write_record_with_id(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                        );
                    } else if with_full_record {
                        // With full record mode
                        write_full_record(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                            &mut qual_buffer,
                        );
                    } else if with_fasta {
                        // With FASTA format
                        write_record_with_fasta(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                        );
                    } else {
                        // Default mode
                        writer.write_all(record.seq()).unwrap();
                        writer.write_all(b"\n").unwrap();
                    }
                }
                None::<()>
            },
        )
        .unwrap();
    }
}

// Calculate average quality of a sequence
#[inline(always)]
fn average_quality(qual: &[u8], encoding: &str) -> f32 {
    quality::average_quality(qual, encoding)
}

// Write record with ID
#[inline(always)]
fn write_record_with_id<W: Write>(
    writer: &mut W,
    record: &seq_io::fastq::RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
) {
    head_buffer.clear();
    seq_buffer.clear();
    head_buffer.extend_from_slice(record.head());
    seq_buffer.extend_from_slice(record.seq());
    writer.write_all(b"@").unwrap();
    writer.write_all(head_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(seq_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
}

// Write full record
#[inline(always)]
fn write_full_record<W: Write>(
    writer: &mut W,
    record: &seq_io::fastq::RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
    qual_buffer: &mut Vec<u8>,
) {
    head_buffer.clear();
    seq_buffer.clear();
    qual_buffer.clear();
    head_buffer.extend_from_slice(record.head());
    seq_buffer.extend_from_slice(record.seq());
    qual_buffer.extend_from_slice(record.qual());
    writer.write_all(b"@").unwrap();
    writer.write_all(head_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(seq_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(b"+").unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(qual_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
}

// Write record in FASTA format
#[inline(always)]
fn write_record_with_fasta<W: Write>(
    writer: &mut W,
    record: &seq_io::fastq::RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
) {
    head_buffer.clear();
    seq_buffer.clear();
    head_buffer.extend_from_slice(record.head());
    seq_buffer.extend_from_slice(record.seq());
    writer.write_all(b">").unwrap();
    writer.write_all(head_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(seq_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
}
