mod args;
mod clipboard;
mod config;
mod file_handling;

use anyhow::{Context, Result};
use args::CliArgs;
use clap::Parser;
use clipboard::copy_to_clipboard;
use std::path::Path;

fn main() -> Result<()> {
    let cli = CliArgs::parse();
    let output = file_handling::process_files(&cli.paths, &cli.ignore)?;

    if cli.output.is_none() {
        copy_to_clipboard(&output)
            .map_err(|e| eprintln!("{}", e))
            .ok();
    }

    if let Some(out_file) = &cli.output {
        if let Some(parent) = Path::new(out_file).parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory for {}", out_file))?;
        }
        std::fs::write(out_file, &output)
            .with_context(|| format!("Failed to write output to {}", out_file))?;
        println!("Output saved to: {}", out_file);
    }

    Ok(())
}
