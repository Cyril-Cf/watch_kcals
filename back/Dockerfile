FROM rust:1.66

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y libpq-dev

ENV CARGO_TARGET_DIR=/tmp/target

RUN cargo install diesel_cli --no-default-features --features "postgres"
RUN cargo install cargo-watch