use clap::Parser;

#[derive(Parser)]
#[command(
    name = "grepq",
    author = "Nicholas D. Crosbie",
    version = clap::crate_version!(),
    about = "quickly filter fastq files by matching sequences to a set of regex patterns",
    long_about = "Copyright (c) 2024 Nicholas D. Crosbie, licensed under the MIT License.",
    after_help = "
       Examples:
             - Print only the matching sequences:
                  grepq regex.txt file.fastq > outfile.txt

             - Print the matching sequences with the record ID:
                  grepq -I regex.txt file.fastq > outfile.txt

             - Print the matching sequences with the record ID, sequence, separator, and quality fields
                  grepq -R regex.txt file.fastq > outfile.txt

             - Count the number of matching fastq records:
                  grepq -c regex.txt file.fastq

           Tips:
             - Order your regex patterns from those that are most likely to match to those that
               are least likely to match. This will speed up the filtering process.

             - Ensure you have enough storage space for the output file.

          Notes:
             - Only supports fastq files.

             - Patterns file must contain one regex pattern per line.

             - When no options are provided, only the matching sequences are printed.

             - Only one of the -I, -R, or -c options can be used at a time.

             - Count option (-c) will support the output of the -R option since it is in fastq format.

             - Inverted matches are not supported.

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

    #[arg(short = 'c', help = "Count the number of matching fastq records")]
    pub count: bool,

    #[arg(help = "Path to the patterns file (one regex pattern per line)")]
    pub patterns: String,

    #[arg(help = "Path to the fastq file")]
    pub file: String,
}
