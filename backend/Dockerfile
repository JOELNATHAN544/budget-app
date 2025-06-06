# Use nightly Rust toolchain
FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/app
COPY . .

# Build the application with offline mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates netcat-traditional && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/budget-app .

# Copy the .env file and prepare script
COPY --from=builder /usr/src/app/.env .
COPY --from=builder /usr/src/app/prepare.sh .

# Make prepare script executable
RUN chmod +x prepare.sh

EXPOSE 8000

# Start the application
CMD ["./budget-app"] 