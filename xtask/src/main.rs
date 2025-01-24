use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};
use xshell::{cmd, Shell};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the project
    Build,
    /// Install the project
    Install {
        #[arg(long, default_value = "/")]
        destdir: PathBuf,
        #[arg(long, default_value = "usr")]
        prefix: PathBuf,
        #[arg(long)]
        mode: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build => build(),
        Commands::Install {
            destdir,
            prefix,
            mode,
        } => install(destdir, prefix, mode.as_deref()),
    }
}

fn build() -> Result<()> {
    let sh = Shell::new()?;
    println!("Building release version...");
    cmd!(sh, "cargo build --release").run()?;
    Ok(())
}

fn install(destdir: &Path, prefix: &Path, mode: Option<&str>) -> Result<()> {
    if !fs::exists("target/release/verdi")? {
        bail!("You must build the project first!")
    }

    let binary_dir = destdir.join(prefix).join("bin");

    // Create target directory if it doesn't exist
    fs::create_dir_all(&binary_dir).context("Failed to create binary directory")?;

    let target = binary_dir.join("verdi");

    fs::copy("target/release/verdi", &target)
        .with_context(|| format!("Failed to copy binary to {:?}", target))?;

    if let Some(mode) = mode {
        // Parse octal mode string (e.g., "755" or "0755")
        let mode = u32::from_str_radix(mode.trim_start_matches('0'), 8)
            .with_context(|| format!("Invalid mode: {mode}"))?;

        fs::set_permissions(&target, fs::Permissions::from_mode(mode))
            .context("Failed to set binary permissions")?;
    }

    println!("Installation complete!");
    Ok(())
}
