#!/usr/bin/env bash

### This script is used to test the grepq program and development build.
# Author: Nicholas D. Crosbie
# Date: December 2024 
###

## Redirect all output to a file, with the current date and time as the filename
# ./test.sh tests.yaml &> ../snapshots/$(date +"%Y-%m-%d-%H:%M:%S").txt
# ./test.sh control tests.yaml &> ../snapshots/$(date +"%Y-%m-%d-%H:%M:%S").txt

# Exit immediately if a command exits with a non-zero status
set -e

# Print the operating system and CPU architecture
if [[ "$OSTYPE" == "linux-"* ]]; then
  echo "OS: Linux"
  echo "CPU: $(uname -m)"
  STAT_CMD="stat -c %s"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "OS: macOS"
  echo "CPU: $(uname -m)"
  STAT_CMD="stat -f %z"
else
  echo "OS: Unknown"
  exit 1
fi

# Check if the control flag is provided
if [ "$1" == "control" ]; then
    GREPQ="grepq"
    echo "version: $(grepq -V)"
    echo "tests:" $2
    shift
else 
    GREPQ="../target/release/grepq"
    echo "version:"
    ../target/release/grepq -V
fi

# Check if the path to the test file is provided
if [ -z "$1" ]; then
    echo "Usage: $0 [control] <path_to_tests_yaml>"
    exit 1
fi

TEST_FILE="$1"

# Load tests and expected sizes from YAML file
tests_yaml=$(yq '.tests' "$TEST_FILE")
expected_sizes_yaml=$(yq '.expected_sizes' "$TEST_FILE")

declare -A tests
declare -A expected_sizes

# Check if tests_yaml and expected_sizes_yaml are not empty
if [ -n "$tests_yaml" ] && [ -n "$expected_sizes_yaml" ]; then
    for key in $(echo "$tests_yaml" | yq 'keys | .[]' -); do
        if [ "$key" != "null" ]; then
            tests[$key]=$(echo "$tests_yaml" | yq ".$key" - | sed "s|\$GREPQ|$GREPQ|g")
            expected_sizes[$key]=$(echo "$expected_sizes_yaml" | yq ".$key" -)
        fi
    done
fi

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
    echo "${tests[$test]}"
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
            actual_size=$($STAT_CMD "matches.json")
            if [ $actual_size -eq ${expected_sizes[$test]} ]; then
                echo -e "\n"
            else
                echo -e "\n${ORANGE}${test} failed${RESET}"
                echo -e "${ORANGE}expected: ${expected_sizes[$test]} bytes${RESET}"
                echo -e "${ORANGE}got: $actual_size bytes${RESET}"
                echo -e "${ORANGE}command was: ${tests[$test]}${RESET}\n"
            fi
        else
            time ${tests[$test]} > /tmp/${test}.txt
            actual_size=$($STAT_CMD "/tmp/${test}.txt")
            if [ $actual_size -eq ${expected_sizes[$test]} ]; then
                echo -e "\n"
            else
                echo -e "\n${ORANGE}${test} failed${RESET}"
                echo -e "${ORANGE}expected: ${expected_sizes[$test]} bytes${RESET}"
                echo -e "${ORANGE}got: $actual_size bytes${RESET}"
                echo -e "${ORANGE}command was: ${tests[$test]} > /tmp/${test}.txt${RESET}\n"
            fi
        fi
    fi
done

