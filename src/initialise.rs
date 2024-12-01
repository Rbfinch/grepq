use crate::arg::Cli;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder as MultiGzEncoder;
pub(crate) use flate2::Compression;
use regex::bytes::RegexSet;
use seq_io::fastq::Reader;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

static SCHEMA: &str = r#"
{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "grepq",
    "version": 1,
    "type": "object",
    "properties": {
        "regexSet": {
            "type": "object",
            "properties": {
                "regexSetName": {
                    "type": "string"
                },
                "regex": {
                    "type": "array",
                    "minItems": 1,
                    "items": {
                        "type": "object",
                        "properties": {
                            "regexName": {
                                "type": "string"
                            },
                            "regexString": {
                                "type": "string"
                            }
                        },
                        "required": [
                            "regexName",
                            "regexString"
                        ]
                    }
                },
                "headerRegex": {
                    "type": "string"
                },
                "minimumSequenceLength": {
                    "type": "number"
                },
                "minimumQuality": {
                    "type": "number"
                },
                "qualityEncoding": {
                    "type": "string"
                }
            },
            "required": [
                "regexSetName",
                "regex"
            ]
        }
    },
    "required": [
        "regexSet"
    ]
}
"#;

type ParseResult = Result<
    (
        RegexSet,
        Option<String>,
        Option<u64>,
        Option<f64>,
        Option<String>,
    ),
    String,
>;

pub fn parse_patterns_file(patterns_path: &str) -> ParseResult {
    if patterns_path.ends_with(".json") {
        let json_file =
            File::open(patterns_path).map_err(|e| format!("Failed to open JSON file: {}", e))?;
        let json: Value = serde_json::from_reader(json_file)
            .map_err(|e| format!("Failed to parse JSON file: {}", e))?;
        let schema: Value = serde_json::from_str(SCHEMA)
            .map_err(|e| format!("Failed to parse embedded schema: {}", e))?;

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

        let regex_set = RegexSet::new(regex_strings)
            .map_err(|e| format!("Failed to compile regex patterns: {}", e))?;
        let header_regex = json["regexSet"]["headerRegex"]
            .as_str()
            .map(|s| s.to_string());
        let minimum_sequence_length = json["regexSet"]["minimumSequenceLength"].as_u64();
        let minimum_quality = json["regexSet"]["minimumQuality"].as_f64();
        let quality_encoding = json["regexSet"]["qualityEncoding"]
            .as_str()
            .map(|s| s.to_string());

        Ok((
            regex_set,
            header_regex,
            minimum_sequence_length,
            minimum_quality,
            quality_encoding,
        ))
    } else {
        let file = File::open(patterns_path)
            .map_err(|e| format!("Failed to open patterns file: {}", e))?;
        let reader = BufReader::new(file);
        let lines: Result<Vec<_>, _> = reader.lines().collect();
        let regex_set = RegexSet::new(lines.map_err(|e| format!("Failed to read lines: {}", e))?)
            .map_err(|e| format!("Failed to compile regex patterns: {}", e))?;
        Ok((regex_set, None, None, None, None))
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
