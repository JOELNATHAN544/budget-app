#!/usr/bin/env bash
set -e

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Build the application with offline mode
SQLX_OFFLINE=true cargo build --release 