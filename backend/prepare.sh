#!/usr/bin/env bash
set -e

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
while ! nc -z postgres 5432; do
  sleep 1
done
echo "PostgreSQL is ready!"

# Run migrations
cargo sqlx database create
cargo sqlx migrate run

# Prepare SQLx cache
cargo sqlx prepare -- --lib

# Start the application
exec ./budget-app 