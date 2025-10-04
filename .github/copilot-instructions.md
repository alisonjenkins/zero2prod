# GitHub Copilot Instructions for zero2prod

## Project Overview

This is a Rust-based email newsletter web application built following the "Zero to Production in Rust" book. The project demonstrates production-ready practices for building web services with Rust, including proper error handling, testing, and database integration.

## Tech Stack

- **Language**: Rust (Edition 2021)
- **Web Framework**: Actix-web 4
- **Database**: PostgreSQL with SQLx for compile-time verified queries
- **Async Runtime**: Tokio
- **Error Handling**: `thiserror` for custom errors, `color-eyre` for enhanced error reporting
- **Configuration**: `config` crate for environment-based settings
- **Testing**: Built-in Rust testing with `sqlx::test` for database integration tests

## Development Setup

### Prerequisites
- Rust toolchain (stable)
- PostgreSQL 17
- SQLx CLI for database migrations
- Docker/Podman for local database setup

### Key Commands
- `cargo fmt` - Format code (enforced in CI)
- `cargo clippy -- -D warnings` - Lint code (warnings as errors in CI)
- `cargo test` - Run all tests (requires database)
- `cargo build` - Build the project
- `./scripts/init_db.sh` - Initialize local PostgreSQL database

### Database Setup
- Run `./scripts/init_db.sh` to start a PostgreSQL container and run migrations
- Configuration in `configuration.yaml`
- Migrations in `migrations/` directory
- SQLx compile-time query verification is enabled

## Architecture

### Project Structure
- `src/main.rs` - Application entry point
- `src/lib.rs` - Library root exposing modules
- `src/startup.rs` - Server initialization and configuration
- `src/routes/` - HTTP route handlers
  - `health_check.rs` - Health check endpoint
  - `subscriptions.rs` - Newsletter subscription handling
- `src/configuration.rs` - Configuration management
- `src/error.rs` - Custom error types using `thiserror`
- `tests/` - Integration tests

### API Endpoints
- `GET /health_check` - Returns 200 OK with empty body
- `POST /subscriptions` - Accepts form data (name, email) to create subscription

### Database Schema
- **subscriptions** table:
  - `id` (uuid, primary key)
  - `email` (text, unique, not null)
  - `name` (text, not null)
  - `subscribed_at` (timestamptz, not null)

## Coding Standards

### Error Handling
- Use `thiserror::Error` for custom error types with descriptive messages
- Errors should include context via the `source` field
- Use `.map_err()` to convert errors to domain-specific types
- Example pattern from `src/error.rs`:
  ```rust
  #[derive(Error, Debug)]
  pub enum AppErr {
      #[error("Error while connecting to Postgres: {source}")]
      PostgresConnection { source: sqlx::Error },
  }
  ```

### Testing
- Use `#[sqlx::test]` for integration tests that need database access
- Tests automatically get a fresh database pool
- Use `color_eyre::Result` as return type in tests
- Keep test helpers in test modules (e.g., `spawn_app` in `tests/health_check.rs`)
- Test both success and error cases (e.g., 200 responses and 400 validation errors)

### Database Queries
- Use SQLx macros (`sqlx::query!`) for compile-time verified queries
- Queries are checked against the database schema at compile time
- Handle query errors explicitly with pattern matching

### Configuration
- Environment-specific settings in `configuration.yaml`
- Use serde for deserialization
- Validate configuration at startup

### Code Style
- Follow Rust standard formatting (enforced by `cargo fmt`)
- Pass Clippy lints without warnings (CI enforces this)
- Use descriptive variable and function names
- Add module-level documentation for public APIs

## CI/CD

The project uses GitHub Actions for continuous integration:
- **Test**: Runs all tests with PostgreSQL service
- **Fmt**: Ensures code is properly formatted
- **Clippy**: Lints code with warnings treated as errors
- **Coverage**: Generates code coverage reports with tarpaulin

All CI jobs must pass before merging pull requests.

## Nix Development Environment

The project includes a Nix flake (`flake.nix`) for reproducible development environments:
- Rust toolchain with specific targets (x86_64 and aarch64 Linux musl)
- All required build tools and dependencies
- Use `direnv` with `.envrc` to auto-load the environment

## Important Patterns

### Dependency Injection
- Database connection pool is passed via `web::Data<PgPool>` in Actix-web
- Server is configured with dependency injection pattern in `startup.rs`

### Async/Await
- All HTTP handlers and database operations are async
- Main function uses `#[tokio::main]` attribute

### Type Safety
- Strong typing throughout the application
- Use of newtype patterns where appropriate (e.g., UUIDs for IDs)

## When Contributing

1. Ensure code is formatted: `cargo fmt`
2. Fix all Clippy warnings: `cargo clippy -- -D warnings`
3. Run tests locally: `cargo test` (requires PostgreSQL)
4. Follow existing error handling patterns
5. Add tests for new functionality
6. Update this file if adding new patterns or architecture changes
