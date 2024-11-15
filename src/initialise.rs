use crate::arg::Cli;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder as MultiGzEncoder;
pub(crate) use flate2::Compression;
use regex::bytes::RegexSet;
use seq_io::fastq::Reader;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn create_regex_set(patterns_path: &str) -> RegexSet {
    let file = File::open(patterns_path).unwrap();
    let reader = BufReader::new(file);
    RegexSet::new(reader.lines().filter_map(Result::ok))
        .expect("Failed to compile regex patterns. Check your patterns file lists one regex pattern per line.")
}

pub fn open_file(file_path: &str) -> File {
    File::open(file_path).expect("Failed to open file")
}

pub fn create_reader(cli: &Cli) -> Reader<Box<dyn BufRead + Send>> {
    let file = open_file(&cli.file);
    let reader: Box<dyn BufRead + Send> = if cli.gzip_input {
        Box::new(BufReader::new(MultiGzDecoder::new(file))) as Box<dyn BufRead + Send>
    } else {
        Box::new(BufReader::new(file)) as Box<dyn BufRead + Send>
    };
    Reader::with_capacity(reader, 8 * 1024 * 1024)
}

pub fn create_writer(cli: &Cli) -> Box<dyn Write> {
    let stdout_lock = io::stdout().lock();
    let compression = if cli.fast_compression {
        Compression::fast()
    } else if cli.best_compression {
        Compression::best()
    } else {
        Compression::default()
    };
    if cli.gzip_output {
        Box::new(MultiGzEncoder::new(stdout_lock, compression))
    } else {
        Box::new(stdout_lock)
    }
}
