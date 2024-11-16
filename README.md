<img src="src/grepq-icon.svg" width="128" />

_quickly filter FASTQ files by matching sequences to a set of regex patterns_

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.14058563.svg)](https://doi.org/10.5281/zenodo.14058563)

## Feature set
- very fast and scales to large FASTQ files
- gzip support
- does not match false positives
- output matched sequences to one of three formats
- tune your pattern file with the `tune` subcommand
- supports inverted matching with the `inverted` subcommand
- plays nicely with your unix workflows

## Features and performance in detail
**1. Very fast and scales to large FASTQ files**

| tool    | time (s) | &times; grep speedup | &times; ripgrep speedup |
|---------|----------|----------------------|-------------------------|
| grepq   | 0.22     | 1558                 | 16                      |
| ripgrep | 3.57     | 96                   | NA                      |
| grep    | 342.79   | NA                   | NA                      |

*2022 model Mac Studio with 32GB RAM and Apple M1 max chip running macOS 15.0.1. The FASTQ file (SRX26365298.fastq) was 874MB in size and was stored on the internal SSD (APPLE SSD AP0512R). The pattern file contained 30 regex patterns (see `examples/regex.txt` for the patterns used). Under the same conditions and using the same pattern file, `grepq` processed a 104GB FASTQ file in 26 seconds (4GB/s) (`grepq` v1.1.8, `ripgrep` v14.1.1 and `grep` 2.6.0-FreeBSD. `ripgrep` and `grep` were run with the default settings).*

**2. Reads and writes regular or gzip-compressed FASTQ files**

Use the `--best` option for best compression, or the `--fast` option for faster compression. 

| tool    | time (s) | &times; grep speedup | &times; ripgrep speedup |
|---------|----------|----------------------|-------------------------|
| grepq   | 2.30     | 149                  | 1.6                     |
| ripgrep | 3.59     | 95                   | NA                      |
| grep    | 343.57   | NA                   | NA                      |

*Conditions and versions as above, but the FASTQ file was gzip-compressed. `grepq` was run with the `-x` option, `ripgrep` with the `-z` option, and `grep` with the `-Z` option.*


**3. Does not match false positives**

`grepq` will only match regex patterns to the sequence field of a FASTQ record, which is the most common use case. Unlike `ripgrep` and `grep`, which will match the regex patterns to the entire FASTQ record, which includes the record ID, sequence, separator, and quality. This can lead to false positives and slow down the filtering process.

**4. Output matched sequences to one of three formats**

- sequences only (default)
- sequences and their corresponding record IDs (`-I` option)
- FASTQ format (`-R` option)

**5. Will tune your pattern file with the `tune` subcommand**

Use the `tune` subcommand to analyze matched substrings and update the number and/or order of regex patterns in your pattern file according to their matched frequency. This can speed up the filtering process. 

Specifying the `-c` option to the `tune` subcommand will output the matched substrings and their frequencies, ranked from highest to lowest.

**6. Supports inverted matching with the `inverted` subcommand**

Use the `inverted` subcommand to output sequences that do not match any of the regex patterns in your pattern file.

**7. Plays nicely with your unix workflows**

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

_File sizes of outfiles to verify `grepq` is working correctly, using the regex file `regex.txt` and the small fastq file `small.fastq`, both located in the `examples` directory:_

(note replace `./target/release/grepq` with `grepq` if you installed from *crates.io*)

```bash
grepq ./examples/regex.txt ./examples/small.fastq > outfile.txt 
15953

grepq  ./examples/regex.txt ./examples/small.fastq inverted > outfile.txt
736547

grepq -I ./examples/regex.txt ./examples/small.fastq > outfile.txt
19515

grepq -I ./examples/regex.txt ./examples/small.fastq inverted > outfile.txt 
901271

grepq -R ./examples/regex.txt ./examples/small.fastq > outfile.txt
35574

grepq -R ./examples/regex.txt ./examples/small.fastq inverted > outfile.txt 
1642712
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