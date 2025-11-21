use Quark::db::CacheDB;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Quark-DB", version = "0.1.0", author = "Yuvraj Biswal")]
#[command(
    about = "A mini cache DB",
    long_about = "A cache DB (just a hashmap as a db) in Rust"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Put { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Size,
    Save { path: String },
    Load { path: String },
}

fn main() {
    let mut db = CacheDB::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Put { key, value } => {
            db.put(key.clone(), value.clone());
            println!("Inserted ('{key}', '{value}')");
        }

        Commands::Get { key } => match db.get(&key) {
            Some(value) => println!("{key} => {value}"),
            None => println!("Key '{key}' not found"),
        },

        Commands::Delete { key } => {
            if db.delete(&key) {
                println!("Deleted key '{key}'");
            } else {
                println!("Key '{key}' not found");
            }
        }

        Commands::Size => {
            println!("DB size: {}", db.size());
        }
        Commands::Save { path } => {
            if let Err(e) = db.save(&path) {
                eprintln!("Error saving DB: {e}");
            } else {
                println!("Saved DB to {path}");
            }
        }
        Commands::Load { path } => match CacheDB::load_from_file(&path) {
            Ok(loaded_db) => {
                println!("Loaded DB from {}", path);
                println!("Size: {}", loaded_db.size());
            }
            Err(e) => eprintln!("Error loading DB: {e}"),
        },
    }
}
