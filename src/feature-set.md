## Feature set

> **Note:**
This README contains documentation for the latest version of `grepq`. If you are working through this documentation and the examples, please ensure that you are using the latest version. You can check the version by running `grepq -V`. For installation instructions, see the [Installation](#installation) section.

- very fast and scales to large FASTQ files
- IUPAC ambiguity code support
- support for gzip and zstd compression
- JSON support for pattern file input and `tune` command output, allowing named regex sets, named regex patterns, and named and unnamed variants
- use **predicates** to filter on the header field (= record ID line) using a regex, minimum sequence length, and minimum average quality score (supports Phred+33 and Phred+64)
- does not match false positives
- output matched sequences to one of four formats
- tune your pattern file and **enumerate named and unnamed variants** with the `tune` command
- supports inverted matching with the `inverted` command
- plays nicely with your unix workflows
- comprehensive help, examples and testing script
- read the preprint at **bioRxiv**: <https://doi.org/10.1101/2025.01.09.632104>

