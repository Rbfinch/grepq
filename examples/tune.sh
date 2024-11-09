#!/bin/bash

# Filter sequences for the most frequent regular expression matches

# Filters a fastq file using `grepq`, tunes the pattern file on a user-specified number of fastq records, and then filters the fastq file again using the tuned pattern file for a user-specified number of the most frequent regex pattern matches.

if [ "$#" -ne 4 ]; then
    echo "Usage: $0 <regex_file> <fastq_file> <n_value> <head_value>"
    exit 1
fi

regex_file=$1
fastq_file=$2
n_value=$3
head_value=$4

grepq $regex_file $fastq_file tune -n $n_value | head -n $head_value > tunedRegs.txt
grepq tunedRegs.txt $fastq_file > tuned-seqs.txt


