#!/usr/bin/env bash

### cookbook.sh
# Author: Nicholas D. Crosbie
# Date: January 2025 
###

# This script provides examples of how to use the grepq tool to search FASTQ 
# files for sequences that match a regular expression pattern. Run from the
# /examples directory.

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
echo -e "${BLUE}\nCommand: grepq -R --write-gzip 16S-no-iupac.txt small.fastq > out.fastq.gz \n${NC}"
grepq -R --write-gzip 16S-no-iupac.txt small.fastq > out.fastq.gz
gunzip -q out.fastq.gz
head -n 10 out.fastq

echo -e "${BLUE}\nRead the FASTQ file in gzip compressed format and save only the matching sequences${NC}"
echo -e "${BLUE}\nCommand: grepq --read-gzip 16S-no-iupac.txt small-copy.fastq.gz > out.txt \n${NC}"
grepq --read-gzip 16S-no-iupac.txt small-copy.fastq.gz > out.txt
head -n 10 out.txt

echo -e "${BLUE}\nRead and save the output in gzip compressed format, with fast compression${NC}"
echo -e "${BLUE}\nCommand: grepq --read-gzip --write-gzip --fast 16S-no-iupac.txt small-copy.fastq.gz > out.fastq.gz \n${NC}"
grepq --read-gzip --write-gzip --fast 16S-no-iupac.txt small-copy.fastq.gz > out.fastq.gz
gunzip -q out.fastq.gz
head -n 10 out.fastq

echo -e "${BLUE}\nRead and save the output in gzip compressed format, with best compression${NC}"
echo -e "${BLUE}\nCommand: grepq --read-gzip --write-gzip --best 16S-no-iupac.txt small-copy.fastq.gz > out.fastq.gz \n${NC}"
grepq --read-gzip --write-gzip --best 16S-no-iupac.txt small-copy.fastq.gz > out.fastq.gz
gunzip -q out.fastq.gz
head -n 10 out.fastq

echo -e "${BLUE}\nRead and save the output in zstd compressed format, with fast compression${NC}"
echo -e "${BLUE}\nCommand: grepq --read-zstd --write-zstd --fast 16S-no-iupac.txt small-copy.fastq.zst > out.fastq.zst \n${NC}"
grepq --read-zstd --write-zstd --fast 16S-no-iupac.txt small-copy.fastq.zst > out.fastq.zst
zstd -d out.fastq.zst -o out.fastq
head -n 10 out.fastq

echo -e "${BLUE}\nRead and save the output in zstd compressed format, with best compression${NC}"
echo -e "${BLUE}\nCommand: grepq --read-zstd --write-zstd --best 16S-no-iupac.txt small-copy.fastq.zst > out.fastq.zst \n${NC}"
grepq --read-zstd --write-zstd --best 16S-no-iupac.txt small-copy.fastq.zst > out.fastq.zst
zstd -d out.fastq.zst -o out.fastq
head -n 10 out.fastq

echo -e "${BLUE}\nCount the number of matching FASTQ records${NC}"
echo -e "${BLUE}\nCommand: grepq -c 16S-no-iupac.txt small.fastq \n${NC}"
grepq -c 16S-no-iupac.txt small.fastq

echo -e "${BLUE}\nFor each matched pattern in a search of the first 2000 records, print the pattern and the number of matches${NC}"
echo -e "${BLUE}\nCommand: grepq 16S-no-iupac.txt small.fastq tune -n 2000 -c \n${NC}"
grepq 16S-no-iupac.txt small.fastq tune -n 2000 -c

echo -e "${BLUE}\nFor each matched pattern in a search of the first 2000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches${NC}"
echo -e "${BLUE}\nCommand: grepq --read-gzip 16S-no-iupac.txt small-copy.fastq.gz tune -n 2000 -c \n${NC}"
grepq --read-gzip 16S-no-iupac.txt small-copy.fastq.gz tune -n 2000 -c

echo -e "${BLUE}\nFor each matched pattern in a search of the first 2000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON file called matches.json${NC}"
echo -e "${BLUE}\nCommand: grepq --read-gzip 16S-no-iupac.json small-copy.fastq.gz tune -n 2000 -c --names --json-matches \n${NC}"
grepq --read-gzip 16S-no-iupac.json small-copy.fastq.gz tune -n 2000 -c --names --json-matches

echo -e "${BLUE}\nFor each matched pattern in a search of the first 2000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON file called matches.json, and include the top three most frequent
variants of each pattern, and their respective counts${NC}"
echo -e "${BLUE}\nCommand: grepq --read-gzip 16S-no-iupac.json small-copy.fastq.gz tune -n 2000 -c --names --json-matches --variants 3 \n${NC}"
grepq --read-gzip 16S-no-iupac.json small-copy.fastq.gz tune -n 2000 -c --names --json-matches --variants 3

echo -e "${BLUE}\nFor each matched pattern in a search of the first 2000 records of a gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON file called matches.json, and include all variants of each pattern, and their respective counts. Note that the `--variants` argument is not given when `--all`
is specified.${NC}"
echo -e "${BLUE}\nCommand: grepq --read-gzip 16S-no-iupac.json small-copy.fastq.gz tune -n 2000 -c --names --json-matches --all \n${NC}"
grepq --read-gzip 16S-no-iupac.json small-copy.fastq.gz tune -n 2000 -c --names --json-matches --all

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