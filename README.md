# KairoDB

Human-readable databases. Minimal. Fast. Local-first.

## What is it

KairoDB is a terminal tool that lets you define database schemas in plain text, apply them to SQLite or PostgreSQL, and read any existing database file in a human-readable format. No ORM. No bloat. Just a clean interface between you and your data.

## Install

Download the binary for your OS from the [Releases](https://github.com/lnfiniware/kairo/releases) page. No Rust required.

Or build from source:
```
git clone https://github.com/lnfiniware/kairo.git
cd kairo
cargo install --path .
```

## Usage

```
kairo init               set up a new project
kairo create <name>      apply a .kairo schema to your database
kairo query <sql>        run a query (supports natural syntax)
kairo read <file.db>     inspect any database file
kairo export <file.db>   convert a database to .kairo schema format
kairo tables             list all tables
kairo status             show project info
```

## Schema syntax

```
table users {
  name: string
  age: int
  active: bool = true
}
```

Save as `schema/users.kairo`, then run `kairo create users`.

## Read any database

```
$ kairo read myapp.db

Database: myapp.db
----------------------------------------
Tables: users, posts

table users {
  id: int
  name: string
  email: string
}

  -- 42 rows
  | id: 1 | name: alice | email: alice@example.com
  | id: 2 | name: bob | email: bob@example.com
  ...
```

## Website

https://kairo.infiniware.bid

## License

GNU GPL 3.0

MADE BY INFINIWARE
