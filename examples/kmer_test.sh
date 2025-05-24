#!/bin/sh

# Test script for k-mer functionality in grepq
# This script demonstrates the use of different k-mer sizes (4, 5, 6, 7)

# Ensure we have the test FASTQ file
if [ ! -f examples/small.fastq ]; then
    echo "Error: examples/small.fastq is missing!"
    exit 1
fi

# Clean up any existing database files
rm -f examples/kmers_*.db

# Build the project first
echo "Building grepq..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed. Please check the output for errors."
    exit 1
fi

# Get path to the built binary
GREPQ_BIN="./target/release/grepq"

# Test tetranucleotide (4-mer) counting with SQL output
echo "Testing tetranucleotide (4-mer) counting..."
$GREPQ_BIN examples/16S-no-iupac.txt examples/small.fastq --tetra --writeSQL examples/kmers_4.db

# Test pentanucleotide (5-mer) counting with SQL output
echo "Testing pentanucleotide (5-mer) counting..."
$GREPQ_BIN examples/16S-no-iupac.txt examples/small.fastq --penta --writeSQL examples/kmers_5.db

# Test hexanucleotide (6-mer) counting with SQL output
echo "Testing hexanucleotide (6-mer) counting..."
$GREPQ_BIN examples/16S-no-iupac.txt examples/small.fastq --hexa --writeSQL examples/kmers_6.db

# Test heptanucleotide (7-mer) counting with SQL output
echo "Testing heptanucleotide (7-mer) counting..."
$GREPQ_BIN examples/16S-no-iupac.txt examples/small.fastq --hepta --writeSQL examples/kmers_7.db

# Test multiple k-mer sizes simultaneously
echo "Testing multiple k-mer sizes simultaneously..."
$GREPQ_BIN examples/16S-no-iupac.txt examples/small.fastq --tetra --penta --hexa --hepta --writeSQL examples/kmers_all.db

echo "K-mer tests complete."
echo "DB files created: kmers_4.db, kmers_5.db, kmers_6.db, kmers_7.db, kmers_all.db"
