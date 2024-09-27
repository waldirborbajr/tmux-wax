# Use an official Rust image as a base
FROM rust:latest AS builder

# Set working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build the Rust project
RUN cargo install --path .

# Use a smaller base image for the final build
FROM debian:buster-slim

# Set working directory in the new image
WORKDIR /usr/local/bin

# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/tmux-wax /usr/local/bin/tmux-wax

# Install dependencies (e.g., SSH, Docker CLI)
RUN apt-get update && apt-get install -y \
    openssh-client \
    docker.io \
    && rm -rf /var/lib/apt/lists/*

# Expose any necessary ports
EXPOSE 8080

# Set the entrypoint command
ENTRYPOINT ["tmux-wax"]

