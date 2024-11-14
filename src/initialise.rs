use crate::arg::Cli;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use regex::bytes::RegexSet;
use seq_io::fastq::Reader;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub fn create_regex_set(patterns_path: &str) -> RegexSet {
    let file = File::open(patterns_path).unwrap();
    let reader = BufReader::new(file);
    RegexSet::new(reader.lines().filter_map(Result::ok))
        .expect("Failed to compile regex patterns. Check your patterns file lists one regex pattern per line.")
}

pub fn create_reader(cli: &Cli) -> Reader<Box<dyn BufRead + Send>> {
    let file_path = &cli.file;
    let file = if cli.gzip_input {
        let file = File::open(file_path).unwrap();
        Box::new(BufReader::new(GzDecoder::new(file))) as Box<dyn BufRead + Send>
    } else {
        let file = File::open(file_path).unwrap();
        Box::new(BufReader::new(file)) as Box<dyn BufRead + Send>
    };
    Reader::with_capacity(file, 8 * 1024 * 1024)
}

pub fn create_writer(cli: &Cli) -> BufWriter<Box<dyn Write>> {
    let stdout = io::stdout();
    let writer: Box<dyn Write> = if cli.gzip_output {
        Box::new(GzEncoder::new(stdout.lock(), Compression::default()))
    } else {
        Box::new(BufWriter::with_capacity(8 * 1024 * 1024, stdout.lock()))
    };
    BufWriter::with_capacity(8 * 1024 * 1024, writer)
}
