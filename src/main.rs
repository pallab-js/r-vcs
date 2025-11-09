mod commands;
mod config;
mod ignore;
mod lock;
mod objects;
mod repository;
mod utils;

use clap::{Parser, Subcommand};
use commands::*;

#[derive(Parser)]
#[command(name = "vcs")]
#[command(about = "A version control system similar to git", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init,
    /// Add files to the staging area
    Add {
        /// Files or directories to add
        paths: Vec<String>,
    },
    /// Commit staged changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
    },
    /// Show repository status
    Status,
    /// Show commit history
    Log {
        /// One-line format
        #[arg(long)]
        oneline: bool,
        /// Number of commits to show
        #[arg(short = 'n', long)]
        number: Option<usize>,
    },
    /// Show file contents from repository
    CatFile {
        /// Object hash
        hash: String,
    },
    /// Configure VCS settings
    Config {
        /// Key to get/set
        key: Option<String>,
        /// Value to set
        value: Option<String>,
        /// Use global config
        #[arg(long)]
        global: bool,
        /// List all config
        #[arg(long)]
        list: bool,
    },
    /// Unstage files from index
    Reset {
        /// Files to unstage
        paths: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => init(),
        Commands::Add { paths } => add(paths),
        Commands::Commit { message } => commit(&message),
        Commands::Status => status(),
        Commands::Log { oneline, number } => log(oneline, number),
        Commands::CatFile { hash } => cat_file(&hash),
        Commands::Config {
            key,
            value,
            global,
            list,
        } => config(key, value, global, list),
        Commands::Reset { paths } => reset(paths),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
