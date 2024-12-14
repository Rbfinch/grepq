#!/usr/bin/env bash

### Compare two snapshot times 
# Author: Nicholas D. Crosbie
# Date: December 2024 
###

# Exit immediately if a command exits with a non-zero status
set -e

# Check if the path to the input file is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <path_to_first_input_file>"
    exit 1
fi

# Check if the path to the second input file is provided
if [ -z "$2" ]; then
    echo "Usage: $0 <path_to_first_input_file> <path_to_second_input_file>"
    exit 1
fi

INPUT_FILE="$1"
SECOND_INPUT_FILE="$2"

# Function to process input file and extract times in seconds
extract_times() {
    local file="$1"
    local times=()
    while IFS= read -r line; do
        real_time=$(echo "$line" | grep real | awk '{print $2}')
        if [ -n "$real_time" ]; then
            minutes=$(echo "$real_time" | cut -d'm' -f1)
            seconds=$(echo "$real_time" | cut -d'm' -f2 | cut -d's' -f1)
            total_seconds=$(echo "$minutes * 60 + $seconds" | bc)
            times+=("$total_seconds")
        fi
    done < "$file"
    echo "${times[@]}"
}

# Extract times from both input files
times_file1=($(extract_times "$INPUT_FILE"))
times_file2=($(extract_times "$SECOND_INPUT_FILE"))

# Check if both files have the same number of times
if [ ${#times_file1[@]} -ne ${#times_file2[@]} ]; then
    echo "Error: The two files have a different number of times."
    exit 1
fi

# Calculate the percent difference for each corresponding time
for i in "${!times_file1[@]}"; do
    time1=${times_file1[$i]}
    time2=${times_file2[$i]}
    percent_difference=$(echo "scale=2; (($time2 - $time1) / $time1) * 100" | bc)
    echo "%diff test-$((i + 1)): $percent_difference%"
done

