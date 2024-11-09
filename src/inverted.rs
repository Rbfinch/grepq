use crate::arg::Cli;
use regex::bytes::RegexSet;
use seq_io::fastq::{Reader, Record};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub fn run_inverted(cli: &Cli) -> io::Result<()> {
    let patterns_path = &cli.patterns;
    let file_path = &cli.file;
    let with_id = cli.with_id;
    let with_full_record = cli.with_full_record;
    let count = cli.count;

    let regex_set = {
        let file = File::open(patterns_path)?;
        let reader = BufReader::new(file);
        RegexSet::new(reader.lines().filter_map(Result::ok))
            .expect("Failed to compile regex patterns. Check your patterns file lists one regex pattern per line.")
    };

    let file = File::open(file_path)?;
    let mut reader = Reader::with_capacity(file, 8 * 1024 * 1024);

    let stdout = io::stdout();
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, stdout.lock());

    if count {
        let mut match_count = 0;
        while let Some(result) = reader.next() {
            let record = result.map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "grepq only supports the fastq format. Check your input file.: {}",
                        e
                    ),
                )
            })?;
            if !regex_set.is_match(record.seq()) {
                match_count += 1;
            }
        }
        writeln!(writer, "{}", match_count).unwrap();
    } else if with_id {
        while let Some(result) = reader.next() {
            let record = result.map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "grepq only supports the fastq format. Check your input file: {}",
                        e
                    ),
                )
            })?;
            if !regex_set.is_match(record.seq()) {
                writer.write_all(b"@").unwrap();
                writer.write_all(record.head()).unwrap();
                writer.write_all(b"\n").unwrap();
                writer.write_all(record.seq()).unwrap();
                writer.write_all(b"\n").unwrap();
            }
        }
    } else if with_full_record {
        while let Some(result) = reader.next() {
            let record = result.map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "grepq only supports the fastq format. Check your input file.: {}",
                        e
                    ),
                )
            })?;
            if !regex_set.is_match(record.seq()) {
                writer.write_all(b"@").unwrap();
                writer.write_all(record.head()).unwrap();
                writer.write_all(b"\n").unwrap();
                writer.write_all(record.seq()).unwrap();
                writer.write_all(b"\n").unwrap();
                writer.write_all(b"+").unwrap();
                writer.write_all(b"\n").unwrap();
                writer.write_all(record.qual()).unwrap();
                writer.write_all(b"\n").unwrap();
            }
        }
    } else {
        while let Some(result) = reader.next() {
            let record = result.map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "grepq only supports the fastq format. Check your input file.: {}",
                        e
                    ),
                )
            })?;
            if !regex_set.is_match(record.seq()) {
                writer.write_all(record.seq()).unwrap();
                writer.write_all(b"\n").unwrap();
            }
        }
    }

    Ok(())
}
