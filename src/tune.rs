use crate::arg::Cli;
use crate::initialise::{create_reader, parse_patterns_file};
use seq_io::fastq::Record;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

pub fn run_tune(cli: &Cli, num_records: usize, include_count: bool) -> io::Result<()> {
    let patterns_path = &cli.patterns;

    let (regex_set, header_regex, sequence_length, minimum_quality) =
        parse_patterns_file(patterns_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut reader = create_reader(cli);

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

    if patterns_path.ends_with(".json") {
        let json: serde_json::Value = serde_json::from_reader(std::fs::File::open(patterns_path)?)?;
        let regex_set_name = json["regexSet"]["regexSetName"]
            .as_str()
            .unwrap_or("Unknown");

        if cli.command.as_ref().map_or(
            false,
            |cmd| matches!(cmd, crate::arg::Commands::Tune(t) if t.include_names),
        ) {
            println!("Regex Set Name: {}", regex_set_name);
        }

        let regex_array = json["regexSet"]["regex"].as_array().unwrap();
        let mut regex_matches = vec![];

        for regex in regex_array {
            let regex_string = regex["regexString"].as_str().unwrap();
            let regex_name = regex["regexName"].as_str().unwrap_or("Unknown");
            let count = match_counts
                .iter()
                .find(|(pattern, _)| pattern == regex_string)
                .map(|(_, count)| count)
                .unwrap_or(&0);

            regex_matches.push(json!({
                "regexName": regex_name,
                "regexString": regex_string,
                "regexCount": count
            }));

            if cli.command.as_ref().map_or(
                false,
                |cmd| matches!(cmd, crate::arg::Commands::Tune(t) if t.include_names),
            ) {
                if include_count {
                    println!("{} ({}): {}", regex_name, regex_string, count);
                } else {
                    println!("{} ({})", regex_name, regex_string);
                }
            } else {
                if *count > 0 {
                    if include_count {
                        println!("{}: {}", regex_string, count);
                    } else {
                        println!("{}", regex_string);
                    }
                }
            }
        }

        if let Some(cmd) = &cli.command {
            if let crate::arg::Commands::Tune(tune) = cmd {
                if tune.json_matches && tune.include_names && include_count {
                    let json_output = json!({
                        "regexSet": {
                            "regexSetName": regex_set_name,
                            "regex": regex_matches
                        }
                    });
                    let file = File::create("matches.json")?;
                    serde_json::to_writer(file, &json_output)?;
                }
            }
        }
    } else {
        for (pattern, count) in &match_counts {
            if count > &0 {
                if include_count {
                    println!("{}: {}", pattern, count);
                } else {
                    println!("{}", pattern);
                }
            }
        }
    }

    Ok(())
}
