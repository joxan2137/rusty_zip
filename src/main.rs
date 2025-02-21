use std::path::Path;

use anyhow::Result;
use clap::{arg, command, Parser};
use std::fs;

mod formats;

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
            let _path = Path::new(&args.files.unwrap());

            Ok(())
        }
        Args {
            benchmark: Some(true),
            ..
        } => {
            todo!()
        }
        Args {
            delete: Some(true),
            files: Some(value),
            ..
        } => {
            let _path = Path::new(&value);

            Ok(())
        }
        Args {
            extract: Some(true),
            files: Some(value),
            ..
        } => {
            let _path = Path::new(&value);

            Ok(())
        }
        Args {
            hash: Some(true),
            files: Some(value),
            ..
        } => {
            let _path = Path::new(&value);

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
            let _path = Path::new(&value);

            Ok(())
        }
        Args {
            rename: Some(true),
            files: Some(value),
            ..
        } => {
            let _path = Path::new(&value);

            Ok(())
        }

        Args {
            test: Some(true),
            files: Some(value),
            ..
        } => {
            let _path = Path::new(&value);

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

#[allow(dead_code)]
fn add_files(files: &str) -> Result<()> {
    let file_vec = files.split(",").collect::<Vec<&str>>();
    for file in file_vec {
        let path = Path::new(&file);
        if !path.exists() {
            return Err(anyhow::anyhow!("File does not exist."));
        }
        let _metadata = fs::metadata(path)?;
    }
    Ok(())
}

use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
fn detect_archive_type(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 16];
    file.read_exact(&mut header).unwrap_or(());
    
    match &header {
        [0x50, 0x4B, 0x03, 0x04, ..] => {
            println!("Detected ZIP archive");
            Ok("ZIP".into())
        },
        [0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C, ..] => { 
            println!("Detected 7z archive");
            Ok("7z".to_string())
        },
        [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00, ..] => {
            println!("Detected RAR5 archive");
            Ok("RAR5".to_string()) 
        },
        [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00, ..] => {
            println!("Detected RAR4 archive");
            Ok("RAR4".to_string()) 
        },
        [0x1F, 0x8B, 0x08, ..] => {
            println!("Detected GZIP archive");
            Ok("GZIP".to_string())
        },
        _ => {
            println!("Detected unknown archive");
            Ok("Unknown".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io::BufReader, path::PathBuf};

    use crate::formats::seven_z;

    use super::*;

    #[test]
    fn test_detect_archive_type() {
        assert_eq!(detect_archive_type("test.zip").unwrap(), "ZIP");
        assert_eq!(detect_archive_type("test.7z").unwrap(), "7z");
        assert_eq!(detect_archive_type("test.rar").unwrap(), "RAR5");
        assert_eq!(detect_archive_type("test.tar.gz").unwrap(), "GZIP");
        assert_eq!(detect_archive_type("test.txt").unwrap(), "Unknown");
    }

    #[test]
    fn test_decompression() {
        let path = PathBuf::from("test_decompression");
        fs::create_dir(&path).unwrap();

        println!("{:?}", path);
        assert_eq!(seven_z::decompress_seven_z(BufReader::new(File::open("test.7z").unwrap()), path.clone()).is_ok(), true);

        fs::remove_dir_all(path).unwrap();
    }
    
    #[test]
    fn test_compression() {
        let path = PathBuf::from("test_compression");
        fs::create_dir(&path).unwrap();
        File::create_new(path.join("test.txt")).unwrap();

        println!("{:?}", path);
        assert_eq!(seven_z::compress_seven_z(&path.join("test.txt"), &path, "file".to_string()).is_ok(), true);

        fs::remove_dir_all(path).unwrap();
    }
}