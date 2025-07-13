#!/bin/bash

echo "Testing database connection..."

# Try to connect to PostgreSQL
PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -c "SELECT version();" 2>/dev/null

if [ $? -eq 0 ]; then
    echo "✅ Database connection successful!"
    
    # Create the database if it doesn't exist
    echo "Creating database 'budget_app' if it doesn't exist..."
    PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -c "CREATE DATABASE budget_app;" 2>/dev/null
    
    if [ $? -eq 0 ]; then
        echo "✅ Database 'budget_app' created successfully!"
    else
        echo "ℹ️  Database 'budget_app' might already exist"
    fi
    
    # Test connection to the specific database
    echo "Testing connection to 'budget_app' database..."
    PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d budget_app -c "SELECT current_database();"
    
else
    echo "❌ Database connection failed!"
    echo "Please check if PostgreSQL is running and accessible on localhost:5432"
fi 