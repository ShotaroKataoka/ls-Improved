# Use the official Rust image from the Docker Hub
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the `Cargo.toml` and `Cargo.lock` files to the working directory
COPY Cargo.toml Cargo.lock ./

# This command will create a dummy application in order to prefetch dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Pre-build dependencies to leverage caching
RUN cargo build --release

# Remove the dummy application
RUN rm -r src

# Copy the source code to the working directory
COPY . .

# Build the actual application
RUN cargo build --release

# Command to run the executable
CMD ["cargo", "run", "--release"]