#!/bin/bash

echo "Setting up PostgreSQL database for Budget App..."

# Stop any existing containers
echo "Stopping existing containers..."
docker compose down 2>/dev/null || true

# Remove existing volumes
echo "Removing existing volumes..."
docker volume rm ubuntu_postgres_data 2>/dev/null || true

# Create PostgreSQL container manually (since docker-compose has issues)
echo "Creating PostgreSQL container..."
docker run --name budget-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=budget_app \
  -p 5432:5432 \
  -d postgres:15

# Wait for PostgreSQL to start
echo "Waiting for PostgreSQL to start..."
sleep 5

# Test connection
echo "Testing database connection..."
PGPASSWORD=postgres psql -h 10.72.220.223 -p 5432 -U postgres -d budget_app -c "SELECT version();" 2>/dev/null

if [ $? -eq 0 ]; then
    echo "✅ Database setup successful!"
    echo "✅ PostgreSQL is running on 10.72.220.223:5432"
    echo "✅ Database 'budget_app' is ready"
    echo ""
    echo "Next steps:"
    echo "1. Run database migrations: cd backend && cargo sqlx migrate run"
    echo "2. Start the backend: cargo run"
    echo "3. Start the frontend: cd frontend && npm start"
else
    echo "❌ Database setup failed!"
fi 