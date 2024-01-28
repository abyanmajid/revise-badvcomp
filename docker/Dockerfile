# Build stage
FROM rust:latest as builder

# Set the working directory inside the container to /usr/src/myapp
WORKDIR /usr/src/myapp

# Copy the Rust project files from the src/server directory relative to the Dockerfile's location
COPY ../server/ .

# Build the application
RUN cargo build --release

# Production stage
FROM ubuntu

# Set the working directory inside the container to /app
WORKDIR /app

# Copy the compiled binary from the builder stage into the production image
COPY --from=builder /usr/src/myapp/target/release/revise-badvcomp .

# Command to run the binary
CMD ["./revise-badvcomp", "serve"]
