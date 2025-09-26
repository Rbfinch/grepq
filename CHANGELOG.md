
# Changelog

1.6.5 (maintenance release)

* updated dependencies

1.6.4 (maintenance release)

* updated dependencies
* updated README.md

1.6.3 (maintenance release)

* updated dependencies
* updated README.md
* updated build.rs
* update help.md

1.6.2 (maintenance release)

* fixed DOI issue in JOSS paper

1.6.1 (maintenance release)

* updates to error handling
* fixed clippy warnings for more idiomatic Rust

1.6.0 (maintenance release)

* updated JOSS paper and README.md
* updated dependencies

1.5.9 (maintenance release)

* removed Dockerfile support
* corresponding updates to documentation

1.5.8

* inclusion of canonical tetranucleotide output when `writeSQL` invoked
* corresponding updates to documentation

1.5.7 (maintenance release)

* updated README.md to clarify use of test.sh script

1.5.6 (maintenance release)

* added build.rs
* updated README.md with new installation instructions

1.5.5 (maintenance release)

* updated dependencies
* fixed typo in README.md

1.5.4

* improvement to command-line parsing when `writeSQL` is invoked
* updates to documentation

1.5.3

* updates to documentation
* addition of two help scripts for SQLite output, `summarise.sql` and `variants-as-json-array.sql`

1.5.2

* updates to documentation

1.5.1

* updated testing scripts
* added export_fastq.sql to examples directory
* corresponding updates to documentation

1.5.0

* added variants field to `writeSQL` command
* fixed summarise command performance regression
* corresponding updates to documentation

1.4.9

* added `writeSQL` command
* corresponding updates to documentation

1.4.8

* added `summarise`command
* corresponding updates to documentation
* add bash bats (TAP-compliant) testing harness

1.4.7

* added support to bucket matched regex to files
* fixed bug in `tune` subcommand that caused inflated count output
* documentation updates

1.4.6

* documentation updates

1.4.5

* updates to several dependencies
* documentation updates

1.4.4

* Support for named variants
* Corresponding changes documentation

1.4.3

* Addition of the --all option to output all variants to matches.json
* Corresponding changes documentation

1.4.2

* Addition of variants output to matches.json
* Corresponding changes documentation

1.4.1

* Updates to README.md

1.4.0

* Inclusion of unit tests and benchmarks via cfg(test) & criterion
* Updates to README.md, --help, cookbook.sh and cookbook.md

1.3.9

* Minor updates to README.md

1.3.8

* support for zstd compression
* corresponding changes to README.md,
--help, cookbook.sh and cookbook.md

1.3.7

* Minor updates to README.md

1.3.6

* Faster gzip support
* Minor updates to README.md

1.3.5

* Support for FASTA output
* Minor updates to test.sh and README.md

1.3.4

* Updates to README.md

1.3.3

* Minor improvements to README.md

1.3.2

* Minor improvements to README.md

1.3.1

* Minor improvements to help text and README.md

1.3.0

* Minor change to README.md

1.2.9

* Updates to README.md
* Updates to cookbook.md
* Addition of cookbook.sh

1.2.8

* Addition of cookbook.md
* Updates to README.md

1.2.7

* Support for IUPAC ambiguity codes
* Updates to README.md and help text
* Fixed bug in `tune` command that caused incorrect output

1.2.6

* Updates to README.md

1.2.5

* Addition of predicates to filter on header field (with regex), minimum sequence length, and minimum quality score (supports Phred+33 and Phred+64)
* No longer need to specify -j for JSON pattern files
* Updates to README.md

1.2.4

* Updates to README.md

1.2.3

* Updates to README.md

1.2.2

* Updates to README.md and help text

1.2.1

* Updates to README.md

1.2.0

* Updates to README.md and help messages

1.1.9

* Updates to README.md
* json support for regex input and tune subcommand output

1.1.8

* Updates to README.md
* gzip support

1.1.7

* Updates to README.md

1.1.6

* Added CITATION.cff

1.1.5

* Parallelized the all but the `tune` subcommand
* Changes to README.md*

1.1.4

* Fixed preprint link in README.md

1.1.3

* Added SARS-CoV-2 example to README.md

1.1.2

* Added the `inverted` subcommand
* Updated README.md

1.1.1

* Minor changes to README.md

1.1.0

* Minor changes to README.md

1.0.9

* Added the `tune` subcommand
* Reorganized the code to make it more modular and easier to maintain
* Changes to -h [--help] and README.md

1.0.8

* Minor changes to -h [--help] and README.md

1.0.7

* Minor changes to README.md

1.0.6

* Minor changes to README.md

1.0.5

* Minor updates to -h [--help]
* Minor updates to README.md
* Addition of CHANGELOG.md
