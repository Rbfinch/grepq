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

use clap::Parser;
use colored::*;
use std::sync::LazyLock;

static AFTER_HELP: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "Overview:".bold().underline(),
        "\n\n`grepq` searches the sequence line of FASTQ records for regular
expressions that are contained in a text or JSON file, or it searches for the
absence of those regular expressions when used with the `inverted` command. The 
FASTQ file on which it operates can be supplied uncompressed or in gzip or zstd
compressed format. Use the `tune` or `summarise` command in a simple shell script
to update the number and order of regex patterns in your pattern file according
to their matched frequency (refer to the examples directory of the `grepq` GitHub
repository, https://github.com/Rbfinch/grepq), further targeting and speeding up
the filtering process.",
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
        "\n\nFor each matched pattern in a search of no more than 100000 matches, print the
    pattern and the number of matches"
            .italic(),
        "\n    grepq regex.txt file.fastq tune -n 100000 -c".bold(),
        "\n\nFor each matched pattern in a search of no more than 100000 matches of a
    gzip-compressed FASTQ file, print the pattern and the number of matches"
            .italic(),
        "\n    grepq --read-gzip regex.txt file.fastq.gz tune -n 100000 -c".bold(),
        "\n\nFor each matched pattern in a search of no more than 100000 matches of a
gzip-compressed FASTQ file, print the pattern and the number of matches to a 
JSON file called matches.json"
            .italic(),
        "\n    grepq --read-gzip regex.json file.fastq.gz tune -n 100000 -c --names --json-matches"
            .bold(),
        "\n\nAs above, but uses the summarise command to ensure that all FASTQ records are processed"
            .italic(),
        "\n    grepq --read-gzip regex.json file.fastq.gz summarise -c --names --json-matches"
            .bold(),
        "\n\nFor each matched pattern in a search of no more than 100000 matches of a
gzip-compressed FASTQ file, print the pattern and the number of matches to a 
JSON file called matches.json, and include the top three most frequent variants of
each pattern, and their respective counts"
            .italic(),
        "\n    grepq --read-gzip regex.json file.fastq.gz tune -n 100000 -c --names --json-matches --variants 3"
            .bold(),
        "\n\nAs above, but uses the summarise command to ensure that all FASTQ records are processed"
            .italic(),
        "\n    grepq --read-gzip regex.json file.fastq.gz summarise -c --names --json-matches --variants 3"
            .bold(),
        "\n\nFor each matched pattern in a search of no more than 100000 matches of a
gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON
file called matches.json, and include all variants of each pattern, and their
respective counts. Note that the `--variants` argument is not given when `--all`
is specified"
            .italic(),
        "\n    grepq --read-gzip regex.json file.fastq.gz tune -n 100000 -c --names --json-matches --all"
            .bold(),
        "\n\nAs above, but uses the summarise command to ensure that all FASTQ records are processed"
            .italic(),
        "\n    grepq --read-gzip regex.json file.fastq.gz summarise -c --names --json-matches --all"
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
        "\n\nFor a gzip-compressed FASTQ file, bucket matched sequences into separate files
named after each regexName, with the output in FASTQ format".italic(),
        "\n    grepq -R --bucket --read-gzip regex.json file.fastq.gz".bold(),
        "\n\nFor a gzip-compressed FASTQ file, bucket matched sequences into separate files
named after each regexName, with the output in FASTQ format, and write a SQLite
database file, limiting the number of tetranucleotides in the TNF and CTNF fields
to two".italic(),
        "\n    grepq -R --read-gzip --writeSQL -N 2 --bucket regex.json file.fastq.gz".bold(),
        "\n\nTips:".bold().underline(),
        "\n\n1. Predicates can be used to filter on the header field (= record ID line)
using a regex, minimum sequence length, and minimum average quality score
(supports Phred+33 and Phred+64). Predicates are specified in a JSON pattern file.
For an example, see regex-and-predicates.json in the examples directory of the
`grepq` GitHub repository (https://github.com/Rbfinch/grepq). Note that regex 
supplied to filter on the header field is first passed as a string to the regex
engine, and then the regex engine is used to match the header field. If you get
an error message, be sure to escape any special characters in the regex pattern.

2. Use the `tune` or `summarise` command (`grepq tune -h` and `grepq summarise -h`
for instructions) in a simple shell script to update the number and order of regex
patterns in your pattern file according to their matched frequency, further targeting
and speeding up the filtering process. When the patterns file is given in JSON 
format, then specifying the `-c`, `--names`, `--json-matches` and `--variants` 
options to the `tune` or `summarise` command will output the matched pattern 
variants and their corresponding counts in JSON format to a file called `matches.json`,
allowing named regex sets, named regex patterns, and named and unnamed variants.
See 16S-no-iupac.txt, 16S-iupac.json, 16S-no-iupac.json and 16S-no-iupac.json for
examples of JSON pattern files, and matches.json for an example of the output of
the `tune` or `summarise` command in JSON format (example files are located in the
examples directory of the `grepq` GitHub repository: https://github.com/Rbfinch/grepq)
(see also the Examples and Notes sections). To list all variants of a pattern, use
the `--all` option. Note that the `--variants` argument is not given when `--all`
is specified.

3. Use the `inverted` command to identify records that do not match any of the
regex patterns in your pattern file.

4. Ensure you have enough storage space for output files.",
        "\n\nNotes:".bold().underline(),
        "\n\n1. `grepq` can output to several formats, including those that are
gzip or zstd compressed. `grepq`, however, will only accept a FASTQ file or a 
compressed (gzip or zstd) FASTQ file as the sequence data file. If you get an
error message, check that the input data file is a FASTQ file or a gzip or zstd
compressed FASTQ file, and that you have specified the correct file format 
(--read-gzip or --read-zstd for FASTQ files compressed by gzip and zstd,
respectively), and file path.

2. Other than when the `inverted` command is given, output to a SQLite database
is supported with the `writeSQL` option. The SQLite database will contain a table
called `fastq_data` with the following fields: the fastq record (header, sequence
and quality fields), length of the sequence (length), percent GC content (GC),
percent GC content as an integer (GC_int), number of unique tetranucleotides in the
sequence (nTN), number of unique canonical tetranucleotides in the sequence (nCTN),
percent tetranucleotide frequency in the sequence (TNF), percent canonical 
tetranucleotide frequency in the sequence (CTNF), and a JSON array containing the
matched regex patterns, the matches and their position(s) in the FASTQ sequence 
(variants). If the pattern file was given in JSON format and contained a non-null
qualityEncoding field, then the average quality score for the sequence 
(average_quality) will also be written. The `--num-tetranucleotides` option can be
used to limit the number of tetranucleotides written to the TNF and CTNF fields of
the fastq_data SQLite table, these being the most or equal most frequent 
tetranucleotides and canonical tetranucleotides in the sequence of the matched 
FASTQ records. A summary of the invoked query (pattern and data files) is written
to a second table called `query`.

3. Pattern files must contain one regex pattern per line or be given in JSON
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

4. When no options are provided, only the matching sequences are printed.

5. Only one of the -I, -F, -R, or -c options can be used at a time.

6. The --read-gzip [--read-zstd] and --write-gzip [--write-zstd] options can be
used separately, or together, and in combination with any of the other filtering
options (the --write-gzip [--write-zstd] option cannot be used with the `tune` 
or `summarise` command).

7. The count option (-c) will support the output of the -R option since it is in
FASTQ format.

8. Other than when the `tune` or `summarise` command is run, a FASTQ record is
deemed to match (and hence provided in the output) when any of the regex patterns
in the pattern file match the sequence of the FASTQ record.
        
9. When the count option (-c) is given with the `tune` or `summarise` command,
`grepq` will count the number of FASTQ records containing a sequence that is 
matched, for each matching regex in the pattern file. If, however, there are 
multiple occurrences of a given regex in a FASTQ record sequence, `grepq`
will count this as one match. To ensure all records are processed, use the 
`summarise` command instead of the `tune` command.

10. When the count option (-c) is not given as part of the `tune` or `summarise`
command, `grepq` prints the total number of matching FASTQ records for the set
of regex patterns in the pattern file.

11. Regex patterns with look-around and backreferences are not supported.",
        "\n\nCitation:".bold().underline(),
        "\n\nIf you use grepq in your research, please cite as follows:",
        "\n\nCrosbie, N.D. (2024). grepq: A Rust application that quickly filters
FASTQ files by matching sequences to a set of regular expressions. bioRxiv, doi:
<https://doi.org/10.1101/2025.01.09.632104>"
    )
});

