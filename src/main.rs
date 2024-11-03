use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use clap::Parser;
use regex::bytes::RegexSet;
use seq_io::fastq::{Reader, Record};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[derive(Parser)]
#[command(
    name = "grepq",
    author = "Nicholas D. Crosbie",
    version = "1.0.1",
    about = "quickly filter fastq files by matching sequences to set of regex patterns",
    long_about = "Copyright (c) 2024 Nicholas D. Crosbie. Licensed under the MIT license.",
    after_help = "Notes:
    - Only supports ASCII-encoded fastq files.
    - When no options are provided, only the matching sequences are printed.
    - Count option (-c) is only supported for full fastq records (for example, the output of -R).
    - Patterns file must contain one regex pattern per line.
    - Inverted matches are not supported.
    - regex patterns with look-around and backreferences are not supported.
    "
)]
struct Cli {
    #[arg(short = 'I', help = "Include record ID in the output")]
    with_id: bool,

    #[arg(
        short = 'R',
        help = "Include record ID, sequence, separator, and quality in the output"
    )]
    with_full_record: bool,

    #[arg(short = 'c', help = "Count the number of matching fastq records")]
    count: bool,

    #[arg(help = "Path to the patterns file (one regex pattern per line)")]
    patterns: String,

    #[arg(help = "Path to the fastq file")]
    file: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let patterns_path = &cli.patterns;
    let file_path = &cli.file;
    let with_id = cli.with_id;
    let with_full_record = cli.with_full_record;
    let count = cli.count;

    let regex_set = {
        let file = File::open(patterns_path)?;
        let reader = BufReader::new(file);
        RegexSet::new(reader.lines().filter_map(Result::ok))
            .expect("Failed to compile regex patterns")
    };

    let file = File::open(file_path)?;
    let mut reader = Reader::with_capacity(file, 8 * 1024 * 1024);

    let stdout = io::stdout();
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, stdout.lock());

    if count {
        let mut match_count = 0;
        while let Some(result) = reader.next() {
            let record = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            if regex_set.is_match(record.seq()) {
                match_count += 1;
            }
        }
        writeln!(writer, "{}", match_count).unwrap();
    } else if with_id {
        while let Some(result) = reader.next() {
            let record = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            if regex_set.is_match(record.seq()) {
                writer.write_all(b"@").unwrap();
                writer.write_all(record.head()).unwrap();
                writer.write_all(b"\n").unwrap();
                writer.write_all(record.seq()).unwrap();
                writer.write_all(b"\n").unwrap();
            }
        }
    } else if with_full_record {
        while let Some(result) = reader.next() {
            let record = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            if regex_set.is_match(record.seq()) {
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
            let record = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            if regex_set.is_match(record.seq()) {
                writer.write_all(record.seq()).unwrap();
                writer.write_all(b"\n").unwrap();
            }
        }
    }

    Ok(())
}
