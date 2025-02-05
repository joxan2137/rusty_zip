use std::env;
use windows_registry::{Key, };
use anyhow::Result;
use zip;

/*
    a : Add files to archive
    b : Benchmark
    d : Delete files from archive
    e : Extract files from archive (without using directory names)
    h : Calculate hash values for files
    i : Show information about supported formats
    l : List contents of archive
    rn : Rename files in archive
    t : Test integrity of archive
    u : Update files to archive
    x : eXtract files with full paths
*/

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!("Rusty Zip\n
        Usage: rz <command> [<switches>...] <archive_name> [<file_names>...] [@listfile]\n
         a : Add files to archive
         e : Extract files from archive (without using directory names)
         l : List contents of archive
         t : Test integrity of archive
        ")
    }

    Ok(())
}
