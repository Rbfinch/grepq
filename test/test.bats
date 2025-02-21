#!/usr/bin/env bats

setup() {
    # Get absolute paths
    REPO_ROOT="$( cd "$( dirname "${BATS_TEST_FILENAME}" )/.." && pwd )"
    APP="${REPO_ROOT}/target/release/grepq"
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

@test "test-1: Basic sequence match" {
    "${APP}" "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-1.txt
    verify_size "/tmp/test-1.txt" 15953
}

@test "test-2: Inverted sequence match" {
    "${APP}" "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-2.txt
    verify_size "/tmp/test-2.txt" 736547
}

@test "test-3: Include record ID" {
    "${APP}" -I "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-3.txt
    verify_size "/tmp/test-3.txt" 19515
}

@test "test-4: Include record ID with inverted match" {
    "${APP}" -I "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-4.txt
    verify_size "/tmp/test-4.txt" 901271
}

@test "test-5: Full FASTQ record output" {
    "${APP}" -R "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-5.txt
    verify_size "/tmp/test-5.txt" 35574
}

@test "test-6: Full FASTQ record output with inverted match" {
    "${APP}" -R "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-6.txt
    verify_size "/tmp/test-6.txt" 1642712
}

@test "test-7: Count matches" {
    "${APP}" -c "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-7.txt
    verify_count "/tmp/test-7.txt" 53
}

@test "test-8: Count inverted matches" {
    "${APP}" -c "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-8.txt
    verify_count "/tmp/test-8.txt" 2447
}

@test "test-9: Tune command with count" {
    "${APP}" "${EXAMPLES_DIR}/16S-no-iupac.txt" "${EXAMPLES_DIR}/small.fastq" tune -n 2000 -c > /tmp/test-9.txt
    verify_size "/tmp/test-9.txt" 310
}

@test "test-10: Tune command with JSON output" {
    "${APP}" --read-gzip "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small-copy.fastq.gz" tune -n 2000 -c --names --json-matches
    verify_size "matches.json" 3704
}

@test "test-11: Basic sequence match with JSON patterns" {
    "${APP}" "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-11.txt
    verify_size "/tmp/test-11.txt" 15953
}

@test "test-12: Inverted sequence match with JSON patterns" {
    "${APP}" "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-12.txt
    verify_size "/tmp/test-12.txt" 736547
}

@test "test-13: Include record ID with JSON patterns" {
    "${APP}" -I "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-13.txt
    verify_size "/tmp/test-13.txt" 19515
}

@test "test-14: Include record ID with inverted match and JSON patterns" {
    "${APP}" -I "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-14.txt
    verify_size "/tmp/test-14.txt" 901271
}

@test "test-15: Full FASTQ record output with JSON patterns" {
    "${APP}" -R "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-15.txt
    verify_size "/tmp/test-15.txt" 35574
}

@test "test-16: Full FASTQ record output with inverted match and JSON patterns" {
    "${APP}" -R "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-16.txt
    verify_size "/tmp/test-16.txt" 1642712
}

@test "test-17: Count matches with JSON patterns" {
    "${APP}" -c "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-17.txt
    verify_count "/tmp/test-17.txt" 53
}

@test "test-18: Count inverted matches with JSON patterns" {
    "${APP}" -c "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-18.txt
    verify_count "/tmp/test-18.txt" 2447
}

@test "test-19: Tune command with count and JSON patterns" {
    "${APP}" "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small.fastq" tune -n 2000 -c > /tmp/test-19.txt
    verify_size "/tmp/test-19.txt" 310
}

@test "test-20: Tune command with JSON output and JSON patterns" {
    "${APP}" --read-gzip "${EXAMPLES_DIR}/16S-no-iupac.json" "${EXAMPLES_DIR}/small-copy.fastq.gz" tune -n 2000 -c --names --json-matches
    verify_size "matches.json" 3704
}

@test "test-21: Basic sequence match with IUPAC patterns" {
    "${APP}" "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-21.txt
    verify_size "/tmp/test-21.txt" 15953
}

@test "test-22: Inverted sequence match with IUPAC patterns" {
    "${APP}" "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-22.txt
    verify_size "/tmp/test-22.txt" 736547
}

@test "test-23: Include record ID with IUPAC patterns" {
    "${APP}" -I "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-23.txt
    verify_size "/tmp/test-23.txt" 19515
}

@test "test-24: Include record ID with inverted match and IUPAC patterns" {
    "${APP}" -I "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-24.txt
    verify_size "/tmp/test-24.txt" 901271
}

@test "test-25: Full FASTQ record output with IUPAC patterns" {
    "${APP}" -R "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-25.txt
    verify_size "/tmp/test-25.txt" 35574
}

@test "test-26: Full FASTQ record output with inverted match and IUPAC patterns" {
    "${APP}" -R "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-26.txt
    verify_size "/tmp/test-26.txt" 1642712
}

@test "test-27: Count matches with IUPAC patterns" {
    "${APP}" -c "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-27.txt
    verify_count "/tmp/test-27.txt" 53
}

@test "test-28: Count inverted matches with IUPAC patterns" {
    "${APP}" -c "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-28.txt
    verify_count "/tmp/test-28.txt" 2447
}

@test "test-29: Tune command with count and IUPAC patterns" {
    "${APP}" "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small.fastq" tune -n 2000 -c > /tmp/test-29.txt
    verify_size "/tmp/test-29.txt" 193
}

@test "test-30: Tune command with JSON output and IUPAC patterns" {
    "${APP}" --read-gzip "${EXAMPLES_DIR}/16S-iupac.json" "${EXAMPLES_DIR}/small-copy.fastq.gz" tune -n 2000 -c --names --json-matches
    verify_size "matches.json" 3493
}

@test "test-31: Basic sequence match with IUPAC and predicates" {
    "${APP}" "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-31.txt
    verify_size "/tmp/test-31.txt" 8127
}

@test "test-32: Inverted sequence match with IUPAC and predicates" {
    "${APP}" "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-32.txt
    verify_size "/tmp/test-32.txt" 445480
}

@test "test-33: Include record ID with IUPAC and predicates" {
    "${APP}" -I "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-33.txt
    verify_size "/tmp/test-33.txt" 9944
}

@test "test-34: Include record ID with inverted match and IUPAC and predicates" {
    "${APP}" -I "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-34.txt
    verify_size "/tmp/test-34.txt" 545164
}

@test "test-35: Full FASTQ record output with IUPAC and predicates" {
    "${APP}" -R "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-35.txt
    verify_size "/tmp/test-35.txt" 18125
}

@test "test-36: Full FASTQ record output with inverted match and IUPAC and predicates" {
    "${APP}" -R "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-36.txt
    verify_size "/tmp/test-36.txt" 993604
}

@test "test-37: Count matches with IUPAC and predicates" {
    "${APP}" -c "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" > /tmp/test-37.txt
    verify_count "/tmp/test-37.txt" 27
}

@test "test-38: Count inverted matches with IUPAC and predicates" {
    "${APP}" -c "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" inverted > /tmp/test-38.txt
    verify_count "/tmp/test-38.txt" 1480
}

@test "test-39: Tune command with count and IUPAC and predicates" {
    "${APP}" "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small.fastq" tune -n 2000 -c > /tmp/test-39.txt
    verify_size "/tmp/test-39.txt" 176
}

@test "test-40: Tune command with JSON output and IUPAC and predicates" {
    "${APP}" --read-gzip "${EXAMPLES_DIR}/16S-iupac-and-predicates.json" "${EXAMPLES_DIR}/small-copy.fastq.gz" tune -n 2000 -c --names --json-matches
    verify_size "matches.json" 3437
}
