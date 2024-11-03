# `grepq` 
_quickly filter fastq files by matching sequences to set of regex patterns_

## Performance
**`grepq` is fast.**

On a Mac Studio with 32GB RAM and Apple M1 max chip, `grepq` processed a 104GB fastq file in 88 seconds, about 1.2GB of fastq data per second.

- output of `time` command
```bash
❯ time ./grepq/target/release/grepq ./test/regex.txt /path/to/SRX22685872.fastq > outfile.txt
________________________________________________________
Executed in   88.43 secs    fish           external
   usr time   73.95 secs    0.11 millis   73.95 secs
   sys time   11.94 secs   12.92 millis   11.93 secs
```
- notes
    - the regex file `regex.txt` contained 30 regex patterns, and `SRX22685872.fastq` is 104GB in size.

- further hardware details

  - Model Name:	Mac Studio
  - Model Identifier:	Mac13,1
  - Model Number:	MJMV3X/A (2022)
  - Chip:	Apple M1 Max
  - Total Number of Cores:	10 (8 performance and 2 efficiency)
  - Memory:	32 GB
  - APPLE SSD AP0512R
  - OS: macOS 15.0.1 (24A348)

## Usage
    
```bash
Usage: grepq [OPTIONS] <PATTERNS> <FILE>

Arguments:
  <PATTERNS>  Path to the patterns file (one regex pattern per line)
  <FILE>      Path to the fastq file

Options:
  -I             Include record ID in the output
  -R             Include record ID, sequence, separator, and quality in the output
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
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
    - relative to the cloned parent directory, the executable will be located in `./grepq/target/release`

_Checksums to verify `grepq` is working correctly, using the regex file `regex.txt` and the small fastq file `small.fastq`, both located in the `test` directory:_

```bash
./grepq/target/release/grepq ./test/regex.txt ./test/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if no option is given
ed0527a4d03481a50b365b03f5d952afab1df259966021699484cd9d59d790fc

./grepq/target/release/grepq -I ./test/regex.txt ./test/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if -I option is given
204bec425495f606611ba20605c6fa6e6d10627fc3203126821a2df8af025fb0

./grepq/target/release/grepq -R ./test/regex.txt ./test/small.fastq > outfile.txt
sha256sum outfile.txt # checksum of outfile.txt if -R option is given
67ad581448b5e9f0feae96b11f7a48f101cd5da8011b8b27a706681f703c6caf
```

## License
MIT

