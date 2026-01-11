//! eml2md CLI - Convert EML files to Markdown

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use eml2md::{parse_eml, formatter::format_markdown};

/// Convert EML files to Markdown
#[derive(Parser, Debug)]
#[command(name = "eml2md", version, about, long_about = None)]
struct Args {
    /// Input EML file
    #[arg(short, long, required = true)]
    input: PathBuf,

    /// Output Markdown file
    #[arg(short, long, required = true)]
    output: PathBuf,

    /// Output format: "simple" or "html"
    #[arg(short, long, default_value = "simple")]
    format: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Parse the EML file
    let email = parse_eml(&args.input)
        .with_context(|| format!("Failed to parse EML file: {}", args.input.display()))?;

    // Format as markdown
    let markdown = format_markdown(&email, &args.format);

    // Write output
    std::fs::write(&args.output, &markdown)
        .with_context(|| format!("Failed to write output file: {}", args.output.display()))?;

    println!(
        "Successfully converted {} to {}",
        args.input.display(),
        args.output.display()
    );

    Ok(())
}
