use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pu_erh_core::Session;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "pu-erh", about = "Block-based graph knowledge base")]
struct Cli {
    #[arg(long)]
    file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Query {
        expression: String,
    },
    Create {
        #[arg(long)]
        parent: Uuid,
    },
    Move {
        id: Uuid,
        #[arg(long)]
        parent: Uuid,
    },
    Delete {
        id: Uuid,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let mut session = Session::open(&cli.file).context("failed to open knowledge base")?;

    match cli.command {
        Commands::Init => {
            session.save().context("failed to save knowledge base")?;
            let root = session.root_id().context("failed to resolve root block")?;
            println!("{root}");
        }
        Commands::Query { expression } => {
            let blocks = session
                .query(&expression)
                .context("query failed")?;
            for block in blocks {
                println!("{}", format_block(&block));
            }
        }
        Commands::Create { parent } => {
            let id = session
                .create_block(Some(parent))
                .context("create failed")?;
            session.save().context("failed to save knowledge base")?;
            println!("{id}");
        }
        Commands::Move { id, parent } => {
            session
                .move_block(id, Some(parent))
                .context("move failed")?;
            session.save().context("failed to save knowledge base")?;
        }
        Commands::Delete { id } => {
            session.delete_block(id).context("delete failed")?;
            session.save().context("failed to save knowledge base")?;
        }
    }

    Ok(())
}

fn format_block(block: &graph::Block) -> String {
    let properties = graph::properties_to_json_string(&block.properties);
    format!("{} {}", block.id, properties)
}
