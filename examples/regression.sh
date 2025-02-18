#!/usr/bin/env bash
# Compare runtime performance of test run to control run of a benchmarking script

if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <test_run.json> <control_run.json> <threshold_percent>"
    exit 1
fi

file1=$1
file2=$2
threshold=$3

# Extract durations from both JSON files
durations1=$(jq -r '.[].duration' "$file1")
durations2=$(jq -r '.[].duration' "$file2")

# Check if both files have the same number of tests
count1=$(echo "$durations1" | wc -l)
count2=$(echo "$durations2" | wc -l)

if [ "$count1" -ne "$count2" ]; then
    echo "Error: The number of tests in the two files do not match."
    exit 1
fi

# Calculate and compare the percent differences
index=1
echo "$durations1" | while read -r duration1; do
    duration2=$(echo "$durations2" | sed -n "${index}p")
    percent_diff=$(echo "scale=2; (($duration2 - $duration1) / $duration1) * 100" | bc)
    abs_percent_diff=$(echo "$percent_diff" | tr -d -)

    echo "Test $index: Percent difference = $percent_diff%"

    if (( $(echo "$percent_diff < 0" | bc -l) )) && (( $(echo "$abs_percent_diff > $threshold" | bc -l) )); then
        printf "\e[1;33mPerformance regression: Test $index exceeds threshold with a percent difference of %s%%\e[0m\n" "$percent_diff"
    elif (( $(echo "$percent_diff > 0" | bc -l) )) && (( $(echo "$abs_percent_diff > $threshold" | bc -l) )); then
        printf "\e[1;32mPerformance improvement: Test $index exceeds threshold with a percent difference of %s%%\e[0m\n" "$percent_diff"
    fi

    index=$((index + 1))
done
