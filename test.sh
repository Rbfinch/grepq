#!/usr/bin/env bash

# This script is used to test the grepq program and development build.
# Author: Nicholas D. Crosbie
# Date: December 2024 

# TODO: 
    # add test for tune command 
    # add Linux support for stat command

# Exit immediately if a command exits with a non-zero status
set -e

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
    ["test-7"]="$GREPQ -c ./examples/regex.txt ./examples/small.fastq"
    ["test-8"]="$GREPQ -c ./examples/regex.txt ./examples/small.fastq inverted"
    ["test-9"]="$GREPQ ./examples/regex.txt ./examples/small.fastq tune -n 10000 -c"
    ["test-10"]="$GREPQ -xj ./examples/regex.json ./examples/SRX26365298.fastq.gz tune -n 100000 -c --names --json-matches"
)

declare -A expected_sizes
expected_sizes=(
    ["test-1"]=15953
    ["test-2"]=736547
    ["test-3"]=19515
    ["test-4"]=901271
    ["test-5"]=35574
    ["test-6"]=1642712
    ["test-7"]=53
    ["test-8"]=2447
    ["test-9"]=411
    ["test-10"]=2366
)

# Using an array to maintain the order of the tests
test_order=("test-1" "test-2" "test-3" "test-4" "test-5" "test-6" "test-7" "test-8" "test-9" "test-10")

# Color codes
BOLD="\033[1m"
ORANGE="\033[38;2;255;165;0m"
RESET="\033[0m"

echo -e "\nTests run:"
echo -e "$(date +"%Y-%m-%d %H:%M:%S")\n"

for test in "${test_order[@]}"; do
    echo -e "${BOLD}${test} ${RESET}"
    if [ "$test" == "test-7" ] || [ "$test" == "test-8" ]; then
        actual_count=$(time ${tests[$test]})
        if [ $actual_count -eq ${expected_sizes[$test]} ]; then
            echo -e "\n"
        else
            echo -e "\n${ORANGE}${test} failed${RESET}"
            echo -e "${ORANGE}expected: ${expected_sizes[$test]} counts${RESET}"
            echo -e "${ORANGE}got: $actual_count counts${RESET}"
            echo -e "${ORANGE}command was: ${tests[$test]}${RESET}\n"
        fi
    else
        if [ "$test" == "test-10" ]; then
            time ${tests[$test]}
            actual_size=$(stat -f %z "matches.json")
            if [ $actual_size -eq ${expected_sizes[$test]} ]; then
                echo -e "\n"
            else
                echo -e "\n${ORANGE}${test} failed${RESET}"
                echo -e "${ORANGE}expected: ${expected_sizes[$test]} bytes${RESET}"
                echo -e "${ORANGE}got: $actual_size bytes${RESET}"
                echo -e "${ORANGE}command was: ${tests[$test]}${RESET}\n"
            fi
        else
            time ${tests[$test]} > ${test}.txt
            if [ "$test" == "test-9" ]; then
                actual_size=$(stat -f %z "${test}.txt")
                if [ $actual_size -eq ${expected_sizes[$test]} ]; then
                    echo -e "\n"
                else
                    echo -e "\n${ORANGE}${test} failed${RESET}"
                    echo -e "${ORANGE}expected: ${expected_sizes[$test]} bytes${RESET}"
                    echo -e "${ORANGE}got: $actual_size bytes${RESET}"
                    echo -e "${ORANGE}command was: ${tests[$test]} > ${test}.txt${RESET}\n"
                fi
            else
                actual_size=$(stat -f %z "${test}.txt")
                if [ $actual_size -eq ${expected_sizes[$test]} ]; then
                    echo -e "\n"
                else
                    echo -e "\n${ORANGE}${test} failed${RESET}"
                    echo -e "${ORANGE}expected: ${expected_sizes[$test]} bytes${RESET}"
                    echo -e "${ORANGE}got: $actual_size bytes${RESET}"
                    echo -e "${ORANGE}command was: ${tests[$test]} > ${test}.txt${RESET}\n"
                fi
            fi
        fi
    fi
done

