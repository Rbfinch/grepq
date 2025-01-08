---
title: 'grepq: A Rust application that quickly filters FASTQ files by matching sequences to a set of regex patterns.'
tags:
  - Rust
  - bioinformatics
  - regular expressions
  - FASTQ
 
authors:
  - name: Nicholas D. Crosbie 
    orcid: 0000-0002-0319-4248
affiliations:
 - name: University of Melbourne, Australia
   index: 1
date: 7 January 2025
bibliography: paper.bib

---
# grepq: A Rust application that quickly filters FASTQ files by matching sequences to a set of regex patterns

# Summary

Regular expressions [@kleene1951representationof] have been an important tool for finding patterns in biological codes for decades [@hodgman2000historical and citations therein]. However, the performance of regular expressions can be slow, especially when matching large datasets. **grepq** is a Rust application that quickly filters FASTQ files by matching sequences to a set of regex patterns. **grepq** is designed to be fast and efficient, with a focus on performance and scalability. It is written in Rust, a systems programming language that is known for its speed and safety. **grepq** is designed to be easy to use, with a simple command-line interface that allows users to quickly filter large FASTQ files, and to update the order in which patterns are matched against sequences through an in-built `tune` command. **grepq** is open-source and available on GitHub.

# Statement of need

The ability to quickly filter FASTQ files by matching sequences to a set of regex patterns is an important task in bioinformatics, especially when working with large datasets. The importance and challenge of this task will only grow as sequencing technologies continue to advance and produce larger and larger datasets. The uses cases are diverse, and include pre-processing of FASTQ files before downstream analysis, quality control of sequencing data, and filtering out unwanted sequences from a dataset. Where decisions need be made quickly, such as in a clinical settings [@bachurin2024structural], biosecurity [@valdivia2012biodefense], and wastewater-based epidemiology in support of public health measures [@choi2018wastewater;@sims2020future;@xylogiannopoulos2021pattern], the ability to quickly filter FASTQ files by matching sequences to a set of regex patterns is attractive as it circumvents the need for more time-consuming bioinformatic workflows.

Regular expressions are a powerful tool for matching sequences, but they can be slow and inefficient when working with large datasets. Furthermore, general purpose tools like **grep** [@gnugrep] and **ripgrep** [@ripgrep] are not optimized for the specific task of filtering FASTQ files, and ocassionaly yield false positives as they scan the entire FASTQ record. Tools such **awk** [@awk] and **gawk** [@gawk]  can be used to filter FASTQ files without yielding false positives, but they are significantly slower than **grepq** and can require the development of more complex scripts to achieve the same result.

# Implementation

**grepq** is implemented in Rust, a systems programming language that is known for its speed and safety. Rust is a modern language that is designed to be fast and efficient, with a focus on performance and scalability. Rust is also known for its safety features, which help prevent common programming errors such as null pointer dereferences and buffer overflows. These features make Rust an ideal choice for implementing a tool like **grepq**, which needs to be fast, efficient, and reliable.

