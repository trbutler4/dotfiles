use anyhow::{Context, Result};
use colored::*;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct DotfileManager {
    dotfiles_dir: PathBuf,
    backup_dir: PathBuf,
    home_dir: PathBuf,
    verbose: bool,
}

#[derive(Deserialize, Debug)]
struct FileMapping {
    source: String,
    target: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    files: Vec<FileMapping>,
}

impl DotfileManager {
    pub fn new(verbose: bool) -> Result<Self> {
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

    pub fn load_config(&self) -> Result<Config> {
        let config_path = self.dotfiles_dir.join("config.toml");
        let config_str = fs::read_to_string(config_path).context("Failed to read config.toml")?;
        let config: Config = toml::from_str(&config_str).context("Failed to parse config.toml")?;
        Ok(config)
    }

    pub fn log(&self, msg: &str) {
        if self.verbose {
            println!("{} {}", "INFO:".blue(), msg);
        }
    }

    pub fn install(&self) -> Result<()> {
        println!("{}", "Installing dotfiles...".green());

        // Create backup directory
        fs::create_dir_all(&self.backup_dir).context("Failed to create backup directory")?;

        // Load and process config
        let config = self.load_config()?;

        for mapping in config.files {
            let source = self.dotfiles_dir.join(&mapping.source);
            let target = self.home_dir.join(&mapping.target);

            if source.is_dir() {
                fs::create_dir_all(&target)?;
                self.process_directory(&source, &target)?;
            } else {
                self.link_file(&source, &target.parent().unwrap_or(&self.home_dir))?;
            }
        }

        self.check_secrets()?;
        Ok(())
    }

    pub fn process_topic(&self, topic: &str, topic_path: &Path) -> Result<()> {
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

    pub fn process_directory(&self, src_dir: &Path, target_dir: &Path) -> Result<()> {
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

    pub fn link_file(&self, src: &Path, target_dir: &Path) -> Result<()> {
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

    pub fn check_secrets(&self) -> Result<()> {
        let secrets_path = self.home_dir.join(".secrets");
        if !secrets_path.exists() {
            println!("{}", "No .secrets file found in home directory".yellow());
            println!("Create one if you need to store sensitive information");
        }
        Ok(())
    }

    pub fn get_target_path(&self, topic: &str, file_name: &str) -> PathBuf {
        match topic {
            "zellij" => self.home_dir.join(".config").join("zellij").join(file_name),
            "nvim" => self.home_dir.join(".config").join("nvim").join(file_name),
            _ => self.home_dir.join(file_name),
        }
    }

    pub fn list_configs(&self) -> Result<()> {
        println!("{}", "Current Configuration Files:".green().bold());
        println!("{}", "=========================".green());

        let config = self.load_config()?;

        for mapping in config.files {
            let source = self.dotfiles_dir.join(&mapping.source);
            let target = self.home_dir.join(&mapping.target);

            println!("\n{}:", mapping.source.blue().bold());
            println!("  Source: {}", source.display());
            println!("  Target: {}", target.display());
            println!(
                "  Status: {}",
                if target.exists() {
                    "Installed".green()
                } else {
                    "Not installed".yellow()
                }
            );
            println!("  {}", "-".repeat(50));
        }
        Ok(())
    }

    pub fn check_status(&self) -> Result<()> {
        println!("{}", "Configuration Status:".green().bold());
        println!("{}", "===================".green());

        let config = self.load_config()?;
        let mut all_good = true;

        for mapping in config.files {
            let target = self.home_dir.join(&mapping.target);
            if !target.exists() {
                println!("{} is not installed", mapping.source);
                all_good = false;
            }
        }

        if all_good {
            println!("\n{}", "All configurations are installed!".green());
        }

        Ok(())
    }

    pub fn add_config(&self, topic: &str, file: &Path) -> Result<()> {
        if !file.exists() {
            anyhow::bail!("File does not exist: {:?}", file);
        }

        let topic_dir = self.dotfiles_dir.join(topic);
        fs::create_dir_all(&topic_dir)?;

        let file_name = file.file_name().context("Invalid file name")?;
        let dest = topic_dir.join(file_name);

        fs::copy(file, &dest)?;
        println!(
            "Added {} to {} configuration",
            file_name.to_string_lossy(),
            topic
        );

        Ok(())
    }
}
