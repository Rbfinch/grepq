#!/usr/bin/env bats

log_file="/Users/nicholascrosbie/Documents/repos/grepq/test/test_timings.log"

log_time() {
    local message="$@"
    date "+%Y-%m-%d %H:%M:%S - $message" >> "$log_file"
}

setup() {
    # Get absolute paths
    REPO_ROOT="$( cd "$( dirname "${BATS_TEST_FILENAME}" )/.." && pwd )"
    EXAMPLES_DIR="${REPO_ROOT}/examples"
    
    # Detect OS for stat command
    case "$(uname -s)" in
        Darwin*)
            STAT_CMD="stat -f%z";;
        Linux*)
            STAT_CMD="stat -c%s";;
        *)
            echo "Unsupported operating system"
            exit 1;;
    esac
    
    # Ensure we're in the examples directory
    cd "${EXAMPLES_DIR}" || exit 1
}

teardown() {
    rm -f /tmp/test-*.txt matches.json
    rm -f "${EXAMPLES_DIR}"/Primer-contig*.fastq
}

verify_size() {
    local output_file=$1
    local expected_size=$2
    local actual_size
    actual_size=$(${STAT_CMD} "$output_file")
    [ "$actual_size" -eq "$expected_size" ]
}

verify_count() {
    local output_file=$1
    local expected_count=$2
    local actual_count
    actual_count=$(cat "$output_file" | tr -d '\n\r')  # Remove newlines and carriage returns
    [ "$actual_count" -eq "$expected_count" ]
}

verify_bucket_files() {
    declare -A expected_sizes=(
        ["Primer-contig-08a.fastq"]=328287
        ["Primer-contig-03R.fastq"]=1260779
        ["Primer-contig-01.fastq"]=273245
        ["Primer-contig-06a.fastq"]=281192
        ["Primer-contig-05bR.fastq"]=457804
        ["Primer-contig-01R.fastq"]=288594
        ["Primer-contig-06c.fastq"]=216915
        ["Primer-contig-03.fastq"]=1111898
        ["Primer-contig-09R.fastq"]=337832
        ["Primer-contig-04.fastq"]=2396586
        ["Primer-contig-07bR.fastq"]=241451
        ["Primer-contig-08b.fastq"]=233844
        ["Primer-contig-08aR.fastq"]=352585
        ["Primer-contig-06b.fastq"]=212779
        ["Primer-contig-02.fastq"]=822125
        ["Primer-contig-10aR.fastq"]=250998
        ["Primer-contig-06bR.fastq"]=209474
        ["Primer-contig-02R.fastq"]=898727
        ["Primer-contig-05aR.fastq"]=431769
        ["Primer-contig-05a.fastq"]=434836
        ["Primer-contig-07a.fastq"]=1100490
        ["Primer-contig-05b.fastq"]=474172
        ["Primer-contig-06aR.fastq"]=293099
        ["Primer-contig-04R.fastq"]=2313454
        ["Primer-contig-07aR.fastq"]=1224340
        ["Primer-contig-09.fastq"]=364610
        ["Primer-contig-06cR.fastq"]=210365
        ["Primer-contig-10.fastq"]=252598
        ["Primer-contig-08bR.fastq"]=225200
        ["Primer-contig-07b.fastq"]=238068
    )
    
    for file in "${!expected_sizes[@]}"; do
        local full_path="${EXAMPLES_DIR}/${file}"
        if [ ! -f "$full_path" ]; then
            echo "Missing expected file: $full_path"
            return 1
        fi
        local actual_size
        actual_size=$(${STAT_CMD} "$full_path")
        if [ "$actual_size" -ne "${expected_sizes[$file]}" ]; then
            echo "Size mismatch for $file: expected ${expected_sizes[$file]}, got $actual_size"
            return 1
        fi
    done
    return 0
}

# Check for --control flag before running tests
if [ "$1" == "--control" ]; then
    APP="grepq"
    shift
fi

measure_time() {
    local command="$1"
    local result
    result=$(hyperfine --warmup 1 --runs 1 --export-json /tmp/hyperfine.json "$command")
    jq '.results[0].times[0]' /tmp/hyperfine.json
}

@test "test-1: Basic sequence match" {
    log_time "Starting test-1"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq > /tmp/test-1.txt")
    verify_size "/tmp/test-1.txt" 15953
    log_time "test-1 took $duration seconds"
    log_time "Ending test-1"
}

@test "test-2: Inverted sequence match" {
    log_time "Starting test-2"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-2.txt")
    verify_size "/tmp/test-2.txt" 736547
    log_time "test-2 took $duration seconds"
    log_time "Ending test-2"
}

