<img src="src/grepq-icon.svg" width="128" />

_quickly filter fastq files by matching sequences to a set of regex patterns_

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.14058563.svg)](https://doi.org/10.5281/zenodo.14058563)

## Features

**1. Very fast and scales to large fastq files**

On a Mac Studio with 32GB RAM and Apple M1 max chip, `grepq` processed a 104GB fastq file against 30 regex patterns in 88 seconds, about 1.2GB of fastq data per second. And for the same fastq file and 30 regex patterns, getting an ordered count of each matched regex using the `tune` subcommand took less than five seconds for 100,000 fastq records.

For a 874MB fastq file, it was around **4.8** and **450** times faster than the general-purpose regex tools `ripgrep` and `grep`, respectively, on the same hardware. 

**2. Does not match false positives**

`grepq` will only match regex patterns to the sequence part of the fastq file, which is the most common use case. Unlike `ripgrep` and `grep`, which will match the regex patterns to the entire fastq record, which includes the record ID, sequence, separator, and quality. This can lead to false positives and slow down the filtering process.

**3. Output matched sequences to one of three formats**

- matched sequences only
- matched sequences and their corresponding record IDs
- matched sequences, their corresponding record IDs, and the quality scores (i.e. fastq format)

**4. Will tune your pattern file with the `tune` subcommand**

Use the `tune` subcommand to analyze matched substrings and update the number and/or order of regex patterns in your pattern file according to their matched frequency. This can speed up the filtering process. 

Specifying the `-c` option to the `tune` subcommand will output the matched substrings and their frequencies, ranked from highest to lowest.

**5. Plays nicely with your unix workflows**

For example:

```bash
#!/bin/bash

# This two-line script shows an example of tuning the regular expression pattern file using the tune subcommand.
grepq regex.txt file.fastq tune -n 50 | head -n 2 > tunedRegs.txt
grepq tunedRegs.txt file.fastq > tuned-seqs.txt
```

## Usage 
Get instructions using `grepq -h`, and `grepq tune -h` for more information on the tuning options.

## Requirements

- `grepq` has been tested on Linux and macOS. It might work on Windows, but it has not been tested.
- Ensure that Rust is installed on your system (https://www.rust-lang.org/tools/install)
- If the build fails, make sure you have the latest version of the Rust compiler by running `rustup update`

## Installation

- From *source*
    - Clone the repository and `cd` into the `grepq` directory
    - Run `cargo build --release`
    - Relative to the cloned parent directory, the executable will be located in `./target/release`
    - Make sure the executable is in your `PATH` or use the full path to the executable

- From *crates.io* (easiest method)
    - `cargo install grepq`

_Checksums to verify `grepq` is working correctly, using the regex file `regex.txt` and the small fastq file `small.fastq`, both located in the `examples` directory:_

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

## Update changes
see [CHANGELOG](https://github.com/Rbfinch/grepq/blob/main/CHANGELOG.md)

## License
MIT

