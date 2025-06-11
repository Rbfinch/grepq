---
title: >-
    grepq: A Rust application that quickly filters FASTQ files by matching sequences to a set of regular expressions
authors: 
  - name: Nicholas D. Crosbie
    email: nicholas.crosbie@unimelb.edu.au
    affiliation: "1"
    orcid: 0000-0002-0319-4248
    corresponding: true
affiliations:
  - index: 1
    name: Melbourne Veterinary School, University of Melbourne, Parkville, Victoria, Australia
date: "15 March 2025"
bibliography: paper.bib
tags: 
  - FASTQ records
  - regular expressions
  - Rust
  - bioinformatics
  - variants
---

# Summary

Regular expressions (regex) [@kleene1951representationof] have been an important tool for finding patterns in biological codes for decades [@hodgman2000historical and citations therein], and unlike fuzzy-finding approaches, do not result in approximate matches. The performance of regular expressions can be slow, however, especially when searching for matching patterns in large files. *grepq* is a Rust application that quickly filters FASTQ files by matching sequences to a set of regular expressions. *grepq* is designed with a focus on performance and scalability, is easy to install and easy to use, enabling users to quickly filter large FASTQ files, to enumerate named and unnamed variants, to update the order in which patterns are matched against sequences through built-in *tune* and *summarise* commands, and optionally, to output a SQLite file for further sequence analysis. *grepq* is open-source and available on *GitHub*, *Crates.io* and *bioconda*.

# Statement of need

The ability to quickly filter FASTQ files by matching sequences to a set of regular expressions is an important task in bioinformatics, especially when working with large datasets. The importance and challenge of this task will only grow as sequencing technologies continue to advance and produce ever larger datasets [@katz2022sequence]. The use cases of *grepq* are diverse, and include pre-processing of FASTQ files before downstream analysis, quality control of sequencing data, and filtering out unwanted sequences. Where decisions need to be made quickly, such as in clinical settings [@bachurin2024structural], biosecurity [@valdivia2012biodefense], and wastewater-based epidemiology in support of public health measures [@choi2018wastewater;@sims2020future;@xylogiannopoulos2021pattern;@merrett2024highly], the ability to quickly filter FASTQ files and enumerate named and unnamed variants by matching sequences to a set of regular expressions is attractive as it circumvents the need for more time-consuming bioinformatic workflows.

Regular expressions are a powerful tool for matching sequences, but they can be slow and inefficient when working with large datasets. Furthermore, general purpose tools like *grep* [@gnugrep] and *ripgrep* [@ripgrep] are not optimized for the specific task of filtering FASTQ files, and occassionally yield false positives as they scan the entire FASTQ record, including the sequence quality field. Tools such as *awk* [@awk] and *gawk* [@gawk]  can be used to filter FASTQ files without yielding false positives, but they are significantly slower than *grepq* and can require the development of more complex scripts to achieve the same result.

# Implementation

