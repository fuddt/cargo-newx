use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "cargo-newx",
    about = "Creates a new Rust project with best practice configuration files",
    version
)]
struct Args {
    /// Name of the project to create
    project_name: String,

    /// Create a library project instead of a binary
    #[arg(long)]
    lib: bool,

    /// Add clippy.toml configuration
    #[arg(long)]
    clippy: bool,

    /// Add both rustfmt.toml and clippy.toml configurations
    #[arg(long)]
    all: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Check if directory already exists
    let project_path = Path::new(&args.project_name);
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", args.project_name);
    }

    // Run cargo new command
    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.arg("new").arg(&args.project_name);
    
    if args.lib {
        cargo_cmd.arg("--lib");
    }

    let output = cargo_cmd
        .output()
        .context("Failed to execute cargo new command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("cargo new failed: {}", stderr);
    }

    println!("Created new Rust project: {}", args.project_name);

    // Always add rustfmt.toml (default behavior)
    copy_template_file(&args.project_name, "rustfmt.toml")?;
    println!("Added rustfmt.toml configuration");

    // Add clippy.toml if requested
    if args.clippy || args.all {
        copy_template_file(&args.project_name, "clippy.toml")?;
        println!("Added clippy.toml configuration");
    }

    println!("Project '{}' created successfully!", args.project_name);
    Ok(())
}

fn copy_template_file(project_name: &str, template_name: &str) -> Result<()> {
    // Get the directory where the binary is located
    let exe_dir = std::env::current_exe()
        .context("Failed to get current executable path")?
        .parent()
        .context("Failed to get executable parent directory")?
        .to_path_buf();

    // Look for templates directory relative to the binary
    let template_path = exe_dir.join("../templates").join(template_name);
    
    // If not found, try relative to current directory (for development)
    let template_path = if template_path.exists() {
        template_path
    } else {
        PathBuf::from("templates").join(template_name)
    };

    if !template_path.exists() {
        anyhow::bail!("Template file '{}' not found", template_name);
    }

    let content = fs::read_to_string(&template_path)
        .with_context(|| format!("Failed to read template file: {}", template_path.display()))?;

    let dest_path = Path::new(project_name).join(template_name);
    fs::write(&dest_path, content)
        .with_context(|| format!("Failed to write file: {}", dest_path.display()))?;

    Ok(())
}
