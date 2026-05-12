mod ui;
mod core;
mod adapters;
mod config;

use clap::{Parser, Subcommand};
use anyhow::{Result, anyhow};
use crate::core::adapter::Adapter;
use crate::adapters::sqlite::SqliteAdapter;

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
            let schema_path = format!("schema/{}.kairo", name);
            let content = std::fs::read_to_string(&schema_path)
                .map_err(|_| anyhow!("Could not find schema file: {}. Create it in schema/ first.", schema_path))?;
            
            let schema = core::parser::parse_schema(&content)?;
            let sql = core::schema::generate_sql(&schema);
            
            println!("Generated SQL for {}:\n\n{}", name, sql);
            
            let config = config::load_config().ok();
            if let Some(cfg) = config {
                if cfg.adapter == "sqlite" {
                    let mut adapter = SqliteAdapter::new();
                    adapter.connect(&cfg.database).await?;
                    adapter.execute(&sql).await?;
                    ui::print_success(&format!("Applied schema for {} to {}", name, cfg.database));
                }
            }
        }
        Some(Commands::Query { query }) => {
            let config = config::load_config().map_err(|_| anyhow!("Could not load kairo.config. Run 'kairo init' first."))?;
            
            if config.adapter == "sqlite" {
                let mut adapter = SqliteAdapter::new();
                adapter.connect(&config.database).await?;
                
                let rows = adapter.query(query).await?;
                for row in rows {
                    let fields: Vec<String> = row.columns.iter().map(|c| format!("{}: {}", c.name, c.value)).collect();
                    println!("{}", fields.join(" | "));
                }
            } else {
                return Err(anyhow!("Adapter '{}' not supported yet", config.adapter));
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