**grepq** obtains its performance and reliability, in part, by using the **seq_io** [@seq_io] and **regex** [@regex] Rust libraries. The **seq_io** library is a well-tested library for reading and writing FASTA and FASTQ files, designed to be fast and efficient, and which includes a module for parallel processing through multi-threading. The **regex** library is designed to work with regular expressions and sets of regular expressions, and is known to be one of the fastest regular expression libraries currently available [@rebar]. The **regex** library supports Perl-like regular expressions without look-around or backreferences (documented at <https://docs.rs/regex/1.*/regex/#syntax>).

By leveraging these libraries, **grepq** is able to quickly filter FASTQ files by matching sequences to a set of regex patterns, while maintaining high performance and reliability.

Further performance gains were obtained by:

- use of the `RegexSet` struct from the **regex** library to match multiple regular expressions against a sequence in a single pass, rather than matching each regular expression individually (the `RegexSet` is created and compiled once before entering any loop that processes the FASTQ records, avoiding the overhead of recompiling the regular expressions for each record)
- multi-threading to process the records within an input FASTQ file in parallel through use of multiple CPU cores
- use of an optimised global memory allocator (the **mimalloc** library [@mimalloc]) to reduce memory fragmentation and improve memory allocation and deallocation performance
- buffer reuse to reduce memory allocations and deallocations
- use of byte slices to avoid the overhead of converting to and from string types
- inlining of performance-critical functions
- use of the `write_all` I/O operation that ensures the data is written in one go, rather than writing data in smaller chunks

# Feature set

**grepq** has the following features:

- support for presence and absence (inverted) matching of a set of regular expressions
- IUPAC ambiguity code support (e.g. N, R, Y, etc.)
- gzip support (reading and writing)
- JSON support for pattern file input and `tune` command output, allowing named regex sets and named regex patterns (pattern files can also be in plain text)
- the ability to set predicates to filter FASTQ records on the header field (= record ID line) using a regular expression, minimum sequence length, and minimum average quality score (supports Phred+33 and Phred+64)
- the ability to output matched sequences to one of four formats (including FASTQ and FASTA)
- the ability to tune the pattern file with the `tune` command: this command will output a plain text or JSON file with the patterns sorted by their frequency of occurrence in the input FASTQ file or gzip-compressed FASTQ file (or a user-specified number of FASTQ records). This can be useful for optimizing the pattern file for performance, for example, by removing patterns that are rarely matched
- the ability to count and summarise the total number of records and the number of matching records (or records that don't match in the case of inverted matching) in the input FASTQ file

When coupled with its excellent runtime performance (see below), this feature set makes **grepq** a powerful and flexible tool for filtering FASTQ files. Colorized output of for matching regex patterns is not implemented to maximise speed and minimise code complexity, but can be achieved by piping the output to **grep** or **ripgrep** for testing purposes.

# Performance

The benchmarking tool **hyperfine** [@Peter_hyperfine_2023] was used to compare the runtime of **grepq**, **fqgrep**, **ripgrep**, **fqgrep**, **awk**, and **gawk** for filtering FASTQ records against a set of regular expressions. The benchmarking was performed on a 2022 model Mac Studio with 32GB RAM and Apple M1 max chip running macOS 15.0.1. The FASTQ file (SRX26365298.fastq) was 874MB in size and was stored on the internal SSD (APPLE SSD AP0512R). The pattern file contained 30 regex patterns (see `examples/16S-no-iupac.txt` for the patterns used). The **awk** and **gawk** commands were run with a bash script, see `examples/match.sh`.

The following shows the speedup of **grepq** for filtering FASTQ records against a set of regular expressions, for a 874MB FASTQ file (SRX26365298.fastq) containing 869034 records:

| tool    | time (s)  | &times; grep speedup | &times; ripgrep speedup | &times; awk speedup |
|---------|-----------|----------------------|-------------------------| ------------------- |
| grepq   |           |                      |                         |
| fqgrep  |           |                      |                         |
| ripgrep |           |   95.99              |  1.00                   |  45.77
| grep    | 343.64    |    1.00              |  0.01                   |   0.48
| awk     | 163.87    |    2.10              |  0.02                   |   1.00
| gawk    | 285.62    |    1.20              |  0.01                   |   0.57

(**grepq** v1.3.5, **fqgrep** v.1.02, **ripgrep** v14.1.1, **grep** 2.6.0-FreeBSD, **awk** v. 20200816, and **gawk** v.5.3.1. **ripgrep** was run with --colors 'match:none' --no-line-number, and **grep** was run with --color=never)

Speedup of **grepq** over **ripgrep** and **grep** for filtering FASTQ records against a set of regular expressions, where the FASTQ file was gzip-compressed:

| tool    | time (s) | &times; grep speedup | &times; ripgrep speedup |
|---------|----------|----------------------|-------------------------|
| grepq   |   2.38   | 145.24               | 1.51                    |
| fqgrep  |   2.38   | 145.24               | 1.51                    |
| ripgrep |   3.59   |  96.29               | 1.00                    |

<details>
  <summary>Details</summary>
  <p>Conditions and versions as above, but the FASTQ file was gzip-compressed. **grepq** was run with the `-x` option, **ripgrep** with the `-z` option, and **grep** with the `-Z` option.</p>
</details>

# Testing

The output of **grepq** was compared against the output of output of **fqgrep**, **ripgrep**, **grep**, **awk** and **gawk**, using the **stat** command [@stat], and any difference investigated using the **diff** command [@diff]. Furthermore, a custom utility, **spikeq** [@spikeq] was developed to generate synthetic FASTQ files with a known number of records and sequences that match a set of regular expressions. This utility was used to test the performance of **grepq** and the aforementioned other tools under controlled conditions.

Finally, a bash test script (`examples/test.sh`) and a simple Rust CLI application, **predate** [@predate], were developed and utilised to automate system testing, and to monitor for  performance regressions.

**grepq** has been tested on macOS 15.0.1 (Apple M1 Max) and Linux Ubuntu 20.04.6 LTS (AMD EPYC 7763 64-Core Processor). It may work on other platforms, but this has not been tested.

# Availability and documentation

**grepq** is open-source and available on GitHub at:
<https://github.com/Rbfinch/grepq>

Documentation for **grepq** is available at the same GitHub repository, and through the `-h` and `--help` command-line options, which includes a list of all available commands and options, and examples of how to use them, as well as a large number of examples in the `examples` directory. Example pattern files in plain text and JSON format are also provided. **grepq** is distributed under the MIT license.

# Acknowledgements

I'm grateful to my family for their patience and support during the development of **grepq**. I would also like to thank the developers of the **seq_io** and **regex** Rust libraries for their excellent work, and the developers of the `hyperfine` benchmarking tool for making it easy to compare the performance of different tools.

# References
