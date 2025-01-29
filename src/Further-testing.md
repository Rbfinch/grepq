## Further testing

`grepq` can be tested using tools that generate synthetic FASTQ files, such as `spikeq` (<https://crates.io/crates/spikeq>)

You can verify that `grepq` has found the regex patterns by using tools such as `grep` and `ripgrep`, using their ability to color-match the regex patterns (this feature is not available in `grepq` as that would make the code more complicated; code maintainability is an objective of this project). Recall, however, that `grep` and `ripgrep` will match the regex patterns to the entire FASTQ record, which includes the record ID, sequence, separator, and quality fields, occasionally leading to false positives.

