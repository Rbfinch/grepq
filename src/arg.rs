use clap::Parser;

#[derive(Parser)]
#[command(
    name = "grepq",
    author = "Nicholas D. Crosbie",
    version = clap::crate_version!(),
    about = "quickly filter FASTQ files by matching sequences to a set of regex patterns",
    long_about = "Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License.",
    after_help = "
       EXAMPLES:
             - Print only the matching sequences
                  `grepq regex.txt file.fastq > output.txt`

             - Print the matching sequences with the record ID
                  `grepq -I regex.txt file.fastq > output.txt`

             - Print the matching sequences in FASTQ format
                  `grepq -R regex.txt file.fastq > output.fastq`

             - Print the matching sequences in gzip compressed FASTQ format
                  `grepq -R -z regex.txt file.fastq > output.fastq.gz`

             - Read the FASTQ file in gzip compressed format
                  `grepq -x regex.txt file.fastq.gz > output.txt`

             - Read and write the output in gzip compressed format, with fast compression
                  `grepq -xz --fast regex.txt file.fastq.gz > output.fastq.gz`

             - Read and write the output in gzip compressed format, with best compression
                  `grepq -xz --best regex.txt file.fastq.gz > output.fastq.gz`

             - Count the number of matching FASTQ records
                  `grepq -c regex.txt file.fastq`

             - For each matched pattern in a search of the first 100000 records, print the pattern and the number of matches
                  `grepq regex.txt file.fastq tune -n 100000 -c`

             - For each matched pattern in a search of the first 100000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches
                    `grepq -x regex.txt file.fastq.gz tune -n 100000 -c`

             - For each matched pattern in a search of the first 100000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON file called matches.json 
                  `grepq -xj regex.json file.fastq.gz tune -n 100000 -c --names --json-matches`

             - Print the records where none of the regex patterns are found
                  `grepq regex.txt file.fastq inverted > output.txt`

             - Print the records where none of the regex patterns are found with the record ID
                  `grepq -I regex.txt file.fastq inverted > output.txt`

             - Print the records where none of the regex patterns are found in FASTQ format
                  `grepq -R regex.txt file.fastq inverted > output.fastq`

            - Count the number of records where none of the regex patterns are found
                  `grepq -c regex.txt file.fastq inverted`

            - Count the total number of records in the FASTQ file using an empty pattern file
                  `grepq -c empty.txt file.fastq inverted` 

           TIPS:

             - Use the `tune` subcommand (`grepq tune -h` for instructions) to analyze matched substrings and update the number and/or order of regex patterns in your pattern file according to their matched frequency. This can speed up the filtering process. Specify that `tune` should output to a JSON file if you want to save the results in a format that preserves the regex names and the name of the regex set (see also the EXAMPLES and NOTES sections).

             - Use the `inverted` subcommand to identify records that do not match any of the regex patterns in your pattern file.

             - Ensure you have enough storage space for output files.

          NOTES:
             - Only supports FASTQ files or gzip compressed FASTQ files that contain DNA sequences.

             - Pattern files must contain one regex pattern per line, and patterns are case-sensitive (you can supply an empty pattern file to count the total number of records in the FASTQ file). The regex patterns should only include the DNA sequence characters (A, C, G, T), and not other IUPAC codes (e.g., not N, R, Y, etc.). If your regex patterns contain any of these other IUPAC codes, then transform them to DNA sequence characters (A, C, G, T) before using them with grepq. See regex.txt and regex.json in the examples directory of `grepq`'s GitHub repository for examples of valid pattern files.

             - When no options are provided, only the matching sequences are printed.

             - Only one of the -I, -R, or -c options can be used at a time.

             - The -x and -z options can be used separately, or together, and in combination any of the other filtering options (the -z option cannot be used with the tune subcommand).

             - The count option (-c) will support the output of the -R option since it is in FASTQ format.

             - When the count option (-c) is given with the `tune` subcommand, `grepq` will count the number of FASTQ records containing a sequence that is matched, for each matching regex in the pattern file. If, however, there are multiple occurrences of a given regex within a FASTQ record sequence field, `grepq` will count this as one match.

             - When the count option (-c) is not given with the `tune` subcommand, `grepq` prints the total number of matching FASTQ records for the set of regex patterns in the pattern file.

             - Regex patterns with look-around and backreferences are not supported.

           CITATION:
          
             If you use `grepq` in your research, please cite as follows:
             
               Crosbie, N.D. (2024). grepq: A Rust application that quickly filters FASTQ files by matching sequences to a set of regex patterns. 10.5281/zenodo.14031703

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
