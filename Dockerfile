FROM rust:latest

# Create a new directory for the Rust project
WORKDIR /app

# Copy the project files
COPY host.rs src/main.rs
COPY host.toml Cargo.toml
COPY host.html index.html
COPY image.jpg /app/static/image.jpg

RUN rustup default nightly

# Build the project
RUN cargo build --release

# Set the default command to run the compiled binary
CMD ["./target/release/host"]