@test "test-3: Include record ID" {
    log_time "Starting test-3"
    duration=$(measure_time "${APP} -I ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq > /tmp/test-3.txt")
    verify_size "/tmp/test-3.txt" 19515
    log_time "test-3 took $duration seconds"
    log_time "Ending test-3"
}

@test "test-4: Include record ID with inverted match" {
    log_time "Starting test-4"
    duration=$(measure_time "${APP} -I ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-4.txt")
    verify_size "/tmp/test-4.txt" 901271
    log_time "test-4 took $duration seconds"
    log_time "Ending test-4"
}

@test "test-5: Full FASTQ record output" {
    log_time "Starting test-5"
    duration=$(measure_time "${APP} -R ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq > /tmp/test-5.txt")
    verify_size "/tmp/test-5.txt" 35574
    log_time "test-5 took $duration seconds"
    log_time "Ending test-5"
}

@test "test-6: Full FASTQ record output with inverted match" {
    log_time "Starting test-6"
    duration=$(measure_time "${APP} -R ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-6.txt")
    verify_size "/tmp/test-6.txt" 1642712
    log_time "test-6 took $duration seconds"
    log_time "Ending test-6"
}

@test "test-7: Count matches" {
    log_time "Starting test-7"
    duration=$(measure_time "${APP} -c ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq > /tmp/test-7.txt")
    verify_count "/tmp/test-7.txt" 53
    log_time "test-7 took $duration seconds"
    log_time "Ending test-7"
}

@test "test-8: Count inverted matches" {
    log_time "Starting test-8"
    duration=$(measure_time "${APP} -c ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-8.txt")
    verify_count "/tmp/test-8.txt" 2447
    log_time "test-8 took $duration seconds"
    log_time "Ending test-8"
}

@test "test-9: Tune command with count" {
    log_time "Starting test-9"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-no-iupac.txt ${EXAMPLES_DIR}/small.fastq tune -n 2000 -c > /tmp/test-9.txt")
    verify_size "/tmp/test-9.txt" 310
    log_time "test-9 took $duration seconds"
    log_time "Ending test-9"
}

@test "test-10: Tune command with JSON output" {
    log_time "Starting test-10"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small-copy.fastq.gz tune -n 2000 -c --names --json-matches")
    verify_size "matches.json" 3704
    log_time "test-10 took $duration seconds"
    log_time "Ending test-10"
}

@test "test-11: Basic sequence match with JSON patterns" {
    log_time "Starting test-11"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-11.txt")
    verify_size "/tmp/test-11.txt" 15953
    log_time "test-11 took $duration seconds"
    log_time "Ending test-11"
}

@test "test-12: Inverted sequence match with JSON patterns" {
    log_time "Starting test-12"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-12.txt")
    verify_size "/tmp/test-12.txt" 736547
    log_time "test-12 took $duration seconds"
    log_time "Ending test-12"
}

@test "test-13: Include record ID with JSON patterns" {
    log_time "Starting test-13"
    duration=$(measure_time "${APP} -I ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-13.txt")
    verify_size "/tmp/test-13.txt" 19515
    log_time "test-13 took $duration seconds"
    log_time "Ending test-13"
}

@test "test-14: Include record ID with inverted match and JSON patterns" {
    log_time "Starting test-14"
    duration=$(measure_time "${APP} -I ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-14.txt")
    verify_size "/tmp/test-14.txt" 901271
    log_time "test-14 took $duration seconds"
    log_time "Ending test-14"
}

@test "test-15: Full FASTQ record output with JSON patterns" {
    log_time "Starting test-15"
    duration=$(measure_time "${APP} -R ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-15.txt")
    verify_size "/tmp/test-15.txt" 35574
    log_time "test-15 took $duration seconds"
    log_time "Ending test-15"
}

@test "test-16: Full FASTQ record output with inverted match and JSON patterns" {
    log_time "Starting test-16"
    duration=$(measure_time "${APP} -R ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-16.txt")
    verify_size "/tmp/test-16.txt" 1642712
    log_time "test-16 took $duration seconds"
    log_time "Ending test-16"
}

@test "test-17: Count matches with JSON patterns" {
    log_time "Starting test-17"
    duration=$(measure_time "${APP} -c ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-17.txt")
    verify_count "/tmp/test-17.txt" 53
    log_time "test-17 took $duration seconds"
    log_time "Ending test-17"
}

@test "test-18: Count inverted matches with JSON patterns" {
    log_time "Starting test-18"
    duration=$(measure_time "${APP} -c ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-18.txt")
    verify_count "/tmp/test-18.txt" 2447
    log_time "test-18 took $duration seconds"
    log_time "Ending test-18"
}

