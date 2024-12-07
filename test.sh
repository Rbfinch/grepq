#!/bin/bash

# This script is used to test the grepq program and development build.
# TODO: 
# add test for tune and count
# add Linux support for stat command

if [ "$1" == "control" ]; then
    GREPQ="grepq"
else 
    GREPQ="./target/release/grepq"
fi

TEST_1="$GREPQ ./examples/regex.txt ./examples/small.fastq"
TEST_2="$GREPQ ./examples/regex.txt ./examples/small.fastq inverted"
TEST_3="$GREPQ -I ./examples/regex.txt ./examples/small.fastq"
TEST_4="$GREPQ -I ./examples/regex.txt ./examples/small.fastq inverted"
TEST_5="$GREPQ -R ./examples/regex.txt ./examples/small.fastq"
TEST_6="$GREPQ -R ./examples/regex.txt ./examples/small.fastq inverted"

echo -e "\n"

# test-1
echo -e "\033[1mtest-1 time\033[0m"
time $TEST_1 > test-1.txt
if [ $(stat -f %z "test-1.txt") -eq 15953 ]; then
    echo -e "\n"
else
    echo -e "\n\033[1;33mtest-1 failed\033[0m"
    echo -e "\033[1;33mgot: $(stat -f %z "test-1.txt") bytes\033[0m"
    echo -e "\033[1;33mexpected: 15953 bytes\033[0m"
    echo -e "\033[1;33mcommand was: $TEST_1 > test-1.txt\033[0m\n"
fi

# test-2
echo -e "\033[1mtest-2 time\033[0m"
time $TEST_2 > test-2.txt
if [ $(stat -f %z "test-2.txt") -eq 736547 ]; then
    echo -e "\n"
else
    echo -e "\n\033[1;33mtest-2 failed\033[0m"
    echo -e "\033[1;33mgot: $(stat -f %z "test-2.txt") bytes\033[0m"
    echo -e "\033[1;33mexpected: 736547 bytes\033[0m"
    echo -e "\033[1;33mcommand was: $TEST_2 > test-2.txt\033[0m\n"
fi

# test-3
echo -e "\033[1mtest-3 time\033[0m"
time $TEST_3 > test-3.txt
if [ $(stat -f %z "test-3.txt") -eq 19515 ]; then
    echo -e "\n"
else
    echo -e "\n\033[1;33mtest-3 failed\033[0m"
    echo -e "\033[1;33mgot: $(stat -f %z "test-3.txt") bytes\033[0m"
    echo -e "\033[1;33mexpected: 19515 bytes\033[0m"
    echo -e "\033[1;33mcommand was: $TEST_3 > test-3.txt\033[0m\n"
fi

# test-4
echo -e "\033[1mtest-4 time\033[0m"
time $TEST_4 > test-4.txt
if [ $(stat -f %z "test-4.txt") -eq 901271 ]; then
    echo -e "\n"
else
    echo -e "\n\033[1;33mtest-4 failed\033[0m"
    echo -e "\033[1;33mgot: $(stat -f %z "test-4.txt") bytes\033[0m"
    echo -e "\033[1;33mexpected: 901271 bytes\033[0m"
    echo -e "\033[1;33mcommand was: $TEST_4 > test-4.txt\033[0m\n"
fi

# test-5
echo -e "\033[1mtest-5 time\033[0m"
time $TEST_5 > test-5.txt
if [ $(stat -f %z "test-5.txt") -eq 35574 ]; then
    echo -e "\n"
else
    echo -e "\n\033[1;33mtest-5 failed\033[0m"
    echo -e "\033[1;33mgot: $(stat -f %z "test-5.txt") bytes\033[0m"
    echo -e "\033[1;33mexpected: 35574 bytes\033[0m"
    echo -e "\033[1;33mcommand was: $TEST_5 > test-5.txt\033[0m\n"
fi

# test-6
echo -e "\033[1mtest-6 time\033[0m"
time $TEST_6 > test-6.txt
if [ $(stat -f %z "test-6.txt") -eq 1642712 ]; then
    echo -e "\n"
else
    echo -e "\n\033[1;33mtest-6 failed\033[0m"
    echo -e "\033[1;33mgot: $(stat -f %z "test-6.txt") bytes\033[0m"
    echo -e "\033[1;33mexpected: 1642712 bytes\033[0m"
    echo -e "\033[1;33mcommand was: $TEST_6 > test-6.txt\033[0m\n"
fi