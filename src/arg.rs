use clap::Parser;
use colored::*;
use std::sync::LazyLock;

static AFTER_HELP: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "Overview:".bold().underline(),
        "\n\n`grepq` searches the sequence line of FASTQ records for regular
expressions that are contained in a text or JSON file, or it searches for the
absence of those regular expressions when used with the `inverted` command. The 
FASTQ file on which it operates can be supplied uncompressed or in gzip or zstd
compressed format. Use the `tune` command in a simple shell script to update the
number and order of regex patterns in your pattern file according to their matched
frequency (refer to the examples directory of the `grepq` GitHub repository:
https://github.com/Rbfinch/grepq), further targeting and speeding up the
filtering process.",
        "\n\nExamples:".bold().underline(),
        "\n\nPrint only the matching sequences".italic(),
        "\n    grepq regex.txt file.fastq".bold(),
        "\n\nPrint the matching sequences with the record ID".italic(),
        "\n    grepq -I regex.txt file.fastq".bold(),
        "\n\nPrint the matching sequences in FASTA format".italic(),
        "\n    grepq -F regex.txt file.fastq".bold(),
        "\n\nPrint the matching sequences in FASTQ format".italic(),
        "\n    grepq -R regex.txt file.fastq".bold(),
        "\n\nSave the matching sequences in gzip compressed FASTQ format".italic(),
        "\n    grepq -R --write-gzip regex.txt file.fastq > output.fastq.gz".bold(),
        "\n\nRead the FASTQ file in gzip compressed format".italic(),
        "\n    grepq --read-gzip regex.txt file.fastq.gz".bold(),
        "\n\nRead and save the output in gzip compressed format, with fast compression"
            .italic(),
        "\n    grepq --read-gzip --write-gzip --fast regex.txt file.fastq.gz > output.fastq.gz"
            .bold(),
        "\n\nRead and save the output in gzip compressed format, with best compression"
            .italic(),
        "\n    grepq --read-gzip --write-gzip --best regex.txt file.fastq.gz > output.fastq.gz"
            .bold(),
        "\n\nRead and save the output in zstd compressed format, with best compression"
            .italic(),
        "\n    grepq --read-zstd --write-zstd --best regex.txt file.fastq.zst > output.fastq.zst"
            .bold(),
        "\n\nCount the number of matching FASTQ records".italic(),
        "\n    grepq -c regex.txt file.fastq".bold(),
        "\n\nFor each matched pattern in a search of the first 100000 records, print the
pattern and the number of matches"
            .italic(),
        "\n    grepq regex.txt file.fastq tune -n 100000 -c".bold(),
        "\n\nFor each matched pattern in a search of the first 100000 records of a
gzip-compressed FASTQ file, print the pattern and the number of matches"
            .italic(),
        "\n    grepq --read-gzip regex.txt file.fastq.gz tune -n 100000 -c".bold(),
        "\n\nFor each matched pattern in a search of the first 100000 records of a
gzip-compressed FASTQ file, print the pattern and the number of matches to a 
JSON file called matches.json"
            .italic(),
        "\n    grepq --read-gzip regex.json file.fastq.gz tune -n 100000 -c --names --json-matches"
            .bold(),
        "\n\nFor each matched pattern in a search of the first 100000 records of a
gzip-compressed FASTQ file, print the pattern and the number of matches to a 
JSON file called matches.json, and include the top three most frequent variants of
each pattern, and their respective counts"
            .italic(),
        "\n    grepq --read-gzip regex.json file.fastq.gz tune -n 100000 -c --names --json-matches --variants 3"
            .bold(),
        "\n\nPrint the records where none of the regex patterns are found".italic(),
        "\n    grepq regex.txt file.fastq inverted".bold(),
        "\n\nPrint the records where none of the regex patterns are found, with the record ID"
            .italic(),
        "\n    grepq -I regex.txt file.fastq inverted".bold(),
        "\n\nPrint the records where none of the regex patterns are found, in FASTQ format"
            .italic(),
        "\n    grepq -R regex.txt file.fastq inverted".bold(),
        "\n\nCount the number of records where none of the regex patterns are found"
            .italic(),
        "\n    grepq -c regex.txt file.fastq inverted".bold(),
        "\n\nCount the total number of records in the FASTQ file using an empty pattern file"
            .italic(),
        "\n    grepq -c empty.txt file.fastq inverted".bold(),
        "\n\nTips:".bold().underline(),
        "\n\n1. Predicates can be used to filter on the header field (= record ID line)
using a regex, minimum sequence length, and minimum average quality score
(supports Phred+33 and Phred+64). Predicates are specified in a JSON pattern file.
For an example, see regex-and-predicates.json in the examples directory of the
`grepq` GitHub repository (https://github.com/Rbfinch/grepq). Note that regex 
supplied to filter on the header field is first passed as a string to the regex
engine, and then the regex engine is used to match the header field. If you get
an error message, be sure to escape any special characters in the regex pattern.

2. Use the `tune` command (`grepq tune -h` for instructions) in a simple shell
script to update the number and order of regex patterns in your pattern file
according to their matched frequency, further targeting and speeding up the
filtering process. When the patterns file is given in JSON format, then specifying
the -c, --names and --json-matches options to the `tune` command will output the
matched substrings and their frequencies in JSON format to a file called 
matches.json, allowing named regex sets and named regex patterns. See 
examples/16S-iupac.json for an example of a JSON pattern file and 
examples/matches.json for an example of the output of the tune command in JSON
format (both files are located in the examples directory of the `grepq` GitHub
repository: https://github.com/Rbfinch/grepq) (see also the Examples and Notes
sections).

3. Use the `inverted` command to identify records that do not match
any of the regex patterns in your pattern file.

4. Ensure you have enough storage space for output files.",
        "\n\nNotes:".bold().underline(),
        "\n\n1. `grepq` can output to several formats, including those that are
gzip or zstd compressed. `grepq`, however, will only accept a FASTQ file or a 
compressed (gzip or zstd) FASTQ file as the sequence data file. If you get an
error message, check that the input data file is a FASTQ file or a gzip or zstd
compressed FASTQ file, and that you have specified the correct file format 
(--read-gzip or --read-zstd for FASTQ files compressed by gzip and zstd,
respectively), and file path.

2. Pattern files must contain one regex pattern per line or be given in JSON
format, and patterns are case-sensitive (you can supply an empty pattern file to
count the total number of records in the FASTQ file). The regex patterns should
only include the DNA sequence characters (A, C, G, T), or IUPAC ambiguity codes
(N, R, Y, ...). See 16S-no-iupac.txt, 16S-iupac.json and  
16S-iupac-and-predicates.json in the examples directory of the `grepq` GitHub
repository (https://github.com/Rbfinch/grepq) for examples of valid pattern files.
Regex patterns to match the header field (= record ID line) must comply with the
Rust regex library syntax (<https://docs.rs/regex/latest/regex/#syntax>). If you
get an error message, be sure to escape any special characters in the regex 
pattern.

3. When no options are provided, only the matching sequences are printed.

4. Only one of the -I, -F, -R, or -c options can be used at a time.

5. The --read-gzip [--read-zstd] and --write-gzip [--write-zstd] options can be
used separately, or together, and in combination with any of the other filtering
options (the --write-gzip [--write-zstd] option cannot be used with the `tune` 
command).

6. The count option (-c) will support the output of the -R option since it is in
FASTQ format.

7. Other than when the `tune` command is run, a FASTQ record is deemed to match
(and hence provided in the output) when any of the regex patterns in the pattern
file match the sequence field of the FASTQ record.
        
8. When the count option (-c) is given with the `tune` command, `grepq` will count
the number of FASTQ records containing a sequence that is matched, for each
matching regex in the pattern file. If, however, there are multiple occurrences
of a given regex within a FASTQ record sequence field, `grepq` will count this as
one match.

9. When the count option (-c) is not given with the `tune` command, `grepq` prints
the total number of matching FASTQ records for the set of regex patterns in the
pattern file.

10. Regex patterns with look-around and backreferences are not supported.",
        "\n\nCitation:".bold().underline(),
        "\n\nIf you use grepq in your research, please cite as follows:",
        "\n\nCrosbie, N.D. (2024). grepq: A Rust application that quickly filters
FASTQ files by matching sequences to a set of regex patterns. 10.5281/zenodo.14031703"
    )
});

#[derive(Parser)]
#[command(
    name = "grepq",
    author = "Nicholas D. Crosbie",
    version = clap::crate_version!(),
    about = "Quickly filter FASTQ files",
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
        help = "Include record ID, sequence, separator, and quality field in the output (i.e. FASTQ format)"
    )]
    pub with_full_record: bool,

    #[arg(short = 'F', long = "fasta", help = "Output in FASTA format")]
    pub with_fasta: bool,

    #[arg(
        short = 'c',
        long = "count",
        help = "Count the number of matching FASTQ records"
    )]
    pub count: bool,

    #[arg(
 //       short = 'x',
        long = "read-gzip",
        help = "Read the FASTQ file in gzip compressed format"
    )]
    pub gzip_input: bool,

    #[arg(
 //       short = 'z',
        long = "write-gzip",
        help = "Write the output in gzip compressed format"
    )]
    pub gzip_output: bool,

    #[arg(
        long = "read-zstd",
        help = "Read the FASTQ file in zstd compressed format"
    )]
    pub zstd_input: bool,

    #[arg(
        long = "write-zstd",
        help = "Write the output in zstd compressed format"
    )]
    pub zstd_output: bool,

    #[arg(short = 'f', long = "fast", help = "Use fast compression")]
    pub fast_compression: bool,

    #[arg(short = 'b', long = "best", help = "Use best compression")]
    pub best_compression: bool,

    #[arg(help = "Path to the patterns file in plain text or JSON format")]
    pub patterns: String,

    #[arg(help = "Path to the FASTQ file in plain text or gzip compressed format")]
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

    #[arg(
        short = 'v',
        long = "variants",
        help = "Number of top most frequent variants to include in the output",
        default_value_t = 1
    )]
    pub variants: usize,
}
