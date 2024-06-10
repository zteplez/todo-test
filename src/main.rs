mod db;

use std::io::Empty;

use clap::{Parser, Subcommand};
use db::{connect, delete_data, insert_data, show_data};
use rusqlite::{params, types::Null, Connection, Result};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Add { name: String, completed: String },
    Delete { id: i32 },
    Show {},
}

fn main() -> Result<()> {
    let args = Args::parse();
    let conn = connect()?;

    match &args.command {
        Commands::Add { name, completed } => {
            insert_data(&conn, name, completed)?;
        }
        Commands::Delete { id } => {
            delete_data(&conn, *id)?;
        }
        Commands::Show {} => {
            let tasks = show_data(&conn)?;
            let mut task_index = 1;
            for task in tasks {
                println!("{} -> {}", task_index, task.name);
                task_index += 1;
            }
        }
        
    }

    Ok(())
}
