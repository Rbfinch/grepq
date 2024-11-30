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

    let (regex_set, header_regex, minimum_sequence_length, minimum_quality, quality_encoding) =
        parse_patterns_file(&cli.patterns)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            .unwrap();
    let header_regex = header_regex.map(|re| Regex::new(&re).unwrap());
    let reader = create_reader(&cli);
    let mut writer = create_writer(&cli);

    let with_id = cli.with_id;
    let with_full_record = cli.with_full_record;
    let count = cli.count;

    // Example usage of convert_and_print function
    quality::convert_and_print(quality::QUALITY_STRING);

    if count {
        let mut match_count = 0;
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get() as usize,
            |record, found| {
                // runs in worker
                *found = false;
                if minimum_sequence_length.map_or(true, |len| record.seq().len() >= len as usize)
                    && minimum_quality.map_or(true, |min_q| {
                        quality::average_quality(
                            record.qual(),
                            quality_encoding.as_deref().unwrap_or("Phred+33"),
                        ) >= min_q as f32
                    })
                    && header_regex
                        .as_ref()
                        .map_or(true, |re| re.is_match(record.head()))
                    && regex_set.is_match(record.seq())
                {
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
            num_cpus::get() as usize,
            |record, found| {
                // runs in worker
                *found = false;
                if minimum_sequence_length.map_or(true, |len| record.seq().len() >= len as usize)
                    && minimum_quality.map_or(true, |min_q| {
                        quality::average_quality(
                            record.qual(),
                            quality_encoding.as_deref().unwrap_or("Phred+33"),
                        ) >= min_q as f32
                    })
                    && header_regex
                        .as_ref()
                        .map_or(true, |re| re.is_match(record.head()))
                    && regex_set.is_match(record.seq())
                {
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
