use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

mod dotfile_manager;
use dotfile_manager::DotfileManager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to execute (install, list, check)
    #[arg(default_value = "install")]
    command: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Parser)]
#[command(author, version, about = "Manage your dotfiles")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install all dotfiles
    Install,

    /// List available configurations
    List,

    /// Check configuration status
    Status,

    /// Add a new configuration file
    Add {
        /// Topic (e.g., vim, zsh)
        topic: String,
        /// Path to the file to add
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let manager = DotfileManager::new(cli.verbose)?;

    match cli.command {
        Commands::Install => {
            manager.install()?;
        }
        Commands::List => {
            manager.list_configs()?;
        }
        Commands::Status => {
            manager.check_status()?;
        }
        Commands::Add { topic, file } => {
            manager.add_config(&topic, &file)?;
        }
    }

    Ok(())
}
