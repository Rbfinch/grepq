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
    version = "1.0.7",
    about = "quickly filter fastq files by matching sequences to a set of regex patterns",
    long_about = "Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License.",
    after_help = "
       Examples:
             - Print only the matching sequences:
                  grepq regex.txt file.fastq > outfile.txt
        
             - Print the matching sequences with the record ID:
                  grepq -I regex.txt file.fastq > outfile.txt
        
             - Print the matching sequences with the record ID, sequence, separator, and quality fields
                  grepq -R regex.txt file.fastq > outfile.txt
        
             - Count the number of matching fastq records:
                  grepq -c regex.txt file.fastq

           Tips:
             - Order your regex patterns from those that are most likely to match to those that
               are least likely to match. This will speed up the filtering process.

             - Ensure you have enough storage space for the output file.

          Notes:
             - Only supports fastq files.

             - Patterns file must contain one regex pattern per line.

             - When no options are provided, only the matching sequences are printed.

             - Only one of the -I, -R, or -c options can be used at a time.

             - Count option (-c) will support the output of the -R option since it is in fastq format.

             - Inverted matches are not supported.

             - Regex patterns with look-around and backreferences are not supported.

Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License."
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
            if regex_set.is_match(record.seq()) {
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
            let record = result.map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "grepq only supports the fastq format. Check your input file.: {}",
                        e
                    ),
                )
            })?;
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
            let record = result.map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "grepq only supports the fastq format. Check your input file.: {}",
                        e
                    ),
                )
            })?;
            if regex_set.is_match(record.seq()) {
                writer.write_all(record.seq()).unwrap();
                writer.write_all(b"\n").unwrap();
            }
        }
    }

    Ok(())
}
