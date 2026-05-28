# Welcome to the KairoDB Wiki

KairoDB is a terminal-first database tool. It reads, writes, and manages databases using plain text.

## v0.4.0

This release introduces native multi-database support (PostgreSQL and SQLite), elegant format mismatch error logging, and standalone installation support so developers can run Kairo DB globally without requiring a local Rust environment.

## v0.3.2

This version improves error handling across the CLI. Commands like `read` and `export` now validate file paths before attempting a database connection, and return clear, human-readable messages when something goes wrong. No more raw stack traces.

## v0.3.1

This version adds the ability to read and export any existing database file into human-readable format.

## Pages

- [Getting Started](Getting-Started)
- [Commands](Commands)
- [Schema Syntax](Schemas)
- [Query Language](Queries)
- [Architecture](Architecture)
- [Philosophy](Philosophy)
