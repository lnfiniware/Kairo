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
#[command(about = "Human-readable databases. Minimal. Fast. Local-first.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new KairoDB project
    Init,
    /// Create a new table or schema
    Create { name: String },
    /// Query data naturally
    Query { query: String },
    /// Run pending migrations
    Migrate,
    /// Start development server
    Dev,
    /// Manage plugins
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// Add a new plugin
    Add { name: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            init_project()?;
        }
        Some(Commands::Create { name }) => {
            let schema_path = std::path::PathBuf::from("schema").join(format!("{}.kairo", name));
            let content = std::fs::read_to_string(&schema_path)
                .map_err(|_| anyhow!("Could not find schema file: {}. Create it in schema/ first.", schema_path.display()))?;
            
            // Strip BOM if present
            let content = content.strip_prefix('\u{FEFF}').unwrap_or(&content);
            
            let schema = core::parser::parse_schema(content)?;
            let sql = core::schema::generate_sql(&schema);
            
            println!("Generated SQL for {}:\n\n{}", name, sql);
            
            let config = config::load_config().ok();
            if let Some(cfg) = config {
                let mut adapter: Box<dyn Adapter> = if cfg.adapter == "sqlite" {
                    let mut a = SqliteAdapter::new();
                    let conn_str = if cfg.database.starts_with("sqlite:") {
                        cfg.database.clone()
                    } else {
                        format!("sqlite:{}?mode=rwc", cfg.database)
                    };
                    a.connect(&conn_str).await?;
                    Box::new(a)
                } else if cfg.adapter == "postgres" {
                    let mut a = PostgresAdapter::new();
                    a.connect(&cfg.database).await?;
                    Box::new(a)
                } else {
                    return Err(anyhow!("Adapter '{}' not supported", cfg.adapter));
                };

                adapter.execute(&sql).await?;
                ui::print_success(&format!("Applied schema for {} to {}", name, cfg.database));
            }
        }
        Some(Commands::Query { query }) => {
            let config = config::load_config().map_err(|_| anyhow!("Could not load kairo.config. Run 'kairo init' first."))?;
            
            let mut adapter: Box<dyn Adapter> = if config.adapter == "sqlite" {
                let mut a = SqliteAdapter::new();
                let conn_str = if config.database.starts_with("sqlite:") {
                    config.database.clone()
                } else {
                    format!("sqlite:{}?mode=rwc", config.database)
                };
                a.connect(&conn_str).await?;
                Box::new(a)
            } else if config.adapter == "postgres" {
                let mut a = PostgresAdapter::new();
                a.connect(&config.database).await?;
                Box::new(a)
            } else {
                return Err(anyhow!("Adapter '{}' not supported", config.adapter));
            };

            let sql = core::query::translate_query(query);
            if sql != *query {
                println!("Translated to: {}", sql);
            }

            let rows = adapter.query(&sql).await?;
            for row in rows {
                let fields: Vec<String> = row.columns.iter().map(|c| format!("{}: {}", c.name, c.value)).collect();
                println!("{}", fields.join(" | "));
            }
        }
        Some(Commands::Migrate) => {
            println!("Running migrations...");
        }
        Some(Commands::Dev) => {
            println!("Starting dev server...");
        }
        Some(Commands::Plugin { action }) => {
            match action {
                PluginCommands::Add { name } => {
                    println!("Adding plugin: {}", name);
                }
            }
        }
        None => {
            ui::print_welcome();
        }
    }

    Ok(())
}

fn init_project() -> Result<()> {
    // Basic init logic
    std::fs::create_dir_all("schema")?;
    std::fs::create_dir_all("data")?;
    std::fs::create_dir_all("migrations")?;
    std::fs::create_dir_all("queries")?;
    std::fs::create_dir_all("plugins")?;
    
    // Create default config
    let config = r#"# KairoDB Configuration
adapter = "sqlite"
database = "data/kairo.db"
"#;
    std::fs::write("kairo.config", config)?;

    ui::print_success("Initialized KairoDB project.");
    Ok(())
}
