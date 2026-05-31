use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pu_erh_core::{PositionHint, Session};
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
        #[arg(long, conflicts_with_all = ["after", "before", "last"])]
        first: bool,
        #[arg(long, conflicts_with_all = ["first", "before", "last"])]
        after: Option<Uuid>,
        #[arg(long, conflicts_with_all = ["first", "after", "last"])]
        before: Option<Uuid>,
        #[arg(long, conflicts_with_all = ["first", "after", "before"])]
        last: bool,
    },
    Move {
        id: Uuid,
        #[arg(long)]
        parent: Uuid,
        #[arg(long, conflicts_with_all = ["after", "before", "last"])]
        first: bool,
        #[arg(long, conflicts_with_all = ["first", "before", "last"])]
        after: Option<Uuid>,
        #[arg(long, conflicts_with_all = ["first", "after", "last"])]
        before: Option<Uuid>,
        #[arg(long, conflicts_with_all = ["first", "after", "before"])]
        last: bool,
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

fn parse_position(first: bool, last: bool, after: Option<Uuid>, before: Option<Uuid>) -> Result<PositionHint> {
    match (first, last, after, before) {
        (true, _, _, _) => Ok(PositionHint::First),
        (_, _, Some(id), _) => Ok(PositionHint::After(id)),
        (_, _, _, Some(id)) => Ok(PositionHint::Before(id)),
        _ => Ok(PositionHint::Last),
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
        Commands::Create { parent, first, last, after, before } => {
            let position = parse_position(first, last, after, before)?;
            let id = session
                .create_block(Some(parent), position)
                .context("create failed")?;
            session.save().context("failed to save knowledge base")?;
            println!("{id}");
        }
        Commands::Move { id, parent, first, last, after, before } => {
            let position = parse_position(first, last, after, before)?;
            session
                .move_block(id, Some(parent), position)
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
