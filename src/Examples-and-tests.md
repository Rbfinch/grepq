## Examples and tests

Get instructions and examples using `grepq -h`, or `grepq tune -h`, `grepq summarise -h` and `grepq inverted -h` for more information on the `tune`, `summarise` and `inverted` commands, respectively. See the `examples` directory for examples of pattern files and FASTQ files, and the `cookbook.sh` and `cookbook.md` files for more examples.

_File sizes of outfiles to verify `grepq` is working correctly, using the regex file `16S-no-iupac.txt` and the small fastq file `small.fastq`, both located in the `examples` directory:_

```bash
grepq ./examples/16S-no-iupac.txt ./examples/small.fastq > outfile.txt 
15953

grepq  ./examples/16S-no-iupac.txt ./examples/small.fastq inverted > outfile.txt
736547

grepq -I ./examples/16S-no-iupac.txt ./examples/small.fastq > outfile.txt
19515

grepq -I ./examples/16S-no-iupac.txt ./examples/small.fastq inverted > outfile.txt 
901271

grepq -R ./examples/16S-no-iupac.txt ./examples/small.fastq > outfile.txt
35574

grepq -R ./examples/16S-no-iupac.txt ./examples/small.fastq inverted > outfile.txt 
1642712
```

For the curious-minded, note that the regex patterns in `16S-no-iupac.txt`, `16S-iupac.json`, `16S-no-iupac.json`, and `16S-iupac-and-predicates.json` are from Table 3 of Martinez-Porchas, Marcel, et al. "How conserved are the conserved 16S-rRNA regions?." PeerJ 5 (2017): e3036.

For more examples, see the `examples` directory and the [cookbook](https://github.com/Rbfinch/grepq/blob/main/cookbook.md), available also as a shell script in the `examples` directory.

**Test script**

You may also run the test script (`test.sh`) in the `examples` directory to more fully test `grepq`. From the `examples directory`, run the following command:

```bash
./test.sh commands-1.yaml; ./test.sh commands-2.yaml; ./test.sh commands-3.yaml; ./test.sh commands-4.yaml
```

If all tests pass, there will be no orange (warning) text in the output, and no test will
report a failure. A summary of the number of passing and failing tests will be displayed at the end of the output. All tests should pass.

_Example of failing test output:_

<span style="color: rgb(255, 165, 0);">
test-7 failed <br>
expected: 54 counts <br>
got: 53 counts <br>
command was: ../target/release/grepq -c 16S-no-iupac.txt small.fastq <br>
</span>
<br>

Further, you can run the `cookbook.sh` script in the `examples` directory to test the cookbook examples, and you can use `predate` (<https://crates.io/crates/predate>) if you prefer a Rust application to a shell script.

Finally, you can run the `test.bats` script in the `test` directory to test the `grepq` executable (`/target/release/grepq`). The `bats` shell script testing framework is required to run the `test.bats` script. You can install `bats` using `brew install bats-core` on macOS, or `sudo apt-get install bats` on Ubuntu. You will need to put the the file `SRX26365298.fastq.gz` (obtain from the SRA using `fastq-dump --accession SRX26365298`) in the `examples` directory to run all tests. There is a `run_tests.sh` script in the `test` directory that will run the `test.bats` script, with various options.

```bash

**SARS-CoV-2 example**

Count of the top five most frequently matched patterns found in SRX26602697.fastq using the pattern file SARS-CoV-2.txt (this pattern file contains 64 sequences of length 60 from Table II of this [preprint](https://doi.org/10.1101/2021.04.14.439840)):

```bash
time grepq SARS-CoV-2.txt SRX26602697.fastq tune -n 10000 -c | head -5
GTATGGAAAAGTTATGTGCATGTTGTAGACGGTTGTAATTCATCAACTTGTATGATGTGT: 1595
CGGAACGTTCTGAAAAGAGCTATGAATTGCAGACACCTTTTGAAATTAAATTGGCAAAGA: 693
TCCTTACTGCGCTTCGATTGTGTGCGTACTGCTGCAATATTGTTAACGTGAGTCTTGTAA: 356
GCGCTTCGATTGTGTGCGTACTGCTGCAATATTGTTAACGTGAGTCTTGTAAAACCTTCT: 332
CCGTAGCTGGTGTCTCTATCTGTAGTACTATGACCAATAGACAGTTTCATCAAAAATTAT: 209

________________________________________________________
Executed in  218.80 millis    fish           external
   usr time  188.97 millis    0.09 millis  188.88 millis
   sys time   31.47 millis    4.98 millis   26.49 millis

```

Obtain `SRX26602697.fastq` from the SRA using `fastq-dump --accession SRX26602697`.

