


# Use the official Rust builder image
FROM rust:1.82.0-slim-bookworm as builder

# Install cmake and other dependencies (e.g., build-essential) in the builder image
RUN apt-get update && apt-get install -y cmake build-essential

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory into the container
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock


# Build the project in release mode
RUN cargo build --release

COPY config config

# Use a minimal base image for the final output
FROM debian:trixie-slim

RUN mkdir -p /usr/local/bin/config
# RUN mkdir -p ./app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/easy-proxy /usr/local/bin/easy-proxy

# Copy the config files from the ./config directory
COPY --from=builder /usr/src/app/config /usr/local/bin/config/

# Expose port 8080 to the host
EXPOSE 8080

# Set the environment variable to point to the config file
ENV EASY_PROXY_CONF=/usr/local/bin/config/global.yaml

# Set the default command
CMD ["bash"]