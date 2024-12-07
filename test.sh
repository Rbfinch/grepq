#!/opt/homebrew/bin/bash

# Author: Nicholas D. Crosbie
# Date: 2024-12-07

# This script is used to test the grepq program and development build.
# TODO: 
# add test for tune and count
# refactor to include tests for SRX26365298.fastq
# add Linux support for stat command

if [ "$1" == "control" ]; then
    GREPQ="grepq"
else 
    GREPQ="./target/release/grepq"
fi

declare -A tests
tests=(
    ["test-1"]="$GREPQ ./examples/regex.txt ./examples/small.fastq"
    ["test-2"]="$GREPQ ./examples/regex.txt ./examples/small.fastq inverted"
    ["test-3"]="$GREPQ -I ./examples/regex.txt ./examples/small.fastq"
    ["test-4"]="$GREPQ -I ./examples/regex.txt ./examples/small.fastq inverted"
    ["test-5"]="$GREPQ -R ./examples/regex.txt ./examples/small.fastq"
    ["test-6"]="$GREPQ -R ./examples/regex.txt ./examples/small.fastq inverted"
)

declare -A expected_sizes
expected_sizes=(
    ["test-1"]=15953
    ["test-2"]=736547
    ["test-3"]=19515
    ["test-4"]=901271
    ["test-5"]=35574
    ["test-6"]=1642712
)

test_order=("test-1" "test-2" "test-3" "test-4" "test-5" "test-6")

echo -e "\nTests run:"
echo -e "$(date +"%Y-%m-%d %H:%M:%S")\n"

for test in "${test_order[@]}"; do
    echo -e "\033[1m${test} time\033[0m"
    time ${tests[$test]} > ${test}.txt
    actual_size=$(stat -f %z "${test}.txt")
    if [ $actual_size -eq ${expected_sizes[$test]} ]; then
        echo -e "\n"
    else
        echo -e "\n\033[1;33m${test} failed\033[0m"
        echo -e "\033[1;33mexpected: ${expected_sizes[$test]} bytes\033[0m"
        echo -e "\033[1;33mgot: $actual_size bytes\033[0m"
        echo -e "\033[1;33mcommand was: ${tests[$test]} > ${test}.txt\033[0m\n"
    fi
done