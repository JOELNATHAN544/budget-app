<<<<<<< HEAD
# budget-app
=======
# Budget Tracker App

A full-stack budget tracking application built with Rust (Actix-web) and React.

## Project Structure

- `backend/`: Rust backend using Actix-web and PostgreSQL
- `frontend/`: React frontend with TypeScript

## Prerequisites

- Rust and Cargo
- Node.js and npm
- PostgreSQL
- Docker (optional)

## Setup

### Backend

1. Navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Create a `.env` file:
   ```
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/budget_app
   RUST_LOG=debug
   RUST_BACKTRACE=1
   ```

3. Create the database:
   ```bash
   createdb budget_app
   ```

4. Run migrations:
   ```bash
   cargo sqlx database create
   cargo sqlx migrate run
   ```

5. Start the backend:
   ```bash
   cargo run
   ```

The backend will run on http://localhost:8000

### Frontend

1. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Start the development server:
   ```bash
   npm start
   ```

The frontend will run on http://localhost:3000

## API Endpoints

- `GET /api/transactions`: Get all transactions
- `POST /api/transactions`: Create a new transaction
- `DELETE /api/transactions/{id}`: Delete a transaction

## Development

### Backend

- The backend uses SQLx for database operations
- Run `cargo sqlx prepare` to prepare SQL queries for offline mode
- Use `cargo test` to run tests

### Frontend

- Built with React and TypeScript
- Uses modern React features and hooks
- Styled with CSS modules

## License

MIT 
>>>>>>> 0479bcc (Initial commit: add backend and frontend)
