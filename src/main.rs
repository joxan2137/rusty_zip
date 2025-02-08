use std::path::Path;

use anyhow::Result;
use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Add files to archive
    #[arg(short, long)]
    add: Option<bool>,
    /// Benchmark
    #[arg(short, long)]
    benchmark: Option<bool>,
    /// Delete files from archive
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
    /// File(s)
    files: Option<String>,
}

const SUPPORTED_FORMATS: &[&str] = &[".zip", ".7z", ".tar", ".gz", ".rar"];

fn main() -> Result<()> {
    let args = Args::parse();

    if args.archive_name.is_some()
        && !SUPPORTED_FORMATS
            .iter()
            .any(|format| args.archive_name.as_ref().unwrap().ends_with(format))
    {
        panic!(
            "Archive format not supported. Supported Archive formats: {:?}",
            SUPPORTED_FORMATS
        );
    }

    handle_args(args)?;
    Ok(())
}

fn handle_args(args: Args) -> Result<()> {
    match args {
        Args {
            add: Some(true), ..
        } => {
            if args.files.is_none() {
                panic!("A path to the target is required.");
            }
            let path = Path::new(&args.files.unwrap());

            Ok(())
        }
        Args {
            benchmark: Some(true),
            ..
        } => {
            loop {
                println!("Benchmarking...");
            }
            Ok(())
        }
        Args {
            delete: Some(true),
            files: Some(value),
            ..
        } => {
            let path = Path::new(&value);

            Ok(())
        }
        Args {
            extract: Some(true),
            files: Some(value),
            ..
        } => {
            let path = Path::new(&value);

            Ok(())
        }
        Args {
            hash: Some(true),
            files: Some(value),
            ..
        } => {
            let path = Path::new(&value);

            Ok(())
        }
        Args {
            info: Some(true), ..
        } => {
            println!("Rusty Zip - A simple archive manager inspired by 7zip made with Rust.");
            println!("Authors: @joxan2137 @scaledcat");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Supported Archive formats: {:?}", SUPPORTED_FORMATS);

            Ok(())
        }

        Args {
            list: Some(true),
            files: Some(value),
            ..
        } => {
            let path = Path::new(&value);

            Ok(())
        }
        Args {
            rename: Some(true),
            files: Some(value),
            ..
        } => {
            let path = Path::new(&value);

            Ok(())
        }

        Args {
            test: Some(true),
            files: Some(value),
            ..
        } => {
            let path = Path::new(&value);

            Ok(())
        }

        Args {
            update: Some(true), ..
        } => {
            panic!("Automatic updates are not supported yet.");
        }
        _ => {
            panic!("Invalid arguments.");
        }
    }
}
