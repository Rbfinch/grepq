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

# Summary

Regular expressions have been the mainstay of sequencing matching for many years [REF]. However, the performance of regular expressions can be slow, especially when matching large datasets. `grepq` is a Rust application that quickly filters FASTQ files by matching sequences to a set of regex patterns. `grepq` is designed to be fast and efficient, with a focus on performance and scalability. It is written in Rust, a systems programming language that is known for its speed and safety. `grepq` is designed to be easy to use, with a simple command-line interface that allows users to quickly filter large FASTQ files by matching sequences to a set of regex patterns, and to update the order in which patterns are matched against sequences through an in-built `tune` command. `grepq` is open-source and available on GitHub.

# Statement of need

The ability to quickly filter FASTQ files by matching sequences to a set of regex patterns is an important task in bioinformatics, especially when working with large datasets. The importance and challenge of this task will only grow as sequencing technologies continue to advance and produce larger and larger datasets. The uses cases are diverse, and include pre-processing of FASTQ files before downstream analysis, quality control of sequencing data, and filtering out unwanted sequences from a dataset. Where decisions need be made quickly, such as in a clinical settings, biosecurity, and wastewater-based epidemiology in support of public health measures [REFS], the ability to quickly filter FASTQ files by matching sequences to a set of regex patterns is attractive as it circumvents the need for more time-consuming bioinformatic workflows.

Regular expressions are a powerful tool for matching sequences, but they can be slow and inefficient when working with large datasets. Furthermore, general purpose tools like `grep` and `ripgrep` are not optimized for the specific task of filtering FASTQ files, and ocassionaly yield false positives as they scan the entire FASTQ record. Tools such `awk` and `sed` can be used to filter FASTQ files without yielding false positives, but they are not as efficient as `grepq` and require the development of more complex scripts to achieve the same result.

# Implementation

`grepq` is implemented in Rust, a systems programming language that is known for its speed and safety. Rust is a modern language that is designed to be fast and efficient, with a focus on performance and scalability. Rust is also known for its safety features, which help prevent common programming errors such as null pointer dereferences and buffer overflows. These features make Rust an ideal choice for implementing a tool like `grepq`, which needs to be fast, efficient, and reliable.

`grepq` obtains its performance by using the `seq_io` and `regex` crates (REFS). The `seq_io` crate is a well-tested Rust library for reading and writing FASTA and FASTQ files, and is designed to be fast and efficient. The `regex` crate is a Rust library for working with regular expressions and sets of regular expressions, and is known to be one of the fastest regular expression libraries currently available [REF]. The `regex` crate supports Perl-like regular expressions without look-around or backreferences (documented at <https://docs.rs/regex/1.*/regex/#syntax>).

By leveraging these libraries, `grepq` is able to quickly filter FASTQ files by matching sequences to a set of regex patterns, while maintaining high performance and reliability. Further performance is obtained by using multi-threading to process the records within an input FASTQ file concurently through use of multiple CPU cores; by using an optimised global memory allocator (the mimalloc crate [REF]) and reuse of buffers to reduce memory allocations and deallocations; use of byte slices to avoid the overhead of converting to and from string types; inlining of performance-critical functions; and use of `write_all` I/O operation that ensures the data is written in one go, rather than writing data in smaller chunks.

# Usage

The `grepq` command-line interface is designed to be simple and easy to use, with a focus on usability and efficiency. The `grepq` command-line interface allows users to quickly filter large FASTQ files by matching sequences to a set of regex patterns, with the ability to set predicates to filter on the header field, minimum sequence length, and minimum average quality score. The `grepq` command-line interface also allows users to output matched sequences to one of four formats (including FASTQ and FASTA), and to count and summarise the total number of records and the number of matching records in the input FASTQ file.

# Feature set

`grepq` has the following features:

- support for presence and absence (inverted) matching of a set of regular expressions
- IUPAC ambiguity code support (e.g. N, R, Y, etc.)
- gzip support (reading and writing)
- JSON support for pattern file input and `tune` command output, allowing named regex sets and named regex patterns (pattern files can also be in plain text)
- the ability to set predicates to filter on the header field (= record ID line) using a regular expression, minimum sequence length, and minimum average quality score (supports Phred+33 and Phred+64)
- the ability to output matched sequences to one of four formats (including FASTQ and FASTA)
- the ability to tune the pattern file with the `tune` command: this command will output a plain text or JSON file with the patterns sorted by their frequency of occurrence in the input FASTQ file (or a user-specified number of FASTQ records). This can be useful for optimizing the pattern file for performance.
- the ability to count and summarise the total number of records and the number of matching records (or records that don't match in the case of inverted matching) in the input FASTQ file

# Performance

The following shows the speedup of `grepq` over `grep`, `ripgrep` and `awk` for filtering FASTQ records against a set of regular expressions, for a 874MB FASTQ file (SRX26365298.fastq) containing 869034 records:

| tool    | time (s)  | &times; grep speedup | &times; ripgrep speedup | &times; awk speedup |
|---------|-----------|----------------------|-------------------------| ------------------- |
| grepq   |   0.20    | 1724.23              | 17.96                   | 822.23
| ripgrep |   3.58    |   95.99              |  1.00                   |  45.77
| grep    | 343.64    |    1.00              |  0.01                   |   0.48
| awk     | 163.87    |    2.10              |  0.02                   |   1.00
| gawk    | 285.62    |    1.20              |  0.01                   |   0.57

<details>
  <summary>Details</summary>
  <p>2022 model Mac Studio with 32GB RAM and Apple M1 max chip running macOS 15.0.1. The FASTQ file (SRX26365298.fastq) was 874MB in size and was stored on the internal SSD (APPLE SSD AP0512R). The pattern file contained 30 regex patterns (see `examples/16S-no-iupac.txt` for the patterns used). Under the same conditions and using the same pattern file, `grepq` processed a 104GB FASTQ file in 26 seconds (4GB/s) (`grepq` v1.3.5, `ripgrep` v14.1.1, `grep` 2.6.0-FreeBSD, `awk` v. 20200816, and `gawk` v.5.3.1. `ripgrep` was run with --colors 'match:none' --no-line-number, and `grep` was run with --color=never). Thw `awk` and `gawk` commands were run with a bash script, see `examples/match.sh`</p>
</details>

Speedup of `grepq` over `ripgrep` and `grep` for filtering FASTQ records against a set of regular expressions, where the FASTQ file was gzip-compressed:

| tool    | time (s) | &times; grep speedup | &times; ripgrep speedup |
|---------|----------|----------------------|-------------------------|
| grepq   |   2.38   | 145.24               | 1.51                    |
| ripgrep |   3.59   |  96.29               | 1.00                    |
| grep    | 345.68   |   1.00               | 0.01                    |

<details>
  <summary>Details</summary>
  <p>Conditions and versions as above, but the FASTQ file was gzip-compressed. `grepq` was run with the `-x` option, `ripgrep` with the `-z` option, and `grep` with the `-Z` option.</p>
</details>

# Testing

# Citations

Citations to entries in paper.bib should be in
[rMarkdown](http://rmarkdown.rstudio.com/authoring_bibliographies_and_citations.html)
format.

If you want to cite a software repository URL (e.g. something on GitHub without a preferred
citation) then you can do it with the example BibTeX entry below for @fidgit.

For a quick reference, the following citation commands can be used:

- `@author:2001`  ->  "Author et al. (2001)"
- `[@author:2001]` -> "(Author et al., 2001)"
- `[@author1:2001; @author2:2001]` -> "(Author1 et al., 2001; Author2 et al., 2002)"

# Acknowledgements

XXXXX

# References
