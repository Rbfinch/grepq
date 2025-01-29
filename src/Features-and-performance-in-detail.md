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

> **Note:**
A regex supplied to filter on the header field (= record ID line) is first passed as a string to the regex engine, and then the regex engine is used to match the header field. Regex patterns to match the header field (= record ID line) must comply with the Rust regex library syntax (<https://docs.rs/regex/latest/regex/#syntax>). If you get an error message, be sure to escape any special characters in the regex pattern.

Predicates are specified in a JSON pattern file. For an example, see `16S-iupac-and-predicates.json` in the `examples` directory.

**4. Does not match false positives**

`grepq` will only match regex patterns to the sequence field of a FASTQ record, which is the most common use case. Unlike `ripgrep` and `grep`, which will match the regex patterns to the entire FASTQ record, which includes the record ID, sequence, separator, and quality fields. This can lead to false positives and slow down the filtering process.

**5. Output matched sequences to one of four formats**

- sequences only (default)
- sequences and their corresponding record IDs (`-I` option)
- FASTA format (`-F` option)
- FASTQ format (`-R` option)

> **Note:**
Other than when the `tune` command is run (see below), a FASTQ record is deemed to match (and hence provided in the output) when _any_ of the regex patterns in the pattern file match the sequence field of the FASTQ record.

**6. Tune your pattern file and enumerate named and unnamed variants with the `tune` command**

Use the `tune` command (`grepq tune -h` for instructions) in a simple shell script to update the number and order of regex patterns in your pattern file according to their matched frequency, further targeting and speeding up the filtering process.

Specifying the `-c` option to the `tune` command will output the matched substrings and their frequencies, ranked from highest to lowest.

When the patterns file is given in JSON format, then specifying the `-c`, `--names`, `--json-matches` and `--variants` options to the `tune` command will output the matched pattern variants and their corresponding counts in JSON format to a file called `matches.json`, allowing named regex sets, named regex patterns, and named and unnamed variants. See `examples/16S-iupac.json` for an example of a JSON pattern file and `examples/matches.json` for an example of the output of the `tune` command in JSON format.

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
                "regexCount": 287,
                "regexName": "Primer contig 06a",
                "regexString": "[AG]AAT[AT]G[AG]CGGGG",
                "variants": [
                    {
                        "count": 219,
                        "variant": "GAATTGACGGGG",
                        "variantName": "06a-v1"
                    },
                    {
                        "count": 43,
                        "variant": "AAATTGACGGGG",
                        "variantName": "06a-v2"
                    },
                    {
                        "count": 21,
                        "variant": "GAATTGGCGGGG",
                        "variantName": "06a-v3"
                    }
                ]
            },
            // matches for other regular expressions...
    ],
    "regexSetName": "conserved 16S rRNA regions"
  }
}
```

To output all variants of each pattern, use the `--all` argument, for example:

```bash
# For each matched pattern in a search of the first 20000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON file called matches.json, and include all variants of each pattern, and their respective counts. Note that the --variants argument is not given when --all is specified.

grepq --read-gzip 16S-no-iupac.json SRX26365298.fastq.gz tune -n 20000 -c --names --json-matches --all
```

> **Note:**
When the count option (-c) is given with the `tune` command, `grepq` will count the number of FASTQ records containing a sequence that is matched, for each matching regex in the pattern file. If, however, there are multiple occurrences of a given regex _within a FASTQ record sequence field_, `grepq` will count this as one match. When the count option (-c) is not given with the `tune` command, `grepq` provides the total number of matching FASTQ records for the set of regex patterns in the pattern file.

**7. Supports inverted matching with the `inverted` command**

Use the `inverted` command to output sequences that do not match any of the regex patterns in your pattern file.

**8. Plays nicely with your unix workflows**

For example, see `tune.sh` in the `examples` directory. This simple script will filter a FASTQ file using `grepq`, tune the pattern file on a user-specified number of FASTQ records, and then filter the FASTQ file again using the tuned pattern file for a user-specified number of the most frequent regex pattern matches. Use a tool like `jq` to conveniently parse the JSON output of the `tune` command.

