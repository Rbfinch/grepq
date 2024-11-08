use crate::arg::Cli;
use regex::bytes::RegexSet;
use seq_io::fastq::{Reader, Record};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run_tune(cli: &Cli, num_records: usize) -> io::Result<()> {
    let patterns_path = &cli.patterns;
    let file_path = &cli.file;

    let regex_set = {
        let file = File::open(patterns_path)?;
        let reader = BufReader::new(file);
        RegexSet::new(reader.lines().filter_map(Result::ok))
            .expect("Failed to compile regex patterns. Check your patterns file lists one regex pattern per line.")
    };

    let file = File::open(file_path)?;
    let mut reader = Reader::with_capacity(file, 8 * 1024 * 1024);

    let mut match_counts: HashMap<String, usize> = HashMap::new();
    let mut total_matches = 0;

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
        for mat in regex_set.matches(record.seq()).into_iter() {
            let matched_pattern = regex_set.patterns()[mat].to_string();
            *match_counts.entry(matched_pattern).or_insert(0) += 1;
            total_matches += 1;
            if total_matches >= num_records {
                break;
            }
        }
        if total_matches >= num_records {
            break;
        }
    }

    let mut match_counts: Vec<_> = match_counts.into_iter().collect();
    match_counts.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Matching patterns:");
    for (pattern, count) in &match_counts {
        println!("{}: {}", pattern, count);
    }

    println!("\nNon-matching patterns:");
    let matched_patterns: std::collections::HashSet<_> =
        match_counts.iter().map(|(p, _)| p).collect();
    for pattern in regex_set.patterns() {
        if !matched_patterns.contains(pattern) {
            println!("{}", pattern);
        }
    }

    Ok(())
}
