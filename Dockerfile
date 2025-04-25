FROM ubuntu:24.10

RUN apt update -y && apt install rustup build-essential cmake libsqlite3-dev -y

RUN rustup default stable

RUN cargo install grepq

ENTRYPOINT ["/root/.cargo/bin/grepq"]
