<img src="src/grepq-icon.svg" width="128" />

_Quickly filter FASTQ files_

[![Crates.io](https://img.shields.io/crates/v/grepq.svg)](https://crates.io/crates/grepq)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**Table of Contents**

- [Feature set](#feature-set)
- [Features and performance in detail](#features-and-performance-in-detail)
- [Usage](#usage)
- [Cookbook](https://github.com/Rbfinch/grepq/blob/main/cookbook.md)
- [Requirements](#requirements)
- [Installation](#installation)
- [Examples and tests](#examples-and-tests)
- [Further testing](#futher-testing)
- [Citation and preprint](#citation)
- [Update changes](#update-changes)
- [License](#license)

## Feature set

- very fast and scales to large FASTQ files
- IUPAC ambiguity code support
- support for gzip and zstd compression
- JSON support for pattern file input and `tune` command output, allowing named regex sets and named regex patterns
- use **predicates** to filter on the header field (= record ID line) using a regex, minimum sequence length, and minimum average quality score (supports Phred+33 and Phred+64)
- does not match false positives
- output matched sequences to one of four formats
- tune your pattern file and **enumerate variants** with the `tune` command
- supports inverted matching with the `inverted` command
- plays nicely with your unix workflows
- comprehensive help, examples and testing script
- read the preprint at **bioRxiv**: <https://doi.org/10.1101/2025.01.09.632104>

## Features and performance in detail

**1. Very fast and scales to large FASTQ files**

| tool          | mean wall time (s) | S.D. wall time (s) | speedup (× grep) | speedup (× ripgrep) | speedup (× awk) |
|---------------|--------------------|--------------------|------------------|---------------------|-----------------|
| _grepq_       | 0.19               | 0.01               | 1796.76          | 18.62               | 863.52          |
| _fqgrep_      | 0.34               | 0.01               | 1017.61          | 10.55               | 489.07          |
| _ripgrep_     | 3.57               | 0.01               | 96.49            | 1.00                | 46.37           |
| _seqkit grep_ | 2.89               | 0.01               | 119.33           | 1.24                | 57.35           |
| _grep_        | 344.26             | 0.55               | 1.00             | 0.01                | 0.48            |
| _awk_         | 165.45             | 1.59               | 2.08             | 0.02                | 1.00            |
| _gawk_        | 287.66             | 1.68               | 1.20             | 0.01                | 0.58            |

<details>
  <summary>Details</summary>
  <p>2022 model Mac Studio with 32GB RAM and Apple M1 max chip running macOS 15.0.1. The FASTQ file (SRX26365298.fastq) was 874MB in size and was stored on the internal SSD (APPLE SSD AP0512R). The pattern file contained 30 regex patterns (see `examples/16S-no-iupac.txt` for the patterns used). grepq v1.4.0, fqgrep v.1.02, ripgrep v14.1.1, seqkit grep v.2.9.0, grep 2.6.0-FreeBSD, awk v. 20200816, and gawk v.5.3.1. fqgrep and seqkit grep were run with default settings, ripgrep was run with -B 1 -A 2 --colors 'match:none' --no-line-number, and grep -B 1 -A 2 was run with --color=never. The tools were configured to output matching records in FASTQ format. The wall times, given in seconds, are the mean of 10 runs, and S.D. is the standard deviation of the wall times, also given in seconds.</p>
</details>

**2. Reads and writes regular or gzip or zstd-compressed FASTQ files**

Use the `--best` option for best compression, or the `--fast` option for faster compression.

| tool      | mean wall time (s) | S.D. wall time (s) | speedup (× ripgrep) |
|-----------|--------------------|--------------------|---------------------|
| _grepq_   | 1.71               | 0.00               | 2.10                |
| _fqgrep_  | 1.83               | 0.01               | 1.95                |
| _ripgrep_ | 3.58               | 0.01               | 1.00                |

<details>
  <summary>Details</summary>
  <p>Conditions and versions as above, but the FASTQ file was gzip-compressed. `grepq` was run with the `--read-gzip` option, `ripgrep` with the `-z` option, and `grep` with the `-Z` option. The wall times, given in seconds, are the mean of 10 runs, and S.D. is the standard deviation of the wall times, also given in seconds.</p>
</details>

**3. Predicates**

Predicates can be used to filter on the header field (= record ID line) using a regex, minimum sequence length, and minimum average quality score (supports Phred+33 and Phred+64).

>[!NOTE]
A regex supplied to filter on the header field (= record ID line) is first passed as a string to the regex engine, and then the regex engine is used to match the header field. Regex patterns to match the header field (= record ID line) must comply with the Rust regex library syntax (<https://docs.rs/regex/latest/regex/#syntax>). If you get an error message, be sure to escape any special characters in the regex pattern.

Predicates are specified in a JSON pattern file. For an example, see `16S-iupac-and-predicates.json` in the `examples` directory.

**4. Does not match false positives**

`grepq` will only match regex patterns to the sequence field of a FASTQ record, which is the most common use case. Unlike `ripgrep` and `grep`, which will match the regex patterns to the entire FASTQ record, which includes the record ID, sequence, separator, and quality fields. This can lead to false positives and slow down the filtering process.

**5. Output matched sequences to one of four formats**

- sequences only (default)
- sequences and their corresponding record IDs (`-I` option)
- FASTA format (`-F` option)
- FASTQ format (`-R` option)

>[!NOTE]
Other than when the `tune` command is run (see below), a FASTQ record is deemed to match (and hence provided in the output) when _any_ of the regex patterns in the pattern file match the sequence field of the FASTQ record.

**6. Tune your pattern file and enumerate variants with the `tune` command**

Use the `tune` command (`grepq tune -h` for instructions) in a simple shell script to update the number and order of regex patterns in your pattern file according to their matched frequency, further targeting and speeding up the filtering process.

Specifying the `-c` option to the `tune` command will output the matched substrings and their frequencies, ranked from highest to lowest.

When the patterns file is given in JSON format, then specifying the `-c`, `--names`, `--json-matches` and `--variants` options to the `tune` command will output the matched pattern variants and their corresponding counts in JSON format to a file called `matches.json`, allowing named regex sets and named regex patterns. See `examples/16S-iupac.json` for an example of a JSON pattern file and `examples/matches.json` for an example of the output of the `tune` command in JSON format.

```bash
# For each matched pattern in a search of the first 20000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON file called matches.json, and include the top three most frequent variants of each pattern, and their respective counts

grepq --read-gzip 16S-no-iupac.json SRX26365298.fastq.gz tune -n 20000 -c --names --json-matches --variants 3
```

Abridged output (see `examples/matches.json` for the full output):

```json
{
  "regexSet": {
    "regex": [
{
                "mostFrequentVariants": [
                    {
                        "count": 219,
                        "variant": "GAATTGACGGGG"
                    },
                    {
                        "count": 43,
                        "variant": "AAATTGACGGGG"
                    },
                    {
                        "count": 21,
                        "variant": "GAATTGGCGGGG"
                    }
                ],
                "regexCount": 287,
                "regexName": "Primer contig 06a",
                "regexString": "[AG]AAT[AT]G[AG]CGGGG"
            },
            {
                "mostFrequentVariants": [
                    {
                        "count": 221,
                        "variant": "CCCCGTCAATTC"
                    },
                    {
                        "count": 43,
                        "variant": "CCCCGTCAATTT"
                    },
                    {
                        "count": 25,
                        "variant": "CCCCGCCAATTC"
                    }
                ],
                "regexCount": 298,
                "regexName": "Primer contig 06aR",
                "regexString": "CCCCG[CT]C[AT]ATT[CT]"
            }
    ],
    "regexSetName": "conserved 16S rRNA regions"
  }
}
```

>[!NOTE]
When the count option (-c) is given with the `tune` command, `grepq` will count the number of FASTQ records containing a sequence that is matched, for each matching regex in the pattern file. If, however, there are multiple occurrences of a given regex _within a FASTQ record sequence field_, `grepq` will count this as one match. When the count option (-c) is not given with the `tune` command, `grepq` provides the total number of matching FASTQ records for the set of regex patterns in the pattern file.

**7. Supports inverted matching with the `inverted` command**

Use the `inverted` command to output sequences that do not match any of the regex patterns in your pattern file.

**8. Plays nicely with your unix workflows**

For example, see `tune.sh` in the `examples` directory. This simple script will filter a FASTQ file using `grepq`, tune the pattern file on a user-specified number of FASTQ records, and then filter the FASTQ file again using the tuned pattern file for a user-specified number of the most frequent regex pattern matches.

## Usage

Get instructions and examples using `grepq -h`, and `grepq tune -h` and `grepq inverted -h` for more information on the `tune` and `inverted` commands, respectively.

>[!NOTE]
`grepq` can output to several formats, including those that are gzip or zstd compressed. `grepq`, however, will only accept a FASTQ file or a compressed (gzip or zstd) FASTQ file as the sequence data file. If you get an error message, check that the input data file is a FASTQ file or a gzip or zstd compressed FASTQ file, and that you have specified the correct file format (--read-gzip or --read-zstd for FASTQ files compressed by gzip and zstd, respectively), and file path. Pattern files must contain one regex pattern per line or be provided in JSON format, and patterns are case-sensitive. You can supply an empty pattern file to count the total number of records in the FASTQ file. The regex patterns for matching FASTQ sequences should only include the DNA sequence characters (A, C, G, T), or IUPAC ambiguity codes (N, R, Y, etc.). See `16S-no-iupac.txt`, `16S-iupac.json` and `16S-iupac-and-predicates.json` in the `examples` directory for examples of valid pattern files. Regex patterns to match the header field (= record ID line) must comply with the Rust regex library syntax (<https://docs.rs/regex/latest/regex/#syntax>). If you get an error message, be sure to escape any special characters in the regex pattern.

## Requirements

- `grepq` has been tested on Linux and macOS. It might work on Windows WSL, but it has not been tested.
- Ensure that Rust is installed on your system (<https://www.rust-lang.org/tools/install>)
- If the build fails, make sure you have the latest version of the Rust compiler by running `rustup update`
- To run the `test.sh` and `cookbook.sh` scripts in the `examples` directory, you will need `yq` (v4.44.6 or later), `gunzip` and version 4 or later of `bash`.

## Installation

- From _crates.io_ (easiest method, but will not install the `examples` directory)
  - `cargo install grepq`

- From _source_ (will install the `examples` directory)
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

For the curious-minded, note that the regex patterns in `16S-no-iupac.txt`, `16S-iupac.json`, and `16S-iupac-and-predicates.json` are from Table 3 of Martinez-Porchas, Marcel, et al. "How conserved are the conserved 16S-rRNA regions?." PeerJ 5 (2017): e3036.

For more examples, see the `examples` directory and the [cookbook](https://github.com/Rbfinch/grepq/blob/main/cookbook.md), available also as a shell script in the `examples` directory.

**Test script**

You may also run the test script (`test.sh`) in the `examples` directory to more fully test `grepq`. From the `examples directory`, run the following command:

```bash
./test.sh commands-1.yaml; ./test.sh commands-2.yaml; ./test.sh commands-3.yaml; ./test.sh commands-4.yaml
```

If all tests pass, there will be no orange (warning) text in the output, and no test will
report a failure. A summary of the number of passing and failing tests will be displayed at the end of the output. All tests should pass.

_Example of failing test output:_

<span style="color: rgb(255, 165, 0);">
test-7 failed <br>
expected: 54 counts <br>
got: 53 counts <br>
command was: ../target/release/grepq -c 16S-no-iupac.txt small.fastq <br>
</span>
<br>

Further, you can run the `cookbook.sh` script in the `examples` directory to test the cookbook examples, and you can use `predate` (<https://crates.io/crates/predate>) if you prefer a Rust application to a shell script.

**SARS-CoV-2 example**

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

Obtain `SRX26602697.fastq` from the SRA using `fastq-dump --accession SRX26602697`.

## Further testing

`grepq` can be tested using tools that generate synthetic FASTQ files, such as `spikeq` (<https://crates.io/crates/spikeq>)

You can verify that `grepq` has found the regex patterns by using tools such as `grep` and `ripgrep`, using their ability to color-match the regex patterns (this feature is not available in `grepq` as that would make the code more complicated; code maintainability is an objective of this project). Recall, however, that `grep` and `ripgrep` will match the regex patterns to the entire FASTQ record, which includes the record ID, sequence, separator, and quality fields, occasionally leading to false positives.

## Citation and preprint

If you use `grepq` in your research, please cite as follows:

Crosbie, N.D. (2025). grepq: A Rust application that quickly filters FASTQ files by matching sequences to a set of regular expressions. **bioRxiv**, doi: <https://doi.org/10.1101/2025.01.09.632104>

## Update changes

see [CHANGELOG](https://github.com/Rbfinch/grepq/blob/main/CHANGELOG.md)

## License

MIT