@test "test-19: Tune command with count and JSON patterns" {
    log_time "Starting test-19"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small.fastq tune -n 2000 -c > /tmp/test-19.txt")
    verify_size "/tmp/test-19.txt" 310
    log_time "test-19 took $duration seconds"
    log_time "Ending test-19"
}

@test "test-20: Tune command with JSON output and JSON patterns" {
    log_time "Starting test-20"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/small-copy.fastq.gz tune -n 2000 -c --names --json-matches")
    verify_size "matches.json" 3704
    log_time "test-20 took $duration seconds"
    log_time "Ending test-20"
}

@test "test-21: Basic sequence match with IUPAC patterns" {
    log_time "Starting test-21"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-21.txt")
    verify_size "/tmp/test-21.txt" 15953
    log_time "test-21 took $duration seconds"
    log_time "Ending test-21"
}

@test "test-22: Inverted sequence match with IUPAC patterns" {
    log_time "Starting test-22"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-22.txt")
    verify_size "/tmp/test-22.txt" 736547
    log_time "test-22 took $duration seconds"
    log_time "Ending test-22"
}

@test "test-23: Include record ID with IUPAC patterns" {
    log_time "Starting test-23"
    duration=$(measure_time "${APP} -I ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-23.txt")
    verify_size "/tmp/test-23.txt" 19515
    log_time "test-23 took $duration seconds"
    log_time "Ending test-23"
}

@test "test-24: Include record ID with inverted match and IUPAC patterns" {
    log_time "Starting test-24"
    duration=$(measure_time "${APP} -I ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-24.txt")
    verify_size "/tmp/test-24.txt" 901271
    log_time "test-24 took $duration seconds"
    log_time "Ending test-24"
}

@test "test-25: Full FASTQ record output with IUPAC patterns" {
    log_time "Starting test-25"
    duration=$(measure_time "${APP} -R ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-25.txt")
    verify_size "/tmp/test-25.txt" 35574
    log_time "test-25 took $duration seconds"
    log_time "Ending test-25"
}

@test "test-26: Full FASTQ record output with inverted match and IUPAC patterns" {
    log_time "Starting test-26"
    duration=$(measure_time "${APP} -R ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-26.txt")
    verify_size "/tmp/test-26.txt" 1642712
    log_time "test-26 took $duration seconds"
    log_time "Ending test-26"
}

@test "test-27: Count matches with IUPAC patterns" {
    log_time "Starting test-27"
    duration=$(measure_time "${APP} -c ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-27.txt")
    verify_count "/tmp/test-27.txt" 53
    log_time "test-27 took $duration seconds"
    log_time "Ending test-27"
}

@test "test-28: Count inverted matches with IUPAC patterns" {
    log_time "Starting test-28"
    duration=$(measure_time "${APP} -c ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-28.txt")
    verify_count "/tmp/test-28.txt" 2447
    log_time "test-28 took $duration seconds"
    log_time "Ending test-28"
}

@test "test-29: Tune command with count and IUPAC patterns" {
    log_time "Starting test-29"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small.fastq tune -n 2000 -c > /tmp/test-29.txt")
    verify_size "/tmp/test-29.txt" 193
    log_time "test-29 took $duration seconds"
    log_time "Ending test-29"
}

@test "test-30: Tune command with JSON output and IUPAC patterns" {
    log_time "Starting test-30"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/small-copy.fastq.gz tune -n 2000 -c --names --json-matches")
    verify_size "matches.json" 3493
    log_time "test-30 took $duration seconds"
    log_time "Ending test-30"
}

@test "test-31: Basic sequence match with IUPAC and predicates" {
    log_time "Starting test-31"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-31.txt")
    verify_size "/tmp/test-31.txt" 8127
    log_time "test-31 took $duration seconds"
    log_time "Ending test-31"
}

@test "test-32: Inverted sequence match with IUPAC and predicates" {
    log_time "Starting test-32"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-32.txt")
    verify_size "/tmp/test-32.txt" 445480
    log_time "test-32 took $duration seconds"
    log_time "Ending test-32"
}

@test "test-33: Include record ID with IUPAC and predicates" {
    log_time "Starting test-33"
    duration=$(measure_time "${APP} -I ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-33.txt")
    verify_size "/tmp/test-33.txt" 9944
    log_time "test-33 took $duration seconds"
    log_time "Ending test-33"
}

