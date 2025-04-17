# Use the specified Rust version and base OS as the base image
ARG BASE_OS
ARG RUST_VERSION
FROM rust:${RUST_VERSION}-${BASE_OS}

## Enable automatic code formatting for Rust (needed by example during entities generation with sea-orm)
RUN ["rustup", "component", "add", "rustfmt"]

# binstall allows downloading only binaries (avoiding time spent downloading and compiling from source)
RUN ["cargo", "install", "cargo-binstall", "--locked"]

# cargo-llvm-cov: generates test reports and code coverage
RUN ["cargo", "binstall", "cargo-llvm-cov", "--secure", "--no-confirm"]
RUN ["rustup", "component", "add", "llvm-tools-preview", "--toolchain", "1.85.1-x86_64-unknown-linux-gnu"]
# cargo-nextest: improves the native `cargo test` by speeding up test execution
RUN ["cargo", "binstall", "cargo-nextest", "--secure", "--no-confirm"]
# cargo-mutants: allows running mutation tests
RUN ["cargo", "binstall", "cargo-mutants", "--secure", "--no-confirm"]

# create container user with same rights than host user

RUN groupadd -g 1000 appuser && \
    useradd -m -u 1000 -g 1000 -s /bin/bash appuser

# create a cargo local cache
ENV CARGO_HOME=/cargo
RUN mkdir -p $CARGO_HOME && chown -R appuser:appuser $CARGO_HOME

# global configuration done ! let's focus on the project itself

WORKDIR /app
RUN chown -R appuser:appuser /app

USER appuser