#[derive(Parser)]
#[command(
    name = "grepq",
    author = "Nicholas D. Crosbie",
    version = clap::crate_version!(),
    about = "Quickly filter FASTQ files",
    long_about = "Copyright (c) 2024 - present: Nicholas D. Crosbie, licensed under the MIT License.",
    after_help = &**AFTER_HELP
)]
pub struct Cli {
    #[arg(long, hide = true)]
    pub markdown_help: bool,

    #[arg(
        short = 'I',
        long = "includeID",
        help = "Include record ID in the output"
    )]
    pub with_id: bool,

    #[arg(
        short = 'R',
        long = "includeRecord",
        help = "Include record ID, sequence, separator, and quality field in the
output (i.e. FASTQ format)"
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

    #[arg(
        long = "bucket",
        help = "Write matched sequences to separate files named after each regexName"
    )]
    pub bucket: bool,

    #[arg(
        long = "writeSQL",
        help = "Write matching records to SQLite database, along with length
of the sequence (length), percent GC content (GC), percent GC content as
an integer (GC_int), number of unique tetranucleotides in the sequence (nTN),
number of unique canonical tetranucleotides in the sequence (nCTN), percent 
tetranucleotide frequency in the sequence (TNF), percent canonical
tetranucleotide frequency in the sequence (CTNF), and average quality score
for the sequence (average_quality) if qualityEncoding is not null"
    )]
    pub write_sql: bool,

    #[arg(
        short = 'N',
        long = "num-tetranucleotides",
        help = "Limit the number of tetranucleotides written to the TNF field of
the fastq_data SQLite table, these being the most or equal most frequent
tetranucleotides in the sequence of the matched FASTQ records",
        requires = "write_sql"
    )]
    pub num_tetranucleotides: Option<usize>,

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
    #[command(about = "Summarise records matching regex patterns and variants in
the FASTQ file")]
    Summarise(Summarise),
}

#[derive(Parser)]
pub struct Summarise {
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
        long = "variants",
        help = "Number of top most frequent variants to include in the output"
    )]
    pub variants: Option<usize>,

    #[arg(long = "all", help = "Include all variants in the output")]
    pub all_variants: bool,
}

#[derive(Parser)]
pub struct Tune {
    #[arg(help = "Total number of matches", short = 'n')]
    pub num_matches: usize,

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
        long = "variants",
        help = "Number of top most frequent variants to include in the output"
    )]
    pub variants: Option<usize>,

    #[arg(long = "all", help = "Include all variants in the output")]
    pub all_variants: bool,
}
