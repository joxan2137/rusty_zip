use std::path::Path;

use anyhow::Result;
use clap::{arg, command, Parser};
use flate2;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Add files to archive
    #[arg(short, long)]
    add: Option<bool>,
    /// Benchmark
    #[arg(short, long)]
    benchmark: Option<bool>,
    /// Delete files frm archive
    #[arg(short, long)]
    delete: Option<bool>,
    /// Extract files from archive (without using directory names)
    #[arg(short = 'e', long)]
    extract: Option<bool>,
    /// Calculate hash values for file
    #[arg(short = 's', long)]
    hash: Option<bool>,
    /// Show information about supported formats
    #[arg(short, long)]
    info: Option<bool>,
    /// List contents of archive
    #[arg(short, long)]
    list: Option<bool>,
    /// Rename files in archive
    #[arg(short, long)]
    rename: Option<bool>,
    /// Test integrity of archive
    #[arg(short, long)]
    test: Option<bool>,
    /// Update files to archive
    #[arg(short, long)]
    update: Option<bool>,
    /// Extract files with full paths
    #[arg(short = 'x', long)]
    full_extract: Option<bool>,
    /// Archive name
    archive_name: Option<String>,
    /// Files
    files: Option<String>,
}

const SUPPORTED_FORMATS: &[&str] = &[".zip", ".7z", ".tar", ".gz", ".rar"];

fn main() -> Result<()> {
    let args = Args::parse();

    if args.archive_name.is_some() {
        if !SUPPORTED_FORMATS
            .iter()
            .any(|format| args.archive_name.as_ref().unwrap().ends_with(format))
        {
            panic!("Archive format not supported. Supported Archive formats: {:?}", SUPPORTED_FORMATS);
        }
    }

    Ok(())
}

fn handle_arg(args: Args) -> Result<()>{


    match args {
        Args { add: Some(true), files: Some(value), .. }  => {
            let path = Path::new(&value);

            Ok(())

        },   
        Args { add: Some(true), files: None, .. } => {
            panic!("A path to the target is required.");
        },
        Args { benchmark: Some(true), .. } => {
            Ok(())
        },


        Args { delete: Some(true), .. } => {
            Ok(())
        },
        Args { extract: Some(true), .. } => {
            Ok(())
        },

        Args { hash: Some(true), .. } => {
            Ok(())
        },

        Args { info: Some(true), .. } => {
            Ok(())
        },

        Args { list: Some(true), .. } => {
            Ok(())
        },

        Args { rename: Some(true), .. } => {
            Ok(())
        },

        Args { test: Some(true), .. } => {
            Ok(())
        },
        Args { update: Some(true), .. } => {
            Ok(())
        },
        Args { full_extract: Some(true), .. } => {
            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}