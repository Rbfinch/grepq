use crate::arg::Cli;
use crate::initialise::{create_reader, create_writer, parse_patterns_file};
use crate::output::{write_full_record, write_record_with_fasta, write_record_with_id};
use regex::bytes::Regex;
use seq_io::fastq::Record;
use seq_io::parallel::parallel_fastq;
use std::io::Write;

// Main function to run the inverted command
pub fn run_inverted(cli: &Cli) {
    let with_id = cli.with_id;
    let with_full_record = cli.with_full_record;
    let with_fasta = cli.with_fasta;
    let count = cli.count;

    // Parse the patterns file
    let (regex_set, header_regex, minimum_sequence_length, minimum_quality, quality_encoding, _, _) =
        parse_patterns_file(&cli.patterns)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            .unwrap();
    let header_regex = header_regex.map(|re| Regex::new(&re).unwrap());
    let reader = create_reader(cli);
    let mut writer = create_writer(cli);

    let check_seq_len = minimum_sequence_length.is_some();
    let check_qual = minimum_quality.is_some();
    let check_header = header_regex.is_some();

    let mut seq_buffer = Vec::new();
    let mut qual_buffer = Vec::new();
    let mut head_buffer = Vec::new();

    if count {
        let mut match_count = 0;
        parallel_fastq(
            reader,
            num_cpus::get() as u32,
            num_cpus::get(),
            |record, found| {
                // runs in worker
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
            num_cpus::get(),
            |record, found| {
                // runs in worker
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
                // runs in main thread
                if *found {
                    if with_id {
                        // With ID mode
                        write_record_with_id(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                        );
                    } else if with_full_record {
                        // With full record mode
                        write_full_record(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                            &mut qual_buffer,
                        );
                    } else if with_fasta {
                        // With FASTA mode
                        write_record_with_fasta(
                            &mut writer,
                            &record,
                            &mut head_buffer,
                            &mut seq_buffer,
                        );
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
