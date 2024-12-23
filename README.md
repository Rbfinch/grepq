<img src="src/grepq-icon.svg" width="128" />

_Quickly filter FASTQ files by matching sequences to a set of regex patterns_

[![Crates.io](https://img.shields.io/crates/v/grepq.svg)](https://crates.io/crates/grepq)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Feature set

- very fast and scales to large FASTQ files
- IUPAC ambiguity code support
- gzip support
- JSON support for pattern file input and `tune` command output, allowing named regex sets and named regex patterns
- use **predicates** to filter on header field (using a regex), minimum sequence length, and minimum total quality score (supports Phred+33 and Phred+64)
- does not match false positives
- output matched sequences to one of three formats
- tune your pattern file with the `tune` command
- supports inverted matching with the `inverted` command
- plays nicely with your unix workflows
- comprehensive help, examples and testing script

## Features and performance in detail
**1. Very fast and scales to large FASTQ files**

| tool    | time (s) | &times; grep speedup | &times; ripgrep speedup |
|---------|----------|----------------------|-------------------------|
| grepq   | 0.22     | 1558                 | 16                      |
| ripgrep | 3.57     | 96                   | NA                      |
| grep    | 342.79   | NA                   | NA                      |

<details>
  <summary>Details</summary>
  <p>2022 model Mac Studio with 32GB RAM and Apple M1 max chip running macOS 15.0.1. The FASTQ file (SRX26365298.fastq) was 874MB in size and was stored on the internal SSD (APPLE SSD AP0512R). The pattern file contained 30 regex patterns (see `examples/16S-no-iupac.txt` for the patterns used). Under the same conditions and using the same pattern file, `grepq` processed a 104GB FASTQ file in 26 seconds (4GB/s) (`grepq` v1.1.8, `ripgrep` v14.1.1 and `grep` 2.6.0-FreeBSD. `ripgrep` and `grep` were run with the default settings).</p>
</details>

**2. Reads and writes regular or gzip-compressed FASTQ files**

Use the `--best` option for best compression, or the `--fast` option for faster compression. 

| tool    | time (s) | &times; grep speedup | &times; ripgrep speedup |
|---------|----------|----------------------|-------------------------|
| grepq   | 2.30     | 149                  | 1.6                     |
| ripgrep | 3.59     | 95                   | NA                      |
| grep    | 343.57   | NA                   | NA                      |

<details>
  <summary>Details</summary>
  <p>Conditions and versions as above, but the FASTQ file was gzip-compressed. `grepq` was run with the `-x` option, `ripgrep` with the `-z` option, and `grep` with the `-Z` option.</p>
</details>

**3. Predicates**

Predicates can be used to filter on the header field (with regex), minimum sequence length, and minimum total quality score (supports Phred+33 and Phred+64). 

>[!NOTE]
A regex supplied to filter on the header field is first passed as a string to the regex engine, and then the regex engine is used to match the header field. If you get an error message, be sure to escape any special characters in the regex pattern.

Predicates are specified in a JSON pattern file. For an example, see `16S-iupac-and-predicates.json` in the `examples` directory.

**4. Does not match false positives**

`grepq` will only match regex patterns to the sequence field of a FASTQ record, which is the most common use case. Unlike `ripgrep` and `grep`, which will match the regex patterns to the entire FASTQ record, which includes the record ID, sequence, separator, and quality fields. This can lead to false positives and slow down the filtering process.

**5. Output matched sequences to one of three formats**

- sequences only (default)
- sequences and their corresponding record IDs (`-I` option)
- FASTQ format (`-R` option)

**6. Will tune your pattern file with the `tune` command**

Use the `tune` command to analyze matched substrings and update the number and/or order of regex patterns in your pattern file according to their matched frequency. This can speed up the filtering process. 

Specifying the `-c` option to the `tune` command will output the matched substrings and their frequencies, ranked from highest to lowest.

When the patterns file is given in JSON format, then specifying the `-c`, `--names` and `--json-matches` options to the `tune` command will output the matched substrings and their frequencies in JSON format to a file called `matches.json`, allowing named regex sets and named regex patterns. See `examples/16S-iupac.json` for an example of a JSON pattern file and `examples/matches.json` for an example of the output of the `tune` command in JSON format.

>[!NOTE]
When the count option (-c) is given with the `tune` command, `grepq` will count the number of FASTQ records containing a sequence that is matched, for each matching regex in the pattern file. If, however, there are multiple occurrences of a given regex *within a FASTQ record sequence field*, `grepq` will count this as one match. When the count option (-c) is not given with the `tune` command, `grepq` provides the total number of matching FASTQ records for the set of regex patterns in the pattern file.

**7. Supports inverted matching with the `inverted` command**

Use the `inverted` command to output sequences that do not match any of the regex patterns in your pattern file.

**8. Plays nicely with your unix workflows**

For example, see `tune.sh` in the `examples` directory. This simple script will filter a FASTQ file using `grepq`, tune the pattern file on a user-specified number of FASTQ records, and then filter the FASTQ file again using the tuned pattern file for a user-specified number of the most frequent regex pattern matches.

## Usage 
Get instructions and examples using `grepq -h`, and `grepq tune -h` and `grepq inverted -h` for more information on the `tune` and `inverted` commands, respectively.

>[!NOTE]
Pattern files must contain one regex pattern per line or be provided in JSON format, and patterns are case-sensitive. You can supply an empty pattern file to count the total number of records in the FASTQ file. The regex patterns should only include the DNA sequence characters (A, C, G, T), or IUPAC ambiguity codes (N, R, Y, etc.). See `16S-no-iupac.txt`, `16S-iupac.json` and `16S-iupac-and-predicates.json` in the `examples` directory for examples of valid pattern files.

## Requirements

- `grepq` has been tested on Linux and macOS. It might work on Windows, but it has not been tested.
- Ensure that Rust is installed on your system (https://www.rust-lang.org/tools/install)
- If the build fails, make sure you have the latest version of the Rust compiler by running `rustup update`
- To use the `test.sh` script in the `examples` directory, you will need `yq` (v4.44.6 or later) installed on your system. To run "test-10" in `commands-1.yaml`, `commands-2.yaml`, `commands-3.yaml` and `commands-4.yaml`, you will need to download the file SRX26365298.fastq.gz from the SRA and place it in the `examples` directory. You can download the file with `fastq-dump --accession SRX26365298`. Obtain `fastq-dump` from the SRA Toolkit, available at NCBI.

## Installation
- From *crates.io* (easiest method, but will not install the `examples` directory)
    - `cargo install grepq`

- From *source* (will install the `examples` directory)
    - Clone the repository and `cd` into the `grepq` directory
    - Run `cargo build --release`
    - Relative to the cloned parent directory, the executable will be located in `./target/release`
    - Make sure the executable is in your `PATH` or use the full path to the executable

## Examples and tests
Get instructions and examples using `grepq -h`, `grepq tune -h` and `grepq inverted -h` for more information on the `tune` and `inverted` commands, respectively. See the `examples` directory for examples of pattern files and FASTQ files.

_File sizes of outfiles to verify `grepq` is working correctly, using the regex file `16S-no-iupac.txt` and the small fastq file `small.fastq`, both located in the `examples` directory:_

```bash
grepq ./examples/16S-no-iupac.txt ./examples/small.fastq > outfile.txt 
15953

grepq  ./examples/16S-no-iupac.txt ./examples/small.fastq inverted > outfile.txt
736547

grepq -I ./examples/16S-no-iupac.txt ./examples/small.fastq > outfile.txt
19515

grepq -I ./examples/16S-no-iupac.txt ./examples/small.fastq inverted > outfile.txt 
901271

grepq -R ./examples/16S-no-iupac.txt ./examples/small.fastq > outfile.txt
35574

grepq -R ./examples/16S-no-iupac.txt ./examples/small.fastq inverted > outfile.txt 
1642712
```

**Test script**

You may also run the test script (`test.sh`) in the `examples` directory to more fully test `grepq`. From the `examples directory`, run the following command:

```bash
./test.sh commands-1.yaml; ./test.sh commands-2.yaml; ./test.sh commands-3.yaml; ./test.sh commands-4.yaml
```

If all tests pass, there will be no orange (warning) text in the output, and no test will
report a failure.

*Example of failing test output:*

<span style="color: rgb(255, 165, 0);">
test-7 failed <br>
expected: 54 counts <br>
got: 53 counts <br>
command was: ../target/release/grepq -c 16S-no-iupac.txt small.fastq <br>
</span>
<br>


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
Executed in  218.80 millis    fish           external
   usr time  188.97 millis    0.09 millis  188.88 millis
   sys time   31.47 millis    4.98 millis   26.49 millis

```

## Futher testing

`grepq` can be tested using tools that generate synthetic FASTQ files, such as `spikeq` (https://github.com/Rbfinch/spikeq)

## Citation

If you use `grepq` in your research, please cite as follows:

Crosbie, N.D. (2024). grepq: A Rust application that quickly filters FASTQ files by matching sequences to a set of regex patterns. 10.5281/zenodo.14031703

## Update changes

see [CHANGELOG](https://github.com/Rbfinch/grepq/blob/main/CHANGELOG.md)

## License
MIT