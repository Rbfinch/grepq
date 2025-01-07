#!/usr/bin/env bash

### This script is used to benchmark the grepq program against gawk and awk. 
# Author: Nicholas D. Crosbie
# Date: January 2025 
###

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 patterns.txt datafile"
    exit 1
fi

patterns_file=$1
data_file=$2

# change gawk to awk to benchmark against awk
gawk -v patterns_file="$patterns_file" '
BEGIN {
    while ((getline pattern < patterns_file) > 0) {
        patterns[pattern] = 1
    }
    close(patterns_file)
}
/length/ { nextLine = 1; next }
nextLine { 
    for (pattern in patterns) {
        if ($0 ~ pattern) {
            print
            break
        }
    }
    nextLine = 0 
}
' "$data_file"
