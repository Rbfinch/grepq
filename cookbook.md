### grepq cookbook

*Print the help message (including **tips** and **notes**)*

**grepq -h**

*and*

**grepq --help**

<br>

*Print the help message for the **tune** command*

**grepq tune -h**

<br>

*Print the help message for the **summarise** command*

**grepq summarise -h**

<br>

*Print the help message for the **inverted** command*

**grepq inverted -h**

<br>

*Print only the matching sequences*

**grepq regex.txt file.fastq**

<br>

*Print the matching sequences with the record ID*

**grepq -I regex.txt file.fastq**

<br>

*Print the matching sequences in FASTQ format*

**grepq -R regex.txt file.fastq**

<br>

*Save the matching sequences in gzip compressed FASTQ format*

**grepq -R --write-gzip regex.txt file.fastq > output.fastq.gz**

<br>

*Read the FASTQ file in gzip compressed format*

**grepq --read-gzip regex.txt file.fastq.gz**

<br>

*Read and save the output in gzip compressed format, with fast
compression*

**grepq --read-gzip --write-gzip --fast regex.txt file.fastq.gz > output.fastq.gz**

<br>

*Read and save the output in gzip compressed format, with best
compression*

**grepq --read-gzip --write-gzip --best regex.txt file.fastq.gz > output.fastq.gz**

<br>

*Read and save the output in zstd compressed format, with best
compression*

**grepq --read-zstd --write-zstd --best regex.txt file.fastq.zst > output.fastq.zst**

<br>

*Read and save the output in zstd compressed format, with fast
compression*

**grepq --read-zstd --write-zstd --fast regex.txt file.fastq.zst > output.fastq.zst**

<br>

*Count the number of matching FASTQ records*

**grepq -c regex.txt file.fastq**

<br>

*For each matched pattern in a search of no more than 100000 matches,
print the pattern and the number of matches*

**grepq regex.txt file.fastq tune -n 100000 -c**

<br>

*For each matched pattern in a search of no more than 100000 matches of
a gzip-compressed FASTQ file, print the pattern and the number of matches*

**grepq --read-gzip regex.txt file.fastq.gz tune -n 100000 -c**

<br>

*For each matched pattern in a search of no more than 100000 matches of
a gzip-compressed FASTQ file, print the pattern and the number of matches to a
JSON file called matches.json*

**grepq --read-gzip regex.json file.fastq.gz tune -n 100000 -c --names --json-matches**

<br>

*As above, but uses the summarise command to ensure that all FASTQ records are
processed*

**grepq --read-gzip regex.json file.fastq.gz summarise -c --names --json-matches**

<br>

*For each matched pattern in a search of no more than 100000 matches of a
gzip-compressed FASTQ file, print the pattern and the number of matches
to a JSON file called matches.json, and include the top three most frequent
variants of each pattern, and their respective counts*

**grepq --read-gzip regex.json file.fastq.gz tune -n 100000 -c --names --json-matches --variants 3**

<br>

*As above, but uses the summarise command to ensure that all FASTQ records are
processed*

**grepq --read-gzip regex.json file.fastq.gz summarise -c --names --json-matches --variants 3**

<br>

*For each matched pattern in a search of no more than 100000 matches of a
gzip-compressed FASTQ file, print the pattern and the number of matches to a JSON
file called matches.json, and include all variants of each pattern, and their
respective counts. Note that the `--variants` argument is not given when `--all`
is specified."*

**grepq --read-gzip regex.json file.fastq.gz tune -n 100000 -c --names --json-matches --all**

<br>

*As above, but uses the summarise command to ensure that all FASTQ records are
processed*

**grepq --read-gzip regex.json file.fastq.gz summarise -c --names --json-matches --all**

<br>

*Print the records where none of the regex patterns are found*

**grepq regex.txt file.fastq inverted**

<br>

*Print the records where none of the regex patterns are found, with
the record ID*

**grepq -I regex.txt file.fastq inverted**

<br>

*Print the records where none of the regex patterns are found, in
FASTQ format*

**grepq -R regex.txt file.fastq inverted**

<br>

*Count the number of records where none of the regex patterns are
found*

**grepq -c regex.txt file.fastq inverted**

<br>

*Count the total number of records in the FASTQ file using an empty
pattern file*

**grepq -c empty.txt file.fastq inverted**

<br>

*For a gzip-compressed FASTQ file, bucket matched sequences into separate files
named after each regexName, with the output in FASTQ format*

**grepq -R --bucket --read-gzip regex.json file.fastq.gz**

<br>

*For a gzip-compressed FASTQ file, bucket matched sequences into separate files
named after each regexName, with the output in FASTQ format, and write a SQLite
database file, limiting the number of tetranucleotides in the TNF and CTNF fields
to two*

**grepq -R --read-gzip --writeSQL -N 2 --bucket regex.json file.fastq.gz**
