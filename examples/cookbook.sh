#!/usr/bin/env bash

### cookbook.sh
# Author: Nicholas D. Crosbie
# Date: December 2024 
###

# This script provides examples of how to use the grepq tool to search FASTQ 
# files for sequences that match a regular expression pattern. Run from the
# /examples directory.

# To run all examples, make sure to download the file SRX26365298.fastq.gz from
# the SRA and place it in the `examples` directory. You can download the file 
# with `fastq-dump --accession SRX26365298`. Obtain `fastq-dump` from the 
# SRA Toolkit, available at NCBI.  =

# Exit immediately if a command exits with a non-zero status
set -e

echo -e "\nPrint only the matching sequences"
echo -e "\nCommand: grepq 16S-no-iupac.txt small.fastq | head -n 10 \n"
grepq 16S-no-iupac.txt small.fastq | head -n 10

echo -e "\nPrint the matching sequences with the record ID"
echo -e "\nCommand: grepq -I 16S-no-iupac.txt small.fastq | head -n 10 \n"
grepq -I 16S-no-iupac.txt small.fastq | head -n 10

echo -e "\nPrint the matching sequences in FASTQ format"
echo -e "\nCommand: grepq -R 16S-no-iupac.txt small.fastq | head -n 10 \n"
grepq -R 16S-no-iupac.txt small.fastq | head -n 10

echo -e "\nSave the matching sequences in gzip compressed FASTQ format"
echo -e "\nCommand: grepq -R -z 16S-no-iupac.txt small.fastq > output.fastq.gz \n"
grepq -R -z 16S-no-iupac.txt small.fastq > output.fastq.gz

echo -e "\nRead the FASTQ file in gzip compressed format and save only the
matching sequences"
echo -e "\nCommand: grepq -x 16S-no-iupac.txt SRX26365298.fastq.gz > output.txt \n"
grepq -x 16S-no-iupac.txt SRX26365298.fastq.gz > output.txt

echo -e "\nRead and save the output in gzip compressed format, with fast compression"
echo -e "\nCommand: grepq -xz --fast 16S-no-iupac.txt SRX26365298.fastq.gz > output.fastq.gz \n"
grepq -xz --fast 16S-no-iupac.txt SRX26365298.fastq.gz > output.fastq.gz

echo -e "\nRead and save the output in gzip compressed format, with best compression"
echo -e "\nCommand: grepq -xz --best 16S-no-iupac.txt SRX26365298.fastq.gz > output.fastq.gz \n"
grepq -xz --best 16S-no-iupac.txt SRX26365298.fastq.gz > output.fastq.gz

echo -e "\nCount the number of matching FASTQ records"
echo -e "\nCommand: grepq -c 16S-no-iupac.txt small.fastq \n"
grepq -c 16S-no-iupac.txt small.fastq

echo -e "\nFor each matched pattern in a search of the first 100000 records,
print the pattern and the number of matches"
echo -e "\nCommand: grepq 16S-no-iupac.txt small.fastq tune -n 100000 -c \n"
grepq 16S-no-iupac.txt small.fastq tune -n 100000 -c

echo -e "\nFor each matched pattern in a search of the first 100000 records of
a gzip-compressed FASTQ file, print the pattern and the number of matches"
echo -e "\nCommand: grepq -x 16S-no-iupac.txt SRX26365298.fastq.gz tune -n 100000 -c \n"
grepq -x 16S-no-iupac.txt SRX26365298.fastq.gz tune -n 100000 -c

echo -e "\nFor each matched pattern in a search of the first 100000 records of
 a gzip-compressed FASTQ file, print the pattern and the number of matches to a
JSON file called matches.json"
echo -e "\nCommand: grepq -x 16S-no-iupac.json SRX26365298.fastq.gz tune -n 100000 -c --names --json-matches \n"
grepq -x 16S-no-iupac.json SRX26365298.fastq.gz tune -n 100000 -c --names --json-matches

echo -e "\nSave the records where none of the regex patterns are found"
echo -e "\nCommand: grepq 16S-no-iupac.txt small.fastq inverted > output.txt \n"
grepq 16S-no-iupac.txt small.fastq inverted > output.txt

echo -e "\nSave the records where none of the regex patterns are found, with
the record ID"
echo -e "\nCommand: grepq -I 16S-no-iupac.txt small.fastq inverted > output.txt \n"
grepq -I 16S-no-iupac.txt small.fastq inverted > output.txt

echo -e "\nPrint the records where none of the regex patterns are found, in FASTQ format"
echo -e "\nCommand: grepq -R 16S-no-iupac.txt small.fastq inverted > output.txt \n"
grepq -R 16S-no-iupac.txt small.fastq inverted > output.txt

echo -e "\nCount the number of records where none of the regex patterns are found"
echo -e "\nCommand: grepq -c 16S-no-iupac.txt small.fastq inverted \n"
grepq -c 16S-no-iupac.txt small.fastq inverted

echo -e "\nCount the total number of records in the FASTQ file using an empty 
 pattern file"
echo -e "\nCommand: grepq -c empty.txt file.fastq inverted \n"
grepq -c empty.txt small.fastq inverted