FROM rust:latest

# Install common dependencies for Rust development
RUN apt update && apt install -y libssl-dev pkg-config cargo-watch

# Set working directory
WORKDIR /usr/src/app

# Copy Cargo files first for caching dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copy the source code
COPY . .

# Set permissions
RUN chmod -R 777 /usr/src/app

# Default command for development (auto-rebuild on file changes)
CMD ["cargo", "watch", "-x", "run"]