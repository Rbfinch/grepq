#!/bin/bash

# Performance testing script for different k-mer sizes
# This script benchmarks the performance of grepq with different k-mer sizes

OUTPUT_DIR="kmer_benchmark_results"
mkdir -p "$OUTPUT_DIR"

echo "Running benchmark tests for different k-mer sizes..."
echo "===================================================="

# Test tetra nucleotides
echo "Testing tetranucleotides..."
time ../target/debug/grepq --writeSQL "$OUTPUT_DIR/tetra.db" --tetranucleotides examples/SARS-CoV-2.txt examples/small.fastq
echo ""

# Test penta nucleotides
echo "Testing pentanucleotides..."
time ../target/debug/grepq --writeSQL "$OUTPUT_DIR/penta.db" --penta examples/SARS-CoV-2.txt examples/small.fastq
echo ""

# Test hexa nucleotides
echo "Testing hexanucleotides..."
time ../target/debug/grepq --writeSQL "$OUTPUT_DIR/hexa.db" --hexa examples/SARS-CoV-2.txt examples/small.fastq
echo ""

# Test hepta nucleotides
echo "Testing heptanucleotides..."
time ../target/debug/grepq --writeSQL "$OUTPUT_DIR/hepta.db" --hepta examples/SARS-CoV-2.txt examples/small.fastq
echo ""

# Test all nucleotides together
echo "Testing all k-mer sizes together..."
time ../target/debug/grepq --writeSQL "$OUTPUT_DIR/all.db" --tetranucleotides --penta --hexa --hepta examples/SARS-CoV-2.txt examples/small.fastq
echo ""

echo "===================================================="
echo "Benchmark tests complete. Results saved to $OUTPUT_DIR"

# Count entries in each table
echo "Count of entries in each database:"
for db in "$OUTPUT_DIR"/*.db; do
    echo "Database: $(basename "$db")"
    sqlite3 "$db" "SELECT 'Tetranucleotides:', COUNT(*) FROM tetranucleotides 2>/dev/null;" 2>/dev/null || echo "No tetranucleotides table"
    sqlite3 "$db" "SELECT 'Pentanucleotides:', COUNT(*) FROM pentanucleotides 2>/dev/null;" 2>/dev/null || echo "No pentanucleotides table"
    sqlite3 "$db" "SELECT 'Hexanucleotides:', COUNT(*) FROM hexanucleotides 2>/dev/null;" 2>/dev/null || echo "No hexanucleotides table"
    sqlite3 "$db" "SELECT 'Heptanucleotides:', COUNT(*) FROM heptanucleotides 2>/dev/null;" 2>/dev/null || echo "No heptanucleotides table"
    echo ""
done
