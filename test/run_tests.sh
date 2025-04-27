#!/bin/bash

# Default configuration
APP="${SCRIPT_DIR}../target/release/grepq"
COMPUTE_TIMINGS=false
SETUP_ONLY=false
REMAINING_ARGS=()
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

install_package() {
    local package=$1
    case "$(uname -s)" in
    Darwin*)
        if ! brew list "$package" &>/dev/null; then
            echo "Installing $package via Homebrew..."
            brew install "$package"
        fi
        ;;
    Linux*)
        if command -v apt-get >/dev/null 2>&1; then
            echo "Installing $package via apt..."
            sudo apt-get update && sudo apt-get install -y "$package"
        elif command -v yum >/dev/null 2>&1; then
            echo "Installing $package via yum..."
            sudo yum install -y "$package"
        elif command -v dnf >/dev/null 2>&1; then
            echo "Installing $package via dnf..."
            sudo dnf install -y "$package"
        else
            echo "Error: Unsupported package manager. Please install $package manually."
            exit 1
        fi
        ;;
    *)
        echo "Error: Unsupported operating system"
        exit 1
        ;;
    esac
}

setup_test_environment() {
    echo "Setting up test environment..."

    # Install BATS if needed
    if ! command -v bats >/dev/null 2>&1; then
        echo "Installing BATS..."
        install_package bats-core
    fi

    # Install hyperfine if needed
    if ! command -v hyperfine >/dev/null 2>&1; then
        echo "Installing hyperfine..."
        install_package hyperfine
    fi

    # Create lib directory
    mkdir -p "${SCRIPT_DIR}/lib"
    cd "${SCRIPT_DIR}/lib" || exit 1

    # Setup bats-support
    if [ ! -d "bats-support" ]; then
        git clone https://github.com/bats-core/bats-support.git
    else
        (cd bats-support && git pull)
    fi

    # Setup bats-assert
    if [ ! -d "bats-assert" ]; then
        git clone https://github.com/bats-core/bats-assert.git
    else
        (cd bats-assert && git pull)
    fi

    # Create test helper if it doesn't exist
    cat >"${SCRIPT_DIR}/test_helper.bash" <<'EOF'
BATS_TEST_DIRNAME="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
load "${BATS_TEST_DIRNAME}/lib/bats-support/load"
load "${BATS_TEST_DIRNAME}/lib/bats-assert/load"
EOF

    echo "Test environment setup complete!"
}

measure_time() {
    local command="$1"
    local result
    if [ "$COMPUTE_TIMINGS" = true ]; then
        result=$(hyperfine --warmup 1 --runs 3 --export-json /tmp/hyperfine.json "$command")
        jq '.results[0].mean' /tmp/hyperfine.json
    else
        result=$($command)
        echo 0 # Return 0 as default duration
    fi
}

# Process command line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
    --setup)
        SETUP_ONLY=true
        ;;
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

# Run setup if requested
if [ "$SETUP_ONLY" = true ]; then
    setup_test_environment
    exit 0
fi

# Always ensure test environment is ready
if [ ! -d "${SCRIPT_DIR}/lib/bats-support" ] || [ ! -d "${SCRIPT_DIR}/lib/bats-assert" ]; then
    setup_test_environment
fi

# Export variables for the tests
export APP
export COMPUTE_TIMINGS

# Base bats command
BATS_CMD=("bats" "${SCRIPT_DIR}/test.bats")

# Detect OS and add filter if Linux
OS_TYPE=$(uname -s)
if [ "$OS_TYPE" = "Linux" ]; then
    # Regex to match tests 1-40 and 48+
    LINUX_FILTER='^test-(([1-9]|[1-3][0-9]|40)|(4[8-9]|[5-9][0-9]|[1-9][0-9]{2,}))$'
    BATS_CMD+=("--filter" "$LINUX_FILTER")
    echo "Running on Linux, excluding tests 41-47."
fi

# Add any remaining arguments passed to the script
BATS_CMD+=("${REMAINING_ARGS[@]}")

# Run the tests
echo "Executing: ${BATS_CMD[*]}"
exec "${BATS_CMD[@]}"