*grepq* obtains its performance and reliability, in part, by using the *seq_io* [@seq_io] and *regex* [@regex] libraries. The *seq_io* library is a well-tested library for parsing FASTQ files, designed to be fast and efficient, and includes a module for parallel processing of FASTQ records through multi-threading. The *regex* library is designed to work with regular expressions and sets of regular expressions, and is known to be one of the fastest regular expression libraries currently available [@rebar]. The *regex* library supports Perl-like regular expressions without look-around or backreferences (documented at <https://docs.rs/regex/1.*/regex/#syntax>).

# Feature set

- support for presence and absence (inverted) matching of a set of regular expressions
- IUPAC ambiguity code support (N, R, Y, etc.)
- support for gzip and zstd compression (reading and writing)
- JSON support for pattern file input and *tune* and *summarise* command output, allowing named regular expression sets and named regular expressions (pattern files can also be in plain text)
- the ability to:
  - set predicates to filter FASTQ records on the header field (= record ID line) using a regular expression, minimum sequence length, and minimum average quality score (supports Phred+33 and Phred+64)
  - output matched sequences in one of four formats (including FASTQ and FASTA)
  - tune the pattern file and enumerate named and unnamed variants with the *tune* and *summarise* commands: these commands will output a plain text or JSON file with the patterns sorted by their frequency of occurrence in the input FASTQ file or gzip-compressed FASTQ file (or a user-specified number of total matches). This can be useful for optimizing the pattern file for performance, for example by removing patterns that are rarely matched and reordering nucleotides within the variable regions of the patterns to improve matching efficiency
  - count and summarise the total number of records and the number of matching records (or records that don't match in the case of inverted matching) in the input FASTQ file
  - bucket matching sequences to separate files named after each regexName with the **--bucket** flag, in any of the four output formats

Other than when the **inverted** command is given, output to a SQLite database is supported with the **writeSQL** option. The SQLite database will contain a table called **fastq_data** with the following fields: the fastq record (header, sequence and quality fields), length of the sequence field (length), percent GC content (GC), percent GC content as an integer (GC_int), number of unique tetranucleotides in the sequence (nTN), number of unique canonical tetranucleotides in the sequence (nCTN), percent tetranucleotide frequency in the sequence (TNF), percent canonical tetranucleotide frequency in the sequence (CTNF), and a JSON array containing the matched regex patterns, the matches and their position(s) in the FASTQ sequence (variants). If the pattern file was given in JSON format and contained a non-null qualityEncoding field, then the average quality score for the sequence field (average_quality) will also be written. The **--num-tetranucleotides** option can be used to limit the number of tetranucleotides written to the TNF field of the fastq_data SQLite table, these being the most or equal most frequent tetranucleotides in the sequence field of the matched FASTQ records. A summary of the invoked query (pattern and data files) is written to a second table called **query**.

# Performance

The performance of *grepq* was compared to that of *fqgrep*, *seqkit* *grep*, *ripgrep*, *grep*, *awk*, and *gawk* using the benchmarking tool *hyperfine*. The test conditions and results are shown in **Table 1**, **Table 2** and **Table 3** (see [supplemental](https://github.com/Rbfinch/grepq/blob/main/paper/supplemental.pdf)).

# Testing and availability

*grepq* is open-source and available at *GitHub* (<https://github.com/Rbfinch/grepq>), *Crates.io* (<https://crates.io/crates/grepq>), and *bioconda* (<https://anaconda.org/bioconda/grepq>), and is distributed under the MIT license. It has been tested on macOS 15.0.1 (Apple M1 Max) and Linux Ubuntu 20.04.6 LTS (AMD EPYC 7763 64-Core Processor). For more information on the testing of *grepq*, see the *README.md* file in the *grepq* repository on *GitHub*.

# Conclusion

The performance of *grepq* was compared to that of *fqgrep*, *seqkit* *grep*, *ripgrep*, *grep*, *awk*, and *gawk* using the benchmarking tool *hyperfine*. For an uncompressed FASTQ file 874MB in size, containing 869,034 records, *grepq* was significantly faster than the other tools tested, with a speedup of 1797 times relative to *grep*, 864 times relative to *awk*, and 19 times relative to *ripgrep*. For a larger uncompressed FASTQ file (104GB in size, and containing 139,700,067 records), *grepq* was 4.4 times faster than *ripgrep* and marginally slower or of equivalent speed to *ripgrep* where the same large file was gzip-compressed. When coupled with its exceptional runtime performance, *grepq*'s feature set makes it a powerful and flexible tool for filtering large FASTQ files.

# Acknowledgements

I thank the authors of the *seq_io*, *regex*, *mimalloc* and *flate2* libraries and *hyperfine* benchmarking tool, and of the *ripgrep* and *fqgrep* tools for providing inspiration for *grepq*.

# Conflicts of interest

The author declares no conflicts of interest.

# References
