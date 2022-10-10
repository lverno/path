use anyhow::{bail, Result};
use clap::Parser;
use std::path::PathBuf;
use winreg::{enums::*, RegKey};

#[derive(Parser)]
#[command(about = "Add/remove a directory to/from path")]
struct Args {
    #[arg(default_value_t = String::from("."), help = "The directory to add/remove")]
    dir: String,
    #[arg(short, long, help = "Remove the directory (if present)")]
    remove: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // make path absolute
    let dir = PathBuf::from(args.dir).canonicalize()?;

    // get the user Path variable, located at HKEY_CURRENT_USER\Environment\Path
    let key =
        RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags("Environment", KEY_ALL_ACCESS)?;
    let value: String = key.get_value("Path")?;
    let mut entries: Vec<_> = value.split(';').map(PathBuf::from).collect();

    if args.remove {
        // remove all instances of the path
        entries.retain(|e| e != &dir);
    } else {
        // disallow duplicates
        if entries.contains(&dir) {
            bail!("already on path");
        }
        entries.push(dir);
    }

    // reconstruct semicolon-separated list
    let new = entries
        .iter()
        .map(|p: &PathBuf| p.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(";");

    key.set_value("Path", &new)?;

    Ok(())
}
