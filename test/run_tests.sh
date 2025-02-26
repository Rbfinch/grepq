#!/bin/bash

# Default executable and flags
APP="/Users/nicholascrosbie/Documents/repos/grepq/target/release/grepq"
COMPUTE_TIMINGS=false
REMAINING_ARGS=()

# Process all arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --control)
            APP="grepq"
            ;;
        --timings)
            COMPUTE_TIMINGS=true
            ;;
        *)
            REMAINING_ARGS+=("$1")
            ;;
    esac
    shift
done

# Export variables for the Bats tests
export APP
export COMPUTE_TIMINGS

# Run the Bats tests with remaining arguments
bats /Users/nicholascrosbie/Documents/repos/grepq/test/test.bats "${REMAINING_ARGS[@]}"
