#!/bin/bash

# This two-line script shows an example of tuning the regular expression pattern file using the tune subcommand.
../target/release/grepq regex.txt small.fastq tune -n 50 | head -n 2 > tunedRegs.txt
../target/release/grepq tunedRegs.txt small.fastq > tuned-seqs.txt


