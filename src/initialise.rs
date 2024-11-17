use crate::arg::Cli;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder as MultiGzEncoder;
pub(crate) use flate2::Compression;
// use jsonschema::Draft;
use regex::bytes::RegexSet;
use seq_io::fastq::Reader;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn create_regex_set_from_json(json_path: &str, schema_path: &str) -> Result<RegexSet, String> {
    let json_file =
        File::open(json_path).map_err(|e| format!("Failed to open JSON file: {}", e))?;
    let schema_file =
        File::open(schema_path).map_err(|e| format!("Failed to open schema file: {}", e))?;

    let json: Value = serde_json::from_reader(json_file)
        .map_err(|e| format!("Failed to parse JSON file: {}", e))?;
    let schema: Value = serde_json::from_reader(schema_file)
        .map_err(|e| format!("Failed to parse schema file: {}", e))?;

    let validator = jsonschema::validator_for(&schema)
        .map_err(|e| format!("Failed to compile schema: {}", e))?;

    let mut error_messages = Vec::new();
    for error in validator.iter_errors(&json) {
        error_messages.push(format!("Error: {error}\nLocation: {}", error.instance_path));
    }

    if !error_messages.is_empty() {
        return Err(format!("JSON validation errors: {:?}", error_messages));
    }

    let regex_strings: Vec<String> = json["regexSet"]["regex"]
        .as_array()
        .ok_or("Invalid JSON structure")?
        .iter()
        .filter_map(|r| {
            r.get("regexString")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    RegexSet::new(regex_strings).map_err(|e| format!("Failed to compile regex patterns: {}", e))
}

pub fn create_regex_set(patterns_path: &str, cli: &Cli) -> RegexSet {
    if cli.json_input {
        match create_regex_set_from_json(patterns_path, "schema.json") {
            Ok(regex_set) => regex_set,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    } else {
        let file = File::open(patterns_path).unwrap();
        let reader = BufReader::new(file);
        RegexSet::new(reader.lines().filter_map(Result::ok))
            .expect("Failed to compile regex patterns. Check your patterns file lists one regex pattern per line.")
    }
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
