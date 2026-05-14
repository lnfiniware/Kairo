mod ui;
mod core;
mod adapters;
mod config;

use clap::{Parser, Subcommand};
use anyhow::{Result, anyhow};
use crate::core::adapter::Adapter;
use crate::adapters::sqlite::SqliteAdapter;
use crate::adapters::postgres::PostgresAdapter;

#[derive(Parser)]
#[command(name = "kairo")]
#[command(version = "0.3.1")]
#[command(about = "Human-readable databases. Minimal. Fast. Local-first.")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new KairoDB project
    Init,

    /// Create a table from a .kairo schema file
    Create {
        /// Name of the schema (matches schema/<name>.kairo)
        name: String,
    },

    /// Run a query against the database
    Query {
        /// SQL or natural query string
        query: String,
    },

    /// Read a database file and show its structure in human-readable format
    Read {
        /// Path to the database file
        file: String,
    },

    /// Export a database file to .kairo schema format
    Export {
        /// Path to the database file
        file: String,
        /// Output file (optional, prints to stdout if not given)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// List all tables in the current database
    Tables,

    /// Show current project status
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            init_project()?;
        }

        Some(Commands::Create { name }) => {
            let schema_path = std::path::PathBuf::from("schema").join(format!("{}.kairo", name));
            let content = std::fs::read_to_string(&schema_path)
                .map_err(|_| anyhow!("no schema file at {}", schema_path.display()))?;

            // Strip BOM
            let content = content.strip_prefix('\u{FEFF}').unwrap_or(&content);

            let schema = core::parser::parse_schema(content)?;
            let sql = core::schema::generate_sql(&schema);

            ui::print_dim("sql:");
            println!("{}", sql);

            let config = config::load_config().ok();
            if let Some(cfg) = config {
                let adapter = connect_adapter(&cfg).await?;
                adapter.execute(&sql).await?;
                ui::print_success(&format!("applied schema '{}' to {}", name, cfg.database));
            } else {
                ui::print_dim("no kairo.config found, schema not applied to any database");
            }
        }

        Some(Commands::Query { query }) => {
            let config = config::load_config()
                .map_err(|_| anyhow!("no kairo.config found. run 'kairo init' first."))?;

            let adapter = connect_adapter(&config).await?;

            let sql = core::query::translate_query(&query);
            if sql != query {
                ui::print_dim(&format!("-> {}", sql));
            }

            let rows = adapter.query(&sql).await?;
            if rows.is_empty() {
                ui::print_dim("(no rows)");
            } else {
                // Print header
                let headers: Vec<&str> = rows[0].columns.iter().map(|c| c.name.as_str()).collect();
                println!("{}", headers.join(" | "));
                println!("{}", "-".repeat(headers.iter().map(|h| h.len() + 3).sum::<usize>()));

                for row in &rows {
                    let fields: Vec<&str> = row.columns.iter().map(|c| c.value.as_str()).collect();
                    println!("{}", fields.join(" | "));
                }
                ui::print_dim(&format!("{} rows", rows.len()));
            }
        }

        Some(Commands::Read { file }) => {
            let output = core::reader::read_database(&file).await?;
            print!("{}", output);
        }

        Some(Commands::Export { file, output }) => {
            let schema = core::reader::export_schema(&file).await?;
            match output {
                Some(path) => {
                    std::fs::write(&path, &schema)?;
                    ui::print_success(&format!("exported schema to {}", path));
                }
                None => {
                    print!("{}", schema);
                }
            }
        }

        Some(Commands::Tables) => {
            let config = config::load_config()
                .map_err(|_| anyhow!("no kairo.config found. run 'kairo init' first."))?;
            let adapter = connect_adapter(&config).await?;
            let rows = adapter.query("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name").await?;

            if rows.is_empty() {
                ui::print_dim("(no tables)");
            } else {
                ui::print_header("tables");
                for row in &rows {
                    ui::print_row("-", &row.columns[0].value);
                }
            }
        }

        Some(Commands::Status) => {
            ui::print_header("kairo v0.3.1");

            match config::load_config() {
                Ok(cfg) => {
                    ui::print_row("adapter ", &cfg.adapter);
                    ui::print_row("database", &cfg.database);

                    let schema_count = std::fs::read_dir("schema")
                        .map(|entries| entries.filter_map(|e| e.ok()).filter(|e| {
                            e.path().extension().map(|ext| ext == "kairo").unwrap_or(false)
                        }).count())
                        .unwrap_or(0);

                    ui::print_row("schemas ", &schema_count.to_string());

                    if cfg.adapter == "sqlite" {
                        let exists = std::path::Path::new(&cfg.database).exists();
                        ui::print_row("db file ", if exists { "exists" } else { "not created yet" });
                    }
                }
                Err(_) => {
                    ui::print_dim("not initialized. run 'kairo init'.");
                }
            }
        }

        None => {
            ui::print_welcome();
        }
    }

    Ok(())
}

async fn connect_adapter(cfg: &config::Config) -> Result<Box<dyn Adapter>> {
    if cfg.adapter == "sqlite" {
        let mut a = SqliteAdapter::new();
        let conn_str = if cfg.database.starts_with("sqlite:") {
            cfg.database.clone()
        } else {
            format!("sqlite:{}?mode=rwc", cfg.database)
        };
        a.connect(&conn_str).await?;
        Ok(Box::new(a))
    } else if cfg.adapter == "postgres" {
        let mut a = PostgresAdapter::new();
        a.connect(&cfg.database).await?;
        Ok(Box::new(a))
    } else {
        Err(anyhow!("adapter '{}' not supported", cfg.adapter))
    }
}

fn init_project() -> Result<()> {
    std::fs::create_dir_all("schema")?;
    std::fs::create_dir_all("data")?;
    std::fs::create_dir_all("migrations")?;
    std::fs::create_dir_all("queries")?;
    std::fs::create_dir_all("plugins")?;

    if !std::path::Path::new("kairo.config").exists() {
        let config = "# KairoDB Configuration\nadapter = \"sqlite\"\ndatabase = \"data/kairo.db\"\n";
        std::fs::write("kairo.config", config)?;
    }

    ui::print_success("initialized.");
    Ok(())
}
