use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::MetadataExt;

use anyhow::Context;
use anyhow::{Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    #[arg(short, long, action=ArgAction::SetTrue)]
    names_only: bool,

    #[arg(last = true, help = "List of paths to print.")]
    paths: Vec<String>,

    #[arg(short='E', long, action=ArgAction::SetTrue, help = "Accept empty files (disabled by default)")]
    accept_empty: bool,

    #[arg(short='H', long, action=ArgAction::SetTrue, help = "Accept hidden files (disabled by default)")]
    accept_hidden: bool,
}

// These are only checked for files we encountered recursively. If the user deliberately added
// these files as arguments, they'd still be included in the output.

const DIR_IGNORE_LIST: &[&str] = &[
    ".git",
    "node_modules",
    "venv",
    ".venv",
    "target",
    ".vscode",
    "build",
];
const FILE_IGNORE_LIST: &[&str] = &[
    ".gitignore",
    "pyproject.toml",
    "Cargo.lock",
    "Cargo.toml",
    "go.mod",
    "go.sum",
];

fn insert_if_unseen_path(
    path: &Utf8Path,
    names_arr: &mut Vec<Utf8PathBuf>,
    hs: &mut HashSet<Utf8PathBuf>,
    cfg: &Config,
    explicitly_specified: bool,
    // _resolve_symlinks: bool, // TODO
) -> Result<()> {
    // Check ignore list & hidden-ness
    if let Some(fname) = path.file_name() {
        if FILE_IGNORE_LIST.contains(&fname) && !explicitly_specified {
            return Ok(());
        }

        let ban_hidden = !cfg.accept_hidden;
        if ban_hidden && fname.starts_with(".") {
            return Ok(());
        }
    } else {
        bail!("Expected {path} to contain a file name at the leaf...")
    }

    // Check emptiness
    // TODO: gate behind unix-path
    let md = fs::metadata(path)
        .with_context(|| format!("Failed to extract metadata for path '{path}'"))?;
    let ban_empty_files = !cfg.accept_empty;
    if ban_empty_files && md.size() == 0 {
        return Ok(());
    }

    let canonicalized = path
        .canonicalize_utf8()
        .with_context(|| format!("Failed to canonicalize path '{path}'"))?;

    if hs.insert(canonicalized) {
        names_arr.push(path.into());
    }

    Ok(())
}

fn extract_fnames_from_dir_recursively(
    dir: &Utf8Path,
    names_arr: &mut Vec<Utf8PathBuf>,
    hs: &mut HashSet<Utf8PathBuf>,
    cfg: &Config,
) -> Result<()> {
    for component in dir.iter() {
        if DIR_IGNORE_LIST.contains(&component) {
            return Ok(());
        }
    }

    for entry in dir.read_dir_utf8()? {
        let entry = entry.with_context(|| format!("Error while reading dir '{}'", dir))?;
        let path = entry.path();

        if path.is_dir() {
            extract_fnames_from_dir_recursively(&path, names_arr, hs, cfg)?;
        } else {
            insert_if_unseen_path(path, names_arr, hs, cfg, false)?;
        }
    }

    Ok(())
}

fn parse_args_to_fnames(cfg: &Config) -> Result<Vec<Utf8PathBuf>> {
    let paths = &cfg.paths;
    let mut names_arr: Vec<Utf8PathBuf> = vec![];
    let mut hs: HashSet<Utf8PathBuf> = HashSet::new();

    for path in paths {
        let path = Utf8Path::new(path);

        if !path.exists() {
            eprintln!("Path '{path}' doesn't exist.");
            continue;
        }

        if path.is_dir() {
            extract_fnames_from_dir_recursively(path, &mut names_arr, &mut hs, cfg)?;
        } else if path.is_file() {
            insert_if_unseen_path(path, &mut names_arr, &mut hs, cfg, true)?;
        } else {
            eprintln!("Path '{path}' is neither a directory nor a file.")
        }
    }

    Ok(names_arr)
}

fn main() -> Result<()> {
    let cfg = Config::parse();
    if cfg.paths.is_empty() {
        eprintln!("No paths provided. See the `-h` flag for help.");
        return Ok(());
    }

    let paths = parse_args_to_fnames(&cfg).with_context(|| "Failed to parse paths")?;
    if paths.is_empty() {
        eprintln!("No valid text files found.");
        return Ok(());
    }

    for path in paths {
        if cfg.names_only {
            std::println!("{path}");
        } else {
            let src = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read '{path}'"))?;
            std::println!("|-- {path} --|\n\n```\n{src}\n```\n");
        }
    }

    Ok(())
}
