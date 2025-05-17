// MIT License

// Copyright (c) 2024 - present Nicholas D. Crosbie

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Main functionality for the inverted command.
// This command filters FASTQ records using a variety of criteria (sequence length, quality, header, regex)
// and either counts matching records or outputs them in one of several formats.
use crate::arg::Cli;
use crate::initialise::{create_reader, create_writer, parse_patterns_file};
use crate::output::{write_full_record, write_record_with_fasta, write_record_with_id};
use regex::bytes::Regex;
use seq_io::fastq::Record;
use seq_io::parallel::parallel_fastq;
use std::io::Write;

// Main function to run the inverted command
pub fn run_inverted(cli: &Cli) {
    // Extract CLI options for output mode.
    let with_id = cli.with_id;
    let with_full_record = cli.with_full_record;
    let with_fasta = cli.with_fasta;
    let count = cli.count;

    // Parse the patterns file to extract regex patterns and optional filter parameters.
    let (regex_set, header_regex, minimum_sequence_length, minimum_quality, quality_encoding, _, _) =
        parse_patterns_file(&cli.patterns)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            .unwrap();

    // Compile the optional header regex.
    let header_regex = header_regex.map(|re| {
        // Compile header regex filter.
        Regex::new(&re).unwrap()
    });

    // Create input reader and output writer based on CLI flags.
    let reader = create_reader(cli);
    let mut writer = create_writer(cli);

    // Determine which filters to apply.
    let check_seq_len = minimum_sequence_length.is_some();
    let check_qual = minimum_quality.is_some();
    let check_header = header_regex.is_some();

    // Initialize buffers to reuse memory.
    let mut seq_buffer = Vec::new();
    let mut qual_buffer = Vec::new();
    let mut head_buffer = Vec::new();

    if count {
        // Count mode: Only count records that match the filter criteria.
        let mut match_count = 0;
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // Worker thread: Apply filters on each record.
                *found = false;
                let seq_len_check = !check_seq_len
                    || record.seq().len() >= minimum_sequence_length.unwrap() as usize;
                let qual_check = !check_qual
                    || crate::quality::average_quality(
                        record.qual(),
                        quality_encoding.as_deref().unwrap_or("Phred+33"),
                    ) >= minimum_quality.unwrap();
                let header_check =
                    !check_header || header_regex.as_ref().unwrap().is_match(record.head());
                let regex_check = !regex_set.is_match(record.seq());

                // Mark record as matching if all conditions are met.
                if seq_len_check && qual_check && header_check && regex_check {
                    *found = true;
                }
            },
            |_, found| {
                // Main thread: Increment count based on the worker's flag.
                if *found {
                    match_count += 1;
                }
                None::<()>
            },
        )
        .unwrap();
        // Output the count.
        writeln!(writer, "{}", match_count).unwrap();
    } else {
        // Record output mode: Write records based on the selected output format.
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // Worker thread: Check filter criteria.
                *found = false;
                let seq_len_check = !check_seq_len
                    || record.seq().len() >= minimum_sequence_length.unwrap() as usize;
                let qual_check = !check_qual
                    || crate::quality::average_quality(
                        record.qual(),
                        quality_encoding.as_deref().unwrap_or("Phred+33"),
                    ) >= minimum_quality.unwrap();
                let header_check =
                    !check_header || header_regex.as_ref().unwrap().is_match(record.head());
                let regex_check = !regex_set.is_match(record.seq());

                if seq_len_check && qual_check && header_check && regex_check {
                    *found = true;
                }
            },
            |record, found| {
                // Main thread: Write the record in the appropriate format if it passed the filters.
                if *found {
                    if with_id {
                        // Output only the record ID.
                        write_record_with_id(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                        );
                    } else if with_full_record {
                        // Output the full FASTQ record.
                        write_full_record(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                            &mut qual_buffer,
                        );
                    } else if with_fasta {
                        // Output in FASTA format.
                        write_record_with_fasta(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                        );
                    } else {
                        // Default: output the raw sequence followed by a newline.
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
