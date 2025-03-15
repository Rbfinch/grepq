use crate::arg::Cli;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder as MultiGzEncoder;
pub(crate) use flate2::Compression;
use regex::bytes::RegexSet;
use seq_io::fastq::Reader;
use serde_json::Value;
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, BufRead, BufReader, Write};
use zstd::stream::{read::Decoder as ZstdDecoder, write::Encoder as ZstdEncoder};

// Static JSON schema used to validate the input patterns file.
static SCHEMA: &str = r#"
{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "grepq",
    "version": 2,
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
                            },
                            "variants": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "variantName": {
                                            "type": "string"
                                        },
                                        "variantString": {
                                            "type": "string"
                                        }
                                    },
                                    "required": [
                                        "variantName",
                                        "variantString"
                                    ]
                                }
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
                "minimumAverageQuality": {
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
        Option<f32>,
        Option<String>,
        Vec<String>,           // regex_names
        Vec<(String, String)>, // variants
    ),
    String,
>;

// Function: convert_iupac_to_regex
// Converts IUPAC nucleotide codes to a regex pattern by replacing ambiguous characters.
// It panics if an illegal character is encountered.
pub fn convert_iupac_to_regex(pattern: &str) -> String {
    let legal = "ACGTYRWSKMBDHVN";

    // Check if the pattern *could* be an IUPAC pattern (all chars are alphabetic).
    let could_be_iupac = pattern.chars().all(|c| c.is_alphabetic());

    if could_be_iupac {
        // If it *could* be IUPAC, check for illegal characters and panic if found.
        for c in pattern.chars() {
            if c.is_alphabetic() && !legal.contains(c.to_ascii_uppercase()) {
                panic!("Illegal character found in pattern: {}", c);
            }
        }

        // If it's a valid IUPAC pattern, convert it to uppercase and replace characters.
        let uppercase = pattern.to_uppercase();
        uppercase
            .replace('Y', "[CT]")
            .replace('R', "[AG]")
            .replace('W', "[AT]")
            .replace('S', "[CG]")
            .replace('K', "[GT]")
            .replace('M', "[AC]")
            .replace('B', "[CGT]")
            .replace('D', "[AGT]")
            .replace('H', "[ACT]")
            .replace('V', "[ACG]")
            .replace('N', "[ACGT]")
    } else {
        // If it's not IUPAC, return the original pattern unchanged.
        pattern.to_string()
    }
}

// Function: validate_dna_sequence
// Validates that a given sequence contains only valid DNA nucleotides.
fn validate_dna_sequence(sequence: &str) -> Result<(), String> {
    if sequence.chars().all(|c| "ACTG".contains(c)) {
        Ok(())
    } else {
        Err(format!("Invalid DNA sequence: {}", sequence))
    }
}

