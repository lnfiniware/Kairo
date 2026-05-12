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
*   **CLI Name**: `kairo` or `kairodb`

## License

GNU GPL 3.0

---

MADE BY INFINIWARE
