use crate::arg::Cli;
use crate::initialise::{create_reader, create_writer, parse_patterns_file};
use crate::quality;
use regex::Regex;
use seq_io::fastq::Record;
use seq_io::parallel::parallel_fastq;
use std::io::Write;
use std::io::{self};

use crate::debug_log;

pub fn run_inverted(cli: &Cli) {
    // No need to initialize env_logger here
    let with_id = cli.with_id;
    let count = cli.count;
    let with_full_record = cli.with_full_record;
    let (regex_set, header_regex, minimum_sequence_length, minimum_quality, quality_encoding) =
        parse_patterns_file(&cli.patterns)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            .unwrap();
    let header_regex = header_regex.map(|re| Regex::new(&re).unwrap());

    let reader = create_reader(cli);
    let mut writer = create_writer(cli);

    if count {
        let mut match_count = 0;
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // runs in worker
                *found = false;
                let seq_len_check =
                    minimum_sequence_length.map_or(true, |len| record.seq().len() >= len as usize);
                let qual_check = minimum_quality.map_or(true, |min_q| {
                    quality::average_quality(
                        record.qual(),
                        quality_encoding.as_deref().unwrap_or("Phred+33"),
                    ) >= min_q as f32
                });
                let header_check = header_regex
                    .as_ref()
                    .map_or(true, |re| re.is_match(std::str::from_utf8(record.head()).unwrap()));
                let regex_check = !regex_set.is_match(record.seq());

                debug_log!("Debug: seq_len_check = {}, qual_check = {}, header_check = {}, regex_check = {}", seq_len_check, qual_check, header_check, regex_check);

                if seq_len_check && qual_check && header_check && !regex_check {
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
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // runs in worker
                *found = false;
                let seq_len_check =
                    minimum_sequence_length.map_or(true, |len| record.seq().len() >= len as usize);
                let qual_check = minimum_quality.map_or(true, |min_q| {
                    quality::average_quality(
                        record.qual(),
                        quality_encoding.as_deref().unwrap_or("Phred+33"),
                    ) >= min_q as f32
                });
                let header_check = header_regex
                    .as_ref()
                    .map_or(true, |re| re.is_match(std::str::from_utf8(record.head()).unwrap()));
                let regex_check = !regex_set.is_match(record.seq());

                debug_log!("Debug: seq_len_check = {}, qual_check = {}, header_check = {}, regex_check = {}", seq_len_check, qual_check, header_check, regex_check);

                if seq_len_check && qual_check && header_check && !regex_check {
                    *found = true;
                }
            },
            |record, found| {
                // runs in main thread
                if *found {
                    if with_id {
                        // With ID mode
                        writer.write_all(b"@").unwrap();
                        writer.write_all(record.head()).unwrap();
                        writer.write_all(b"\n").unwrap();
                        writer.write_all(record.seq()).unwrap();
                        writer.write_all(b"\n").unwrap();
                    } else if with_full_record {
                        // With full record mode
                        writer.write_all(b"@").unwrap();
                        writer.write_all(record.head()).unwrap();
                        writer.write_all(b"\n").unwrap();
                        writer.write_all(record.seq()).unwrap();
                        writer.write_all(b"\n").unwrap();
                        writer.write_all(b"+").unwrap();
                        writer.write_all(b"\n").unwrap();
                        writer.write_all(record.qual()).unwrap();
                        writer.write_all(b"\n").unwrap();
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
