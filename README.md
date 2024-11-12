<img src="src/grepq-icon.svg" width="128" />

_quickly filter FASTQ files by matching sequences to a set of regex patterns_

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.14058563.svg)](https://doi.org/10.5281/zenodo.14058563)

## Features
**1. Very fast and scales to large FASTQ files**

| tool    | time (s) | &times; grep speedup | &times; ripgrep speedup |
|---------|----------|----------------------|-------------------------|
| grepq   | 0.31     | 1123                 | 11                      |
| ripgrep | 3.50     | 98                   | NA                      |
| grep    | 342.79   | NA                   | NA                      |

*Test conditions*

Mac Studio (2022 model) with 32GB RAM and Apple M1 max chip running macOS 15.0.1. The FASTQ file was 874MB in size and was stored on the internal SSD (APPLE SSD AP0512R). The pattern file contained 30 regex patterns (see `examples/regex.txt` for the patterns used).

Under the same conditions and using the same pattern file, `grepq` processed a 104GB FASTQ file in 26 seconds (4GB/s).

*Versions of the tools used*

 `grepq` v1.1.5, `ripgrep` v14.1.1 and `grep` 2.6.0-FreeBSD. `ripgrep` and `grep` were run with the default settings.

**2. Does not match false positives**

`grepq` will only match regex patterns to the sequence field of a FASTQ record, which is the most common use case. Unlike `ripgrep` and `grep`, which will match the regex patterns to the entire FASTQ record, which includes the record ID, sequence, separator, and quality. This can lead to false positives and slow down the filtering process.

**3. Output matched sequences to one of three formats**

- sequences only (default)
- sequences and their corresponding record IDs (`-I` option)
- FASTQ format (`-R` option)

**4. Will tune your pattern file with the `tune` subcommand**

Use the `tune` subcommand to analyze matched substrings and update the number and/or order of regex patterns in your pattern file according to their matched frequency. This can speed up the filtering process. 

Specifying the `-c` option to the `tune` subcommand will output the matched substrings and their frequencies, ranked from highest to lowest.

**5. Supports inverted matching with the `inverted` subcommand**

Use the `inverted` subcommand to output sequences that do not match any of the regex patterns in your pattern file.

**6. Plays nicely with your unix workflows**

For example, see `tune.sh` in the `examples` directory. This simple script will filter a FASTQ file using `grepq`, tune the pattern file on a user-specified number of FASTQ records, and then filter the FASTQ file again using the tuned pattern file for a user-specified number of the most frequent regex pattern matches.

## Usage 
Get instructions and examples using `grepq -h`, and `grepq tune -h` and `grepq inverted -h` for more information on the `tune` and `inverted` subcommands, respectively.

## Requirements

- `grepq` has been tested on Linux and macOS. It might work on Windows, but it has not been tested.
- Ensure that Rust is installed on your system (https://www.rust-lang.org/tools/install)
- If the build fails, make sure you have the latest version of the Rust compiler by running `rustup update`

## Installation
- From *crates.io* (easiest method)
    - `cargo install grepq`

- From *source*
    - Clone the repository and `cd` into the `grepq` directory
    - Run `cargo build --release`
    - Relative to the cloned parent directory, the executable will be located in `./target/release`
    - Make sure the executable is in your `PATH` or use the full path to the executable

## Examples
`grepq -h` will show you the available options and subcommands, with examples of how to use them.

_Checksums to verify `grepq` is working correctly, using the regex file `regex.txt` and the small fastq file `small.fastq`, both located in the `examples` directory:_

(note replace `./target/release/grepq` with `grepq` if you installed from *crates.io*)

```bash
./target/release/grepq ./examples/regex.txt ./examples/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if no option is given
ed0527a4d03481a50b365b03f5d952afab1df259966021699484cd9d59d790fc

./target/release/grepq -I ./examples/regex.txt ./examples/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if -I option is given
204bec425495f606611ba20605c6fa6e6d10627fc3203126821a2df8af025fb0

./target/release/grepq -R ./examples/regex.txt ./examples/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if -R option is given
67ad581448b5e9f0feae96b11f7a48f101cd5da8011b8b27a706681f703c6caf
```

- **SARS-CoV-2 example**

Count of the top five most frequently matched patterns found in SRX26602697.fastq using the pattern file SARS-CoV-2.txt (this pattern file contains 64 sequences of length 60 from Table II of this [preprint](https://doi.org/10.1101/2021.04.14.439840)):

```bash
time grepq SARS-CoV-2.txt SRX26602697.fastq tune -n 10000 -c | head -5
GTATGGAAAAGTTATGTGCATGTTGTAGACGGTTGTAATTCATCAACTTGTATGATGTGT: 1595
CGGAACGTTCTGAAAAGAGCTATGAATTGCAGACACCTTTTGAAATTAAATTGGCAAAGA: 693
TCCTTACTGCGCTTCGATTGTGTGCGTACTGCTGCAATATTGTTAACGTGAGTCTTGTAA: 356
GCGCTTCGATTGTGTGCGTACTGCTGCAATATTGTTAACGTGAGTCTTGTAAAACCTTCT: 332
CCGTAGCTGGTGTCTCTATCTGTAGTACTATGACCAATAGACAGTTTCATCAAAAATTAT: 209

________________________________________________________
Executed in  236.47 millis    fish           external
   usr time  203.88 millis    0.12 millis  203.76 millis
   sys time   34.74 millis   13.57 millis   21.16 millis

```

## Update changes
see [CHANGELOG](https://github.com/Rbfinch/grepq/blob/main/CHANGELOG.md)

## License
MIT