// Function: parse_patterns_file
// Parses a patterns file which may be JSON or plain text, validates it against a schema,
// and compiles regex patterns. It also extracts optional settings.
pub fn parse_patterns_file(patterns_path: &str) -> ParseResult {
    if patterns_path.ends_with(".json") {
        // Open and parse the JSON file.
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

        // Convert patterns using IUPAC-to-regex conversion.
        let regex_strings: Vec<String> = json["regexSet"]["regex"]
            .as_array()
            .ok_or("Invalid JSON structure")?
            .iter()
            .filter_map(|r| {
                r.get("regexString")
                    .and_then(|s| s.as_str())
                    .map(convert_iupac_to_regex)
            })
            .collect();

        let regex_set = RegexSet::new(&regex_strings)
            .map_err(|e| format!("Failed to compile regex patterns: {}", e))?;
        let header_regex = json["regexSet"]["headerRegex"]
            .as_str()
            .map(|s| s.to_string());
        let minimum_sequence_length = json["regexSet"]["minimumSequenceLength"].as_u64();
        let minimum_quality = json["regexSet"]["minimumAverageQuality"]
            .as_f64()
            .map(|q| q as f32);
        let quality_encoding = json["regexSet"]["qualityEncoding"]
            .as_str()
            .map(|s| s.to_string());

        // Process regex variants with additional DNA validation.
        let variants: Vec<_> = json["regexSet"]["regex"]
            .as_array()
            .ok_or("Invalid JSON structure")?
            .iter()
            .filter_map(|r| r.get("variants"))
            .flat_map(|v| v.as_array().unwrap_or(&Vec::new()).clone())
            .map(|variant| -> Result<_, String> {
                let variant_name = variant["variantName"]
                    .as_str()
                    .ok_or("Invalid variantName")?
                    .to_string();
                let variant_string = variant["variantString"]
                    .as_str()
                    .ok_or("Invalid variantString")?
                    .to_string();
                validate_dna_sequence(&variant_string)?;
                Ok((variant_name, variant_string))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let regex_names = json["regexSet"]["regex"]
            .as_array()
            .ok_or("Invalid JSON structure")?
            .iter()
            .map(|r| {
                r.get("regexName")
                    .and_then(|s| s.as_str())
                    .unwrap_or_else(|| r.get("regexString").and_then(|s| s.as_str()).unwrap())
                    .to_string()
            })
            .collect::<Vec<_>>();

        Ok((
            regex_set,
            header_regex,
            minimum_sequence_length,
            minimum_quality,
            quality_encoding,
            regex_names, // Regex pattern names.
            variants,    // Regex variants.
        ))
    } else {
        // Process plain-text pattern files.
        let file = File::open(patterns_path)
            .map_err(|e| format!("Failed to open patterns file: {}", e))?;
        let reader = BufReader::new(file);
        let lines: Result<Vec<_>, _> = reader.lines().collect();
        let regex_strings: Vec<String> = lines
            .map_err(|e| format!("Failed to read lines: {}", e))?
            .iter()
            .map(|line| convert_iupac_to_regex(line))
            .collect();
        let regex_set = RegexSet::new(&regex_strings)
            .map_err(|e| format!("Failed to compile regex patterns: {}", e))?;
        let regex_names = regex_strings.clone(); // Use regex strings as names.
        Ok((regex_set, None, None, None, None, regex_names, Vec::new()))
    }
}

// Function: open_file
// Opens the given file path and returns a File handle.
pub fn open_file(file_path: &str) -> File {
    File::open(file_path).expect("Failed to open file")
}

// Function: create_reader
// Creates a buffered reader for the input file, handling compression based on CLI flags.
pub fn create_reader(cli: &Cli) -> Reader<Box<dyn BufRead + Send>> {
    let file = open_file(&cli.file);
    let reader: Box<dyn BufRead + Send> = if cli.gzip_input {
        // Use Gzip decompression.
        Box::new(BufReader::new(MultiGzDecoder::new(file)))
    } else if cli.zstd_input {
        // Use Zstd decompression.
        match ZstdDecoder::new(file) {
            Ok(decoder) => Box::new(BufReader::new(decoder)),
            Err(e) => {
                eprintln!("Error: Failed to read zstd compressed file. The file may be corrupted or incomplete.");
                eprintln!("Underlying error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        Box::new(BufReader::new(file))
    };
    Reader::with_capacity(reader, 8 * 1024 * 1024)
}

// Struct ZstdWriter
// A wrapper for writing Zstd compressed data.
struct ZstdWriter<W: Write> {
    encoder: Option<ZstdEncoder<'static, W>>,
}

impl<W: Write> ZstdWriter<W> {
    // Create a new ZstdWriter with specified compression level.
    fn new(writer: W, compression_level: i32) -> io::Result<Self> {
        let mut encoder = ZstdEncoder::new(writer, compression_level)?;
        encoder.include_checksum(true)?;
        Ok(Self {
            encoder: Some(encoder),
        })
    }
}

impl<W: Write> Write for ZstdWriter<W> {
    // Write data to the underlying Zstd encoder.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(encoder) = &mut self.encoder {
            encoder.write(buf)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Encoder has been finalized",
            ))
        }
    }

    // Flush the encoder.
    fn flush(&mut self) -> io::Result<()> {
        if let Some(encoder) = &mut self.encoder {
            encoder.flush()
        } else {
            Ok(())
        }
    }
}

impl<W: Write> Drop for ZstdWriter<W> {
    // Finalize the encoder when ZstdWriter is dropped.
    fn drop(&mut self) {
        if let Some(encoder) = self.encoder.take() {
            let _ = encoder.finish();
        }
    }
}

// Function: create_writer
// Creates a writer for the output file, handling various compression or formatting options based on CLI flags.
pub fn create_writer(cli: &Cli) -> Box<dyn Write> {
    let stdout_lock = io::stdout().lock();

    if cli.gzip_output {
        // Write output using Gzip compression.
        let compression = if cli.fast_compression {
            Compression::fast()
        } else if cli.best_compression {
            Compression::best()
        } else {
            Compression::default()
        };
        Box::new(MultiGzEncoder::new(stdout_lock, compression))
    } else if cli.zstd_output {
        // Write output using Zstd compression.
        let level = if cli.fast_compression {
            1
        } else if cli.best_compression {
            21
        } else {
            3
        };
        Box::new(ZstdWriter::new(stdout_lock, level).unwrap())
    } else if cli.with_fasta {
        // Write output in FASTA format.
        Box::new(BufWriter::new(stdout_lock))
    } else {
        // Default to writing plain output.
        Box::new(stdout_lock)
    }
}
