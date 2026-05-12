# Architecture

KairoDB is built as a modular system in Rust.

## Core Components

### 1. Parser (Pest)
The parser reads `.kairo` files and translates them into an internal Abstract Syntax Tree (AST). It handles the custom grammar defined in `kairo.pest`.

### 2. Schema Engine
The schema engine takes the AST and translates it into database-specific SQL. It manages type mapping and default values.

### 3. Adapter Layer
The adapter layer defines a unified trait (`Adapter`) that all database backends must implement. This ensures that the core logic remains database-agnostic.
- **SQLite Adapter**: Uses `sqlx` to interface with local `.db` files.
- **Postgres Adapter**: Uses `sqlx` to connect to remote PostgreSQL servers.

### 4. CLI Engine (Clap)
The CLI engine manages commands, arguments, and terminal interaction. It is designed to be minimal and fast.

## Project Structure
```txt
src/
 ├── main.rs            # Entry point & Command routing
 ├── core/              # Business logic (Parser, Schema, Query)
 ├── adapters/          # Database implementations
 └── ui.rs              # Terminal output formatting
```
