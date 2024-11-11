use crate::arg::Cli;
use regex::bytes::RegexSet;
use seq_io::fastq::{Reader, Record};
use seq_io::parallel::parallel_fastq;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub fn run_inverted(cli: &Cli) {
    let patterns_path = &cli.patterns;
    let file_path = &cli.file;
    let with_id = cli.with_id;
    let with_full_record = cli.with_full_record;
    let count = cli.count;

    let regex_set = {
        let file = File::open(patterns_path).unwrap();
        let reader = BufReader::new(file);
        RegexSet::new(reader.lines().filter_map(Result::ok))
            .expect("Failed to compile regex patterns. Check your patterns file lists one regex pattern per line.")
    };

    let file = File::open(file_path).unwrap();
    let reader = Reader::with_capacity(file, 8 * 1024 * 1024);

    let stdout = io::stdout();
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, stdout.lock());

    if count {
        let mut match_count = 0;
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get() as usize,
            |record, found| {
                // runs in worker
                *found = false;
                if !regex_set.is_match(record.seq()) {
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
                if !regex_set.is_match(record.seq()) {
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