@test "test-34: Include record ID with inverted match and IUPAC and predicates" {
    log_time "Starting test-34"
    duration=$(measure_time "${APP} -I ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-34.txt")
    verify_size "/tmp/test-34.txt" 545164
    log_time "test-34 took $duration seconds"
    log_time "Ending test-34"
}

@test "test-35: Full FASTQ record output with IUPAC and predicates" {
    log_time "Starting test-35"
    duration=$(measure_time "${APP} -R ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-35.txt")
    verify_size "/tmp/test-35.txt" 18125
    log_time "test-35 took $duration seconds"
    log_time "Ending test-35"
}

@test "test-36: Full FASTQ record output with inverted match and IUPAC and predicates" {
    log_time "Starting test-36"
    duration=$(measure_time "${APP} -R ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-36.txt")
    verify_size "/tmp/test-36.txt" 993604
    log_time "test-36 took $duration seconds"
    log_time "Ending test-36"
}

@test "test-37: Count matches with IUPAC and predicates" {
    log_time "Starting test-37"
    duration=$(measure_time "${APP} -c ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq > /tmp/test-37.txt")
    verify_count "/tmp/test-37.txt" 27
    log_time "test-37 took $duration seconds"
    log_time "Ending test-37"
}

@test "test-38: Count inverted matches with IUPAC and predicates" {
    log_time "Starting test-38"
    duration=$(measure_time "${APP} -c ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq inverted > /tmp/test-38.txt")
    verify_count "/tmp/test-38.txt" 1480
    log_time "test-38 took $duration seconds"
    log_time "Ending test-38"
}

@test "test-39: Tune command with count and IUPAC and predicates" {
    log_time "Starting test-39"
    duration=$(measure_time "${APP} ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small.fastq tune -n 2000 -c > /tmp/test-39.txt")
    verify_size "/tmp/test-39.txt" 176
    log_time "test-39 took $duration seconds"
    log_time "Ending test-39"
}

@test "test-40: Tune command with JSON output and IUPAC and predicates" {
    log_time "Starting test-40"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-iupac-and-predicates.json ${EXAMPLES_DIR}/small-copy.fastq.gz tune -n 2000 -c --names --json-matches")
    verify_size "matches.json" 3437
    log_time "test-40 took $duration seconds"
    log_time "Ending test-40"
}

@test "test-41: Bucket flag with gzipped input" {
    log_time "Starting test-41"
    duration=$(measure_time "${APP} -R --bucket --read-gzip ${EXAMPLES_DIR}/16S-iupac.json ${EXAMPLES_DIR}/SRX26365298.fastq.gz")
    verify_bucket_files
    log_time "test-41 took $duration seconds"
    log_time "Ending test-41"
}

@test "test-42: Tune command with JSON output - large dataset" {
    log_time "Starting test-42"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/SRX26365298.fastq.gz tune -n 10000000 -c --names --json-matches")
    verify_size "matches.json" 4827
    log_time "test-42 took $duration seconds"
    log_time "Ending test-42"
}

@test "test-43: Summarise command with JSON output - large dataset" {
    log_time "Starting test-43"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/SRX26365298.fastq.gz summarise -c --names --json-matches")
    verify_size "matches.json" 4827
    log_time "test-43 took $duration seconds"
    log_time "Ending test-43"
}

@test "test-44: Tune command with JSON output and 3 variants - large dataset" {
    log_time "Starting test-44"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/SRX26365298.fastq.gz tune -n 10000000 -c --names --json-matches --variants 3")
    verify_size "matches.json" 7527
    log_time "test-44 took $duration seconds"
    log_time "Ending test-44"
}

@test "test-45: Summarise command with JSON output and 3 variants - large dataset" {
    log_time "Starting test-45"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/SRX26365298.fastq.gz summarise -c --names --json-matches --variants 3")
    verify_size "matches.json" 7527
    log_time "test-45 took $duration seconds"
    log_time "Ending test-45"
}

@test "test-46: Tune command with JSON output and all variants - large dataset" {
    log_time "Starting test-46"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/SRX26365298.fastq.gz tune -n 10000000 -c --names --json-matches --all")
    verify_size "matches.json" 17995
    log_time "test-46 took $duration seconds"
    log_time "Ending test-46"
}

@test "test-47: Summarise command with JSON output and all variants - large dataset" {
    log_time "Starting test-47"
    duration=$(measure_time "${APP} --read-gzip ${EXAMPLES_DIR}/16S-no-iupac.json ${EXAMPLES_DIR}/SRX26365298.fastq.gz summarise -c --names --json-matches --all")
    verify_size "matches.json" 17995
    log_time "test-47 took $duration seconds"
    log_time "Ending test-47"
}
