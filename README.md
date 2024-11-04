<img src="src/grepq-icon.svg" width="128" />

_quickly filter fastq files by matching sequences to a set of regex patterns_

## Performance
**`grepq` is fast.**

On a Mac Studio with 32GB RAM and Apple M1 max chip, `grepq` processed a 104GB fastq file in 88 seconds, about 1.2GB of fastq data per second. For a 874MB fastq file, it was around **4.8** and **450** times faster than the general-purpose regex tools `ripgrep` and `grep`, respectively, on the same hardware. Furthermore, `grepq` will only match regex patterns to the sequence part of the fastq file, which is the most common use case. This is in contrast to `ripgrep` and `grep`, which will match the regex patterns to the entire fastq record, which includes the record ID, sequence, separator, and quality. This can lead to false positives and slow down the filtering process.


## Usage
    
```bash
Usage: grepq [OPTIONS] <PATTERNS> <FILE>

Arguments:
  <PATTERNS>
          Path to the patterns file (one regex pattern per line)

  <FILE>
          Path to the fastq file

Options:
  -I
          Include record ID in the output

  -R
          Include record ID, sequence, separator, and quality in the output

  -c
          Count the number of matching fastq records

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Notes:
    - Only supports fastq files.
    - When no options are provided, only the matching sequences are printed.
    - Only one of the -I, -R, or -c options can be used at a time.
    - Count option (-c) will support the output of the -R option since it is in fastq format.
    - Patterns file must contain one regex pattern per line.
    - Inverted matches are not supported.
    - regex patterns with look-around and backreferences are not supported.
```

- tips
    - order your regex patterns from those that are most likely to match to those that are least likely to match. This will speed up the filtering process.
    - ensure you have enought space storage space for the output file.

## Requirements

- `grepq` has been tested on Linux and macOS. It might work on Windows, but it has not been tested.
- ensure that rust is installed on your system (https://www.rust-lang.org/tools/install)

## Installation

- from source
    - clone the repository and `cd` into the `grepq` directory
    - run `cargo build --release`
    - relative to the cloned parent directory, the executable will be located in `./target/release`

- from Cargo.io
    - `cargo install grepq`

_Checksums to verify `grepq` is working correctly, using the regex file `regex.txt` and the small fastq file `small.fastq`, both located in the `test` directory:_

```bash
./target/release/grepq ./test/regex.txt ./test/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if no option is given
ed0527a4d03481a50b365b03f5d952afab1df259966021699484cd9d59d790fc

./target/release/grepq -I ./test/regex.txt ./test/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if -I option is given
204bec425495f606611ba20605c6fa6e6d10627fc3203126821a2df8af025fb0

./target/release/grepq -R ./test/regex.txt ./test/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if -R option is given
67ad581448b5e9f0feae96b11f7a48f101cd5da8011b8b27a706681f703c6caf
```

## License
MIT

