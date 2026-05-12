# KairoDB

Human-readable databases. Minimal. Fast. Local-first.

KairoDB is a database abstraction system designed for humans. It bridges the gap between your data and your tools with a focus on clarity, simplicity, and developer happiness.

## Philosophy

*   **Human-first**: Code and schemas should be readable by people, not just machines.
*   **Minimal**: No bloated ORMs. No unnecessary abstractions.
*   **Git-friendly**: Version-control your database structures naturally.
*   **Local-first**: Work where you are, then scale where you need.
*   **Extensible**: Modular adapter system for any database.

## Installation

```bash
# Clone the repo
git clone https://github.com/lnfiniware/kairo.git
cd kairo

# Install globally
cargo install --path .
```

## Quick Start

```bash
# Initialize a new project
$ kairo init

# Define your schema in schema/users.kairo
$ echo 'table users { name: string, age: int }' > schema/users.kairo

# Create the table
$ kairo create users

# Query naturally
$ kairo query "SELECT * FROM users"
```

## Branding

*   **Website**: [https://kairo.infiniware.bid](https://kairo.infiniware.bid)
*   **CLI Names**: `kairo` and `kairodb`
*   **Version**: v0.2.0

## Natural Query Language

KairoDB v0.2 introduces the foundation of a natural query language. Instead of raw SQL, you can use more human-readable commands:

```bash
$ kairo query "from users where age > 18"
```
*Translated to: SELECT * FROM users where age > 18*

GNU GPL 3.0

## Support us

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/infiniware)

---

MADE BY INFINIWARE
