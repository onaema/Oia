# Dockerfile for Backend (Rust)
FROM rust:1.69 as builder

WORKDIR /app
COPY . .

# Build the Rust application
RUN cargo build --release

# Create a minimal image for running the application
FROM debian:buster-slim
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/oia /app/oia

# Expose the port the application runs on
EXPOSE 3030

# Run the application
CMD ["/app/oia"]