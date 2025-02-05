use anyhow::Result;
use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Add files to archive
    #[arg(short, long)]
    add: bool,
    /// Benchmark
    #[arg(short, long)]
    benchmark: bool,
    /// Delete files frm archive
    #[arg(short, long)]
    delete: bool,
    /// Extract files from archive (without using directory names)
    #[arg(short = 'e', long)]
    extract: bool,
    /// Calculate hash values for file
    #[arg(short = 's', long)]
    hash: bool,
    /// Show information about supported formats
    #[arg(short, long)]
    info: bool,
    /// List contents of archive
    #[arg(short, long)]
    list: bool,
    /// Rename files in archive
    #[arg(short, long)]
    rename: bool,
    /// Test integrity of archive
    #[arg(short, long)]
    test: bool,
    /// Update files to archive
    #[arg(short, long)]
    update: bool,
    /// Extract files with full paths
    #[arg(short = 'x', long)]
    full_extract: bool,
    /// Archive name
    archive_name: String,
    /// Files
    files: Option<Vec<String>>,
}

const SUPPORTED_FORMATS: &[&str] = &[".zip", ".7z"];

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.archive_name.is_ascii() {
        panic!("non ascii archive name");
    }

    if !SUPPORTED_FORMATS
        .iter()
        .any(|format| args.archive_name.ends_with(format))
    {
        panic!("format not supported")
    }

    Ok(())
}
