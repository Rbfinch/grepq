#!/bin/bash

# Install BATS if not already installed
if ! command -v bats >/dev/null 2>&1; then
    brew install bats-core
fi

# Create lib directory if it doesn't exist
mkdir -p "$(dirname "$0")/lib"
cd "$(dirname "$0")/lib" || exit 1

# Clone or update bats-support
if [ ! -d "bats-support" ]; then
    git clone https://github.com/bats-core/bats-support.git
else
    cd bats-support && git pull && cd ..
fi

# Clone or update bats-assert
if [ ! -d "bats-assert" ]; then
    git clone https://github.com/bats-core/bats-assert.git
else
    cd bats-assert && git pull && cd ..
fi

echo "BATS test environment setup complete!"
