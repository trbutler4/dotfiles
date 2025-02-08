use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

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

struct DotfileManager {
    dotfiles_dir: PathBuf,
    backup_dir: PathBuf,
    home_dir: PathBuf,
    verbose: bool,
}

impl DotfileManager {
    fn new(verbose: bool) -> Result<Self> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        let dotfiles = home.join("dotfiles");
        let backup = home
            .join(".dotfiles_backup")
            .join(chrono::Local::now().format("%Y%m%d_%H%M%S").to_string());

        Ok(Self {
            dotfiles_dir: dotfiles,
            backup_dir: backup,
            home_dir: home,
            verbose,
        })
    }

    fn log(&self, msg: &str) {
        if self.verbose {
            println!("{} {}", "INFO:".blue(), msg);
        }
    }

    fn install(&self) -> Result<()> {
        println!("{}", "Installing dotfiles...".green());

        // Create backup directory
        fs::create_dir_all(&self.backup_dir).context("Failed to create backup directory")?;

        // Process each topic directory
        for entry in fs::read_dir(&self.dotfiles_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let topic_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .context("Invalid topic name")?;

                if topic_name == ".git" {
                    continue;
                }

                self.process_topic(topic_name, &path)?;
            }
        }

        self.check_secrets()?;
        Ok(())
    }

    fn process_topic(&self, topic: &str, topic_path: &Path) -> Result<()> {
        println!("{} {}", "Processing topic:".green(), topic);

        // Determine target directory based on topic
        let target_dir = match topic {
            "zellij" => self.home_dir.join(".config").join("zellij"),
            "nvim" => self.home_dir.join(".config").join("nvim"),
            _ => self.home_dir.clone(),
        };

        // Create target directory if it doesn't exist
        fs::create_dir_all(&target_dir)?;

        // Process all files in the topic directory
        self.process_directory(topic_path, &target_dir)?;

        Ok(())
    }

    fn process_directory(&self, src_dir: &Path, target_dir: &Path) -> Result<()> {
        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                self.link_file(&path, target_dir)?;
            } else if path.is_dir() {
                let new_target = target_dir.join(path.file_name().unwrap());
                fs::create_dir_all(&new_target)?;
                self.process_directory(&path, &new_target)?;
            }
        }

        Ok(())
    }

    fn link_file(&self, src: &Path, target_dir: &Path) -> Result<()> {
        let file_name = src.file_name().context("Invalid file name")?;
        let dest = target_dir.join(file_name);

        self.log(&format!("Processing file: {:?}", file_name));

        // Backup existing file
        if dest.exists() {
            let backup = self.backup_dir.join(file_name);
            fs::rename(&dest, &backup).context("Failed to backup existing file")?;
            println!("{} {:?}", "Backed up:".yellow(), dest);
        }

        // Create symlink
        #[cfg(unix)]
        std::os::unix::fs::symlink(src, &dest).context("Failed to create symlink")?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_file(src, &dest).context("Failed to create symlink")?;

        println!("{} {:?} -> {:?}", "Linked:".green(), src, dest);
        Ok(())
    }

    fn check_secrets(&self) -> Result<()> {
        let secrets_path = self.home_dir.join(".secrets");
        if !secrets_path.exists() {
            println!("{}", "No .secrets file found in home directory".yellow());
            println!("Create one if you need to store sensitive information");
        }
        Ok(())
    }

    fn list_configs(&self) -> Result<()> {
        println!("{}", "Available configurations:".green());
        for entry in fs::read_dir(&self.dotfiles_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if name != ".git" {
                        println!("- {}", name);
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let manager = DotfileManager::new(args.verbose)?;

    match args.command.as_str() {
        "install" => manager.install()?,
        "list" => manager.list_configs()?,
        "check" => {
            println!("{}", "Checking configuration...".green());
            manager.check_secrets()?;
        }
        _ => println!("{}", "Unknown command".red()),
    }

    Ok(())
}
