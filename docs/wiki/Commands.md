# KairoDB Commands

KairoDB provides a minimal set of commands to manage your databases.

## `init`
Initializes a new KairoDB project in the current directory.
```bash
kairo init
```

## `create <name>`
Reads the schema file `schema/<name>.kairo`, generates the SQL, and applies it to the configured database.
```bash
kairo create users
```

## `query "<query>"`
Executes a query against the database. Supports both raw SQL and Kairo's Natural Query syntax.
```bash
# Natural Query
kairo query "from users where age > 18"

# Raw SQL
kairo query "SELECT * FROM users LIMIT 5"
```

## `migrate`
(Coming Soon) Manages database migrations across different environments.

## `dev`
(Coming Soon) Starts a development environment with live-reloading.

## `plugin`
(Coming Soon) Extends KairoDB with custom adapters and features.
