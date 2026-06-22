use anyhow::Context;
use anyhow::{Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, action=ArgAction::SetTrue)]
    names_only: bool,

    #[arg(last = true, help = "List of paths to print.")]
    paths: Vec<String>,
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

fn extract_fnames_from_dir_recursively(
    dir: &Utf8Path,
    names_arr: &mut Vec<Utf8PathBuf>,
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
            extract_fnames_from_dir_recursively(&path, names_arr)?;
        } else {
            if let Some(fname) = path.file_name() {
                if !FILE_IGNORE_LIST.contains(&fname) && !fname.starts_with(".") {
                    names_arr.push(path.into());
                }
            } else {
                bail!("Expected {path} to be a file...")
            }
        }
    }

    Ok(())
}

fn parse_args_to_fnames(args: Vec<String>) -> Result<Vec<Utf8PathBuf>> {
    let mut names_arr: Vec<Utf8PathBuf> = vec![];

    for arg in &args {
        let path = Utf8Path::new(&arg);

        if !path.exists() {
            eprintln!("Path '{path}' doesn't exist.");
            continue;
        }

        if path.is_dir() {
            extract_fnames_from_dir_recursively(path, &mut names_arr)?;
        } else if path.is_file() {
            names_arr.push(path.to_path_buf());
        } else {
            eprintln!("Path '{path}' is neither a directory nor a file.")
        }
    }

    Ok(names_arr)
}

fn main() -> Result<()> {
    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();

    let paths = parse_args_to_fnames(args.paths).with_context(|| "Failed to parse paths")?;
    if paths.is_empty() {
        println!("No valid text files found.");
        return Ok(());
    }

    for path in paths {
        if args.names_only {
            std::println!("{}", path);
        } else {
            let src = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read '{}'", path))?;
            std::println!("|-- {} --|\n\n```\n{}\n```\n", path, src);
        }
    }

    Ok(())
}
