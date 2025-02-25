#!/bin/bash

# Default executable
APP="/Users/nicholascrosbie/Documents/repos/grepq/target/release/grepq"

# Check for --control flag
if [ "$1" == "--control" ]; then
    APP="grepq"
    shift
fi

# Export the APP variable for the Bats tests
export APP

# Run the Bats tests
bats /Users/nicholascrosbie/Documents/repos/grepq/test/test.bats "$@"
