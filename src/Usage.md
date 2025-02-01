## Usage

Get instructions and examples using `grepq -h`, and `grepq tune -h` and `grepq inverted -h` for more information on the `tune` and `inverted` commands, respectively.

> **Note:**
`grepq` can output to several formats, including those that are gzip or zstd compressed. `grepq`, however, will only accept a FASTQ file or a compressed (gzip or zstd) FASTQ file as the sequence data file. If you get an error message, check that the input data file is a FASTQ file or a gzip or zstd compressed FASTQ file, and that you have specified the correct file format (--read-gzip or --read-zstd for FASTQ files compressed by gzip and zstd, respectively), and file path. Pattern files must contain one regex pattern per line or be provided in JSON format, and patterns are case-sensitive. You can supply an empty pattern file to count the total number of records in the FASTQ file. The regex patterns for matching FASTQ sequences should only include the DNA sequence characters (A, C, G, T), or IUPAC ambiguity codes (N, R, Y, etc.). See `16S-no-iupac.txt`, `16S-iupac.json`, `16S-no-iupac.json`, and `16S-iupac-and-predicates.json` in the `examples` directory for examples of valid pattern files. Regex patterns to match the header field (= record ID line) must comply with the Rust regex library syntax (<https://docs.rs/regex/latest/regex/#syntax>). If you get an error message, be sure to escape any special characters in the regex pattern.

### Preparing pattern files

Whilst `grepq` can accept pattern files in plain text format (one regex pattern per line), it is recommended to use JSON format for more complex pattern files since JSON pattern files can contain named regex sets, named regex patterns, and named and unnamed variants. JSON can be a little verbose, so you may want to prepare you pattern file in YAML format (for example, see `16S-iupac.yaml` in the `examples` directory) and then convert it to JSON using a tool like `yq`. For example, to convert a YAML pattern file to JSON, use the following command:

```bash
yq eval '. | tojson' pattern-file.yaml > pattern-file.json
```

`grepq` will validate the JSON pattern file before processing it, and will provide an error message if the JSON pattern file is not valid. However, if you wish to validate the JSON pattern file before running `grepq`, you can use a tool such as `ajv` and `grepq`'s JSON schema file (`grepq-schema.json`, located in the `examples` directory), for example:

```bash
ajv --strict=false -s grepq-schema.json -d pattern-file.json
```

