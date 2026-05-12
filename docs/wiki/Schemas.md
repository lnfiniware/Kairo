# Kairo Schema Syntax

KairoDB uses a human-readable syntax to define database tables. Files should be saved with the `.kairo` extension in the `schema/` directory.

## Basic Table
```kairo
table <table_name> {
  <field_name>: <type>
}
```

## Supported Types
- `string`: Maps to `TEXT` (SQLite) or `VARCHAR/TEXT` (PostgreSQL).
- `int`: Maps to `INTEGER`.
- `bool`: Maps to `BOOLEAN`.

## Default Values
You can specify default values using the `=` operator.
```kairo
table posts {
  title: string
  published: bool = false
  views: int = 0
}
```

## Formatting
- Fields can be separated by newlines or commas.
- Semicolons are not required.
- Comments can be added using `//`.
