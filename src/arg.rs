use clap::Parser;

#[derive(Parser)]
#[command(
    name = "grepq",
    author = "Nicholas D. Crosbie",
    version = clap::crate_version!(),
    about = "quickly filter FASTQ files by matching sequences to a set of regex patterns",
    long_about = "Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License.",
    after_help = "
       Examples:
             - Print only the matching sequences:
                  `grepq regex.txt file.fastq > output.txt`

             - Print the matching sequences with the record ID:
                  `grepq -I regex.txt file.fastq > output.txt`

             - Print the matching sequences in FASTQ format
                  `grepq -R regex.txt file.fastq > output.fastq`

             - Print the matching sequences in gzip compressed FASTQ format:
                  `grepq -R -z regex.txt file.fastq > output.fastq.gz`

             - Read the FASTQ file in gzip compressed format:
                  `grepq -x regex.txt file.fastq.gz > output.txt`

             - Read and write the output in gzip compressed format, with fast compression:
                  `grepq -xz --fast regex.txt file.fastq.gz > output.fastq.gz`

             - Read and write the output in gzip compressed format, with best compression:
                  `grepq -xz --best regex.txt file.fastq.gz > output.fastq.gz`

             - Count the number of matching FASTQ records:
                  `grepq -c regex.txt file.fastq`

             - For each matched pattern in a search of the first 100000 records, print the pattern and the number of matches:
                  `grepq regex.txt file.fastq tune -n 100000 -c`

             - For each matched pattern in a search of the first 100000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches:
                    `grepq -x regex.txt file.fastq.gz tune -n 100000 -c`

             - Print the records where none of the regex patterns are found:
                  `grepq regex.txt file.fastq inverted > output.txt`

             - Print the records where none of the regex patterns are found with the record ID:
                  `grepq -I regex.txt file.fastq inverted > output.txt`

             - Print the records where none of the regex patterns are found in FASTQ format:
                  `grepq -R regex.txt file.fastq inverted > output.fastq`

            - Count the number of records where none of the regex patterns are found:
                  `grepq -c regex.txt file.fastq inverted`

            - Count the total number of records in the FASTQ file using an empty pattern file:
                  `grepq -c empty.txt file.fastq inverted` 

           Tips:

             - Use the `tune` subcommand (`grepq tune -h` for instructions) to analyze matched substrings and update the number and/or order of regex patterns in your pattern file according to their matched frequency. This can speed up the filtering process.

             - Use the `inverted` subcommand to identify records that do not match any of the regex patterns in your pattern file.

             - Ensure you have enough storage space for output files.

          Notes:
             - Only supports FASTQ files or gzip compressed FASTQ files.

             - Patterns file must contain one regex pattern per line, and patterns are case-sensitive (you can supply an empty pattern file to count the total number of records in the FASTQ file).

             - When no options are provided, only the matching sequences are printed.

             - Only one of the -I, -R, or -c options can be used at a time.

             - The -x and -z options can be used separately, or together, and in combination any of the other filtering options (the -z option cannot be used with the tune subcommand).

             - Count option (-c) will support the output of the -R option since it is in FASTQ format.

             - Regex patterns with look-around and backreferences are not supported.

Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License."
)]
pub struct Cli {
    #[arg(short = 'I', help = "Include record ID in the output")]
    pub with_id: bool,

    #[arg(
        short = 'R',
        help = "Include record ID, sequence, separator, and quality in the output"
    )]
    pub with_full_record: bool,

    #[arg(short = 'c', help = "Count the number of matching FASTQ records")]
    pub count: bool,

    #[arg(short = 'x', help = "Read the FASTQ file in gzip compressed format")]
    pub gzip_input: bool,

    #[arg(short = 'z', help = "Write the output in gzip compressed format")]
    pub gzip_output: bool,

    #[arg(long = "fast", help = "Use fast compression")]
    pub fast_compression: bool,

    #[arg(long = "best", help = "Use best compression")]
    pub best_compression: bool,

    #[arg(short = 'j', help = "Read the patterns file in JSON format")]
    pub json_input: bool,

    #[arg(help = "Path to the patterns file (one regex pattern per line)")]
    pub patterns: String,

    #[arg(help = "Path to the FASTQ file")]
    pub file: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser)]
pub enum Commands {
    #[command(about = "Tune the regex patterns by analyzing matched substrings")]
    Tune(Tune),
    #[command(about = "Print records where none of the regex patterns are found")]
    Inverted,
}

#[derive(Parser)]
pub struct Tune {
    #[arg(help = "Number of matched records", short = 'n')]
    pub num_records: usize,

    #[arg(short = 'c', help = "Include count of records for matching patterns")]
    pub include_count: bool,

    #[arg(
        long = "names",
        help = "Include regexSetName and regexName in the output"
    )]
    pub include_names: bool,

    #[arg(
        long = "json-matches",
        help = "Write the output to a JSON file called matches.json"
    )]
    pub json_matches: bool,
}
