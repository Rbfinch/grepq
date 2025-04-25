FROM ubuntu:24.10 AS builder-amd64

RUN apt update -y && apt install -y \
    rustup \
    build-essential \
    cmake \
    libsqlite3-dev \
    pkg-config \
    libssl-dev \
    zlib1g-dev

RUN rustup default stable

# Create a directory for building
WORKDIR /app

# Copy your Cargo.toml and Cargo.lock (if it exists)
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs if it doesn't exist yet to cache dependencies
RUN mkdir -p src && echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs

# Now copy the real source code
COPY src/ ./src/
COPY benches/ ./benches/

# Build for x86_64
RUN cargo build --release

# ---

FROM ubuntu:24.10 AS builder-arm64

RUN apt update -y && apt install -y \
    rustup \
    build-essential \
    cmake \
    libsqlite3-dev \
    pkg-config \
    libssl-dev \
    zlib1g-dev

RUN rustup default stable
RUN rustup target add aarch64-unknown-linux-gnu

# Create a directory for building
WORKDIR /app

# Copy your Cargo.toml and Cargo.lock (if it exists)
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs if it doesn't exist yet to cache dependencies
RUN mkdir -p src && echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs

# Now copy the real source code
COPY src/ ./src/
COPY benches/ ./benches/

# Build for arm64
RUN cargo build --target aarch64-unknown-linux-gnu --release

# --- Final Image ---
FROM ubuntu:24.10

# Install necessary runtime dependencies
RUN apt update -y && apt install libsqlite3-0 -y

# Create a directory for the application
WORKDIR /app

# Copy the built binaries from the builder stages
COPY --from=builder-amd64 /app/target/release/grepq /usr/local/bin/grepq-amd64
COPY --from=builder-arm64 /app/target/aarch64-unknown-linux-gnu/release/grepq /usr/local/bin/grepq-arm64

# Create a script to run the correct binary based on architecture
RUN echo '#!/bin/sh' > /usr/local/bin/grepq
RUN echo 'case "$(uname -m)" in' >> /usr/local/bin/grepq
RUN echo '  x86_64) exec /usr/local/bin/grepq-amd64 "$@";;' >> /usr/local/bin/grepq
RUN echo '  aarch64) exec /usr/local/bin/grepq-arm64 "$@";;' >> /usr/local/bin/grepq
RUN echo '  *) echo "Unsupported architecture: $(uname -m)" 1>&2; exit 1;;' >> /usr/local/bin/grepq
RUN echo 'esac' >> /usr/local/bin/grepq
RUN chmod +x /usr/local/bin/grepq

ENTRYPOINT ["/usr/local/bin/grepq"]