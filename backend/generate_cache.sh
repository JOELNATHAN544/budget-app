#!/usr/bin/env bash
set -e

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
while ! nc -z postgres 5432; do
  echo "Waiting for PostgreSQL..."
  sleep 2
done
echo "PostgreSQL is ready!"

# Create database and run migrations
echo "Creating database..."
cargo sqlx database create || true

echo "Running migrations..."
cargo sqlx migrate run

# Generate the query cache
echo "Generating query cache..."
cargo sqlx prepare -- --lib 