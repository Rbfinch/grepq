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

BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}\nSave only the matching sequences${NC}"
echo -e "${BLUE}\nCommand: grepq 16S-no-iupac.txt small.fastq | head -n 10 \n${NC}"
grepq 16S-no-iupac.txt small.fastq > out.txt
head -n 10 out.txt

echo -e "${BLUE}\nSave the matching sequences with the record ID${NC}"
echo -e "${BLUE}\nCommand: grepq -I 16S-no-iupac.txt small.fastq | head -n 10 \n${NC}"
grepq -I 16S-no-iupac.txt small.fastq > out.txt
head -n 10 out.txt

echo -e "${BLUE}\nSave the matching sequences in FASTQ format${NC}"
echo -e "${BLUE}\nCommand: grepq -R 16S-no-iupac.txt small.fastq | head -n 10 \n${NC}"
grepq -R 16S-no-iupac.txt small.fastq > out.txt
head -n 10 out.txt

echo -e "${BLUE}\nSave the matching sequences in gzip compressed FASTQ format${NC}"
echo -e "${BLUE}\nCommand: grepq -R -z 16S-no-iupac.txt small.fastq > out.fastq.gz \n${NC}"
grepq -R -z 16S-no-iupac.txt small.fastq > out.fastq.gz
gunzip -q out.fastq.gz
head -n 10 out.fastq

echo -e "${BLUE}\nRead the FASTQ file in gzip compressed format and save only the matching sequences${NC}"
echo -e "${BLUE}\nCommand: grepq -x 16S-no-iupac.txt SRX26365298.fastq.gz > out.txt \n${NC}"
grepq -x 16S-no-iupac.txt SRX26365298.fastq.gz > out.txt
head -n 10 out.txt

echo -e "${BLUE}\nRead and save the out in gzip compressed format, with fast compression${NC}"
echo -e "${BLUE}\nCommand: grepq -xz --fast 16S-no-iupac.txt SRX26365298.fastq.gz > out.fastq.gz \n${NC}"
grepq -xz --fast 16S-no-iupac.txt SRX26365298.fastq.gz > out.fastq.gz
gunzip -q out.fastq.gz
head -n 10 out.fastq

echo -e "${BLUE}\nRead and save the out in gzip compressed format, with best compression${NC}"
echo -e "${BLUE}\nCommand: grepq -xz --best 16S-no-iupac.txt SRX26365298.fastq.gz > out.fastq.gz \n${NC}"
grepq -xz --best 16S-no-iupac.txt SRX26365298.fastq.gz > out.fastq.gz
gunzip -q out.fastq.gz
head -n 10 out.fastq

echo -e "${BLUE}\nCount the number of matching FASTQ records${NC}"
echo -e "${BLUE}\nCommand: grepq -c 16S-no-iupac.txt small.fastq \n${NC}"
grepq -c 16S-no-iupac.txt small.fastq

echo -e "${BLUE}\nFor each matched pattern in a search of the first 100000 records, print the pattern and the number of matches${NC}"
echo -e "${BLUE}\nCommand: grepq 16S-no-iupac.txt small.fastq tune -n 100000 -c \n${NC}"
grepq 16S-no-iupac.txt small.fastq tune -n 100000 -c

echo -e "${BLUE}\nFor each matched pattern in a search of the first 100000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches${NC}"
echo -e "${BLUE}\nCommand: grepq -x 16S-no-iupac.txt SRX26365298.fastq.gz tune -n 100000 -c \n${NC}"
grepq -x 16S-no-iupac.txt SRX26365298.fastq.gz tune -n 100000 -c

echo -e "${BLUE}\nFor each matched pattern in a search of the first 100000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON file called matches.json${NC}"
echo -e "${BLUE}\nCommand: grepq -x 16S-no-iupac.json SRX26365298.fastq.gz tune -n 100000 -c --names --json-matches \n${NC}"
grepq -x 16S-no-iupac.json SRX26365298.fastq.gz tune -n 100000 -c --names --json-matches

echo -e "${BLUE}\nSave the records where none of the regex patterns are found${NC}"
echo -e "${BLUE}\nCommand: grepq 16S-no-iupac.txt small.fastq inverted > out.txt \n${NC}"
grepq 16S-no-iupac.txt small.fastq inverted > out.txt
head -n 10 out.txt

echo -e "${BLUE}\nSave the records where none of the regex patterns are found, with the record ID${NC}"
echo -e "${BLUE}\nCommand: grepq -I 16S-no-iupac.txt small.fastq inverted > out.txt \n${NC}"
grepq -I 16S-no-iupac.txt small.fastq inverted > out.txt
head -n 10 out.txt

echo -e "${BLUE}\nSave the records where none of the regex patterns are found, in FASTQ format${NC}"
echo -e "${BLUE}\nCommand: grepq -R 16S-no-iupac.txt small.fastq inverted > out.txt \n${NC}"
grepq -R 16S-no-iupac.txt small.fastq inverted > out.txt
head -n 10 out.txt

echo -e "${BLUE}\nCount the number of records where none of the regex patterns are found${NC}"
echo -e "${BLUE}\nCommand: grepq -c 16S-no-iupac.txt small.fastq inverted \n${NC}"
grepq -c 16S-no-iupac.txt small.fastq inverted

echo -e "${BLUE}\nCount the total number of records in the FASTQ file using an empty pattern file${NC}"
echo -e "${BLUE}\nCommand: grepq -c empty.txt file.fastq inverted \n${NC}"
grepq -c empty.txt small.fastq inverted