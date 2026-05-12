# Kairo Query Language

KairoDB supports both raw SQL and a more human-readable **Natural Query** syntax.

## Natural Query (v0.2+)
KairoDB allows you to query your data without writing raw SQL. This is the preferred way for simple operations.

### Basic Fetch
To select all columns from a table:
```bash
kairo query "from users"
```
*Translated to: SELECT * FROM users*

### Filtering
You can add conditions just like in SQL:
```bash
kairo query "from users where age > 18"
```
*Translated to: SELECT * FROM users where age > 18*

### Ordering and Limits
```bash
kairo query "from users order by name limit 10"
```
*Translated to: SELECT * FROM users order by name limit 10*

## Raw SQL
If you need complex joins or specific aggregations, you can always pass raw SQL directly:
```bash
kairo query "SELECT count(*), age FROM users GROUP BY age"
```

## How it Works
The query engine looks for the `from` keyword at the start of your query. If found, it automatically prepends `SELECT *` to make the query valid SQL for your configured adapter. This ensures that the syntax is both natural and database-agnostic.
