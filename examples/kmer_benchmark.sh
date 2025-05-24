#!/bin/bash

# Performance testing script for different k-mer sizes
# This script benchmarks the performance of grepq with different k-mer sizes

OUTPUT_DIR="kmer_benchmark_results"
mkdir -p "$OUTPUT_DIR"
GREPQ_BIN="./target/release/grepq"

echo "Running benchmark tests for different k-mer sizes..."
echo "===================================================="

# Test tetra nucleotides
echo "Testing tetranucleotides..."
time $GREPQ_BIN --writeSQL "$OUTPUT_DIR/tetra.db" --tetra examples/SARS-CoV-2.txt examples/small.fastq
echo ""

# Test penta nucleotides
echo "Testing pentanucleotides..."
time $GREPQ_BIN --writeSQL "$OUTPUT_DIR/penta.db" --penta examples/SARS-CoV-2.txt examples/small.fastq
echo ""

# Test hexa nucleotides
echo "Testing hexanucleotides..."
time $GREPQ_BIN --writeSQL "$OUTPUT_DIR/hexa.db" --hexa examples/SARS-CoV-2.txt examples/small.fastq
echo ""

# Test hepta nucleotides
echo "Testing heptanucleotides..."
time $GREPQ_BIN --writeSQL "$OUTPUT_DIR/hepta.db" --hepta examples/SARS-CoV-2.txt examples/small.fastq
echo ""

# Test all nucleotides together
echo "Testing all k-mer sizes together..."
time $GREPQ_BIN --writeSQL "$OUTPUT_DIR/all.db" --tetra --penta --hexa --hepta examples/SARS-CoV-2.txt examples/small.fastq
echo ""

echo "===================================================="
echo "Benchmark tests complete. Results saved to $OUTPUT_DIR"

# Count entries in each table
echo "Count of entries in each database:"
for db in "$OUTPUT_DIR"/*.db; do
    echo "Database: $(basename "$db")"

    # Check for tetranucleotides table
    if sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='tetranucleotides';" | grep -q "tetranucleotides"; then
        echo -n "Tetranucleotides: "
        sqlite3 "$db" "SELECT COUNT(*) FROM tetranucleotides;"
    else
        echo "No tetranucleotides table"
    fi

    # Check for pentanucleotides table
    if sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='pentanucleotides';" | grep -q "pentanucleotides"; then
        echo -n "Pentanucleotides: "
        sqlite3 "$db" "SELECT COUNT(*) FROM pentanucleotides;"
    else
        echo "No pentanucleotides table"
    fi

    # Check for hexanucleotides table
    if sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='hexanucleotides';" | grep -q "hexanucleotides"; then
        echo -n "Hexanucleotides: "
        sqlite3 "$db" "SELECT COUNT(*) FROM hexanucleotides;"
    else
        echo "No hexanucleotides table"
    fi

    # Check for heptanucleotides table
    if sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='heptanucleotides';" | grep -q "heptanucleotides"; then
        echo -n "Heptanucleotides: "
        sqlite3 "$db" "SELECT COUNT(*) FROM heptanucleotides;"
    else
        echo "No heptanucleotides table"
    fi

    # Also check for canonical tables
    if sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='canonical_tetranucleotides';" | grep -q "canonical_tetranucleotides"; then
        echo -n "Canonical Tetranucleotides: "
        sqlite3 "$db" "SELECT COUNT(*) FROM canonical_tetranucleotides;"
    fi

    if sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='canonical_pentanucleotides';" | grep -q "canonical_pentanucleotides"; then
        echo -n "Canonical Pentanucleotides: "
        sqlite3 "$db" "SELECT COUNT(*) FROM canonical_pentanucleotides;"
    fi

    if sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='canonical_hexanucleotides';" | grep -q "canonical_hexanucleotides"; then
        echo -n "Canonical Hexanucleotides: "
        sqlite3 "$db" "SELECT COUNT(*) FROM canonical_hexanucleotides;"
    fi

    if sqlite3 "$db" "SELECT name FROM sqlite_master WHERE type='table' AND name='canonical_heptanucleotides';" | grep -q "canonical_heptanucleotides"; then
        echo -n "Canonical Heptanucleotides: "
        sqlite3 "$db" "SELECT COUNT(*) FROM canonical_heptanucleotides;"
    fi

    echo ""
done
