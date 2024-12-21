use clap::Parser;
use colored::*;
use std::sync::LazyLock;

static AFTER_HELP: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}
    {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "Examples:".bold().underline(),
        "\n\nPrint only the matching sequences".italic(),
        "\n    grepq regex.txt file.fastq".bold(),
        "\n\nPrint the matching sequences with the record ID".italic(),
        "\n    grepq -I regex.txt file.fastq".bold(),
        "\n\nPrint the matching sequences in FASTQ format".italic(),
        "\n    grepq -R regex.txt file.fastq".bold(),
        "\n\nSave the matching sequences in gzip compressed FASTQ format".italic(),
        "\n    grepq -R -z regex.txt file.fastq > output.fastq.gz".bold(),
        "\n\nRead the FASTQ file in gzip compressed format".italic(),
        "\n    grepq -x regex.txt file.fastq.gz".bold(),
        "\n\nRead and save the output in gzip compressed format, with fast
compression"
            .italic(),
        "\n    grepq -xz --fast regex.txt file.fastq.gz > output.fastq.gz".bold(),
        "\n\nRead and save the output in gzip compressed format, with best
compression"
            .italic(),
        "\n    grepq -xz --best regex.txt file.fastq.gz > output.fastq.gz".bold(),
        "\n\nCount the number of matching FASTQ records".italic(),
        "\n    grepq -c regex.txt file.fastq".bold(),
        "\n\nFor each matched pattern in a search of the first 100000 records,
print the pattern and the number of matches"
            .italic(),
        "\n    grepq regex.txt file.fastq tune -n 100000 -c".bold(),
        "\n\nFor each matched pattern in a search of the first 100000 records of
a gzip-compressed FASTQ file, print the pattern and the number of matches"
            .italic(),
        "\n    grepq -x regex.txt file.fastq.gz tune -n 100000 -c".bold(),
        "\n\nFor each matched pattern in a search of the first 100000 records of
a gzip-compressed FASTQ file, print the pattern and the number of matches to a 
JSON file called matches.json"
            .italic(),
        "\n    grepq -xj regex.json file.fastq.gz tune -n 100000 -c --names --json-matches".bold(),
        "\n\nPrint the records where none of the regex patterns are found".italic(),
        "\n    grepq regex.txt file.fastq inverted".bold(),
        "\n\nPrint the records where none of the regex patterns are found, with
the record ID"
            .italic(),
        "\n    grepq -I regex.txt file.fastq inverted".bold(),
        "\n\nPrint the records where none of the regex patterns are found, in 
FASTQ format"
            .italic(),
        "\n    grepq -R regex.txt file.fastq inverted".bold(),
        "\n\nCount the number of records where none of the regex patterns are
found"
            .italic(),
        "\n    grepq -c regex.txt file.fastq inverted".bold(),
        "\n\nCount the total number of records in the FASTQ file using an empty
pattern file"
            .italic(),
        "\n    grepq -c empty.txt file.fastq inverted".bold(),
        "\n\nTips:".bold().underline(),
        "\n\nUse the `tune` subcommand (`grepq tune -h` for instructions) to
analyze matched substrings and update the number and/or order of regex patterns
in your pattern file according to their matched frequency. This can speed up the
filtering process. Specify that `tune` should output to a JSON file if you want
to save the results in a format that preserves the regex names and the name of
the regex set (see also the Examples and Notes sections).",
        "\n\nUse the `inverted` subcommand to identify records that do not match
any of the regex patterns in your pattern file.",
        "\n\nEnsure you have enough storage space for output files.",
        "\n\nNotes:".bold().underline(),
        "\n\nOnly supports FASTQ files or gzip compressed FASTQ files that
contain DNA sequences.",
        "\n\nPattern files must contain one regex pattern per line, and patterns
are case-sensitive (you can supply an empty pattern file to count the total
number of records in the FASTQ file). The regex patterns should only include the
DNA sequence characters (A, C, G, T), and not other IUPAC codes (not N, R, Y, ...).
If your regex patterns contain any of these other IUPAC codes, then transform
them to DNA sequence characters (A, C, G, T) before using them with `grepq`. See
regex.txt and regex.json in the examples directory of `grepq`'s GitHub repository
for examples of valid pattern files.",
        "\n\nWhen no options are provided, only the matching sequences are
printed.",
        "\n\nOnly one of the -I, -R, or -c options can be used at a time.",
        "\n\nThe -x and -z options can be used separately, or together, and in
combination any of the other filtering options (the -z option cannot be used
with the tune subcommand).",
        "\n\nThe count option (-c) will support the output of the -R option 
since it is in FASTQ format.",
        "\n\nWhen the count option (-c) is given with the `tune` subcommand,
`grepq` will count the number of FASTQ records containing a sequence that is
matched, for each matching regex in the pattern file. If, however, there are
multiple occurrences of a given regex within a FASTQ record sequence field,
`grepq` will count this as one match.",
        "\n\nWhen the count option (-c) is not given with the `tune` subcommand,
`grepq` prints the total number of matching FASTQ records for the set of regex
patterns in the pattern file.",
        "\n\nRegex patterns with look-around and backreferences are not supported.",
        "\n\nCitation:".bold().underline(),
        "\n\nIf you use `grepq` in your research, please cite as follows:",
        "\n\nCrosbie, N.D. (2024). grepq: A Rust application that quickly filters
FASTQ files by matching sequences to a set of regex patterns. 10.5281/zenodo.14031703"
    )
});

#[derive(Parser)]
#[command(
    name = "grepq",
    author = "Nicholas D. Crosbie",
    version = clap::crate_version!(),
    about = "Quickly filter FASTQ files by matching sequences to a set of regex patterns",
    long_about = "Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License.",
    after_help = &**AFTER_HELP
)]
pub struct Cli {
    #[arg(
        short = 'I',
        long = "includeID",
        help = "Include record ID in the output"
    )]
    pub with_id: bool,

    #[arg(
        short = 'R',
        long = "includeRecord",
        help = "Include record ID, sequence, separator, and quality in the output"
    )]
    pub with_full_record: bool,

    #[arg(
        short = 'c',
        long = "count",
        help = "Count the number of matching FASTQ records"
    )]
    pub count: bool,

    #[arg(
        short = 'x',
        long = "extract",
        help = "Read the FASTQ file in gzip compressed format"
    )]
    pub gzip_input: bool,

    #[arg(
        short = 'z',
        long = "zip",
        help = "Write the output in gzip compressed format"
    )]
    pub gzip_output: bool,

    #[arg(short = 'f', long = "fast", help = "Use fast compression")]
    pub fast_compression: bool,

    #[arg(short = 'b', long = "best", help = "Use best compression")]
    pub best_compression: bool,

    #[arg(help = "Path to the patterns file")]
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
