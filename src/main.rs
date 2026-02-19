use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// These are only checked for files we encountered recursively. If the user deliberately added
// these files as arguments, they'd still be included in the output.

const DIR_IGNORE_LIST: &[&str] = &[".git", "node_modules", "venv", ".venv", "target"];
const FILE_IGNORE_LIST: &[&str] = &[".gitignore", "pyproject.toml", "Cargo.lock", "Cargo.toml"];

fn extract_fnames_from_dir_recursively(dir: &Path, names_arr: &mut Vec<PathBuf>) -> io::Result<()> {
    for component in dir.iter() {
        if DIR_IGNORE_LIST.contains(&component.to_str().unwrap()) {
            return Ok(());
        }
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            extract_fnames_from_dir_recursively(&path, names_arr)?;
        } else {
            // Apologies for this abomination
            let fname = path.file_name().unwrap().to_str().unwrap();

            if !FILE_IGNORE_LIST.contains(&fname) && !fname.starts_with(".") {
                names_arr.push(path);
            }
        }
    }

    Ok(())
}

fn parse_args_to_fnames(args: Vec<String>) -> io::Result<Vec<PathBuf>> {
    let mut names_arr: Vec<PathBuf> = vec![];

    for arg in &args[1..] {
        let path = Path::new(&arg);

        if path.is_dir() {
            extract_fnames_from_dir_recursively(path, &mut names_arr)?;
        } else {
            names_arr.push(path.to_path_buf());
        }
    }

    Ok(names_arr)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let paths = parse_args_to_fnames(args)?;
    if paths.is_empty() {
        println!("No valid text files found.");
        return Ok(());
    }

    for path in paths {
        let src = std::fs::read_to_string(&path)?;
        std::println!("|-- {} --| \n\n```\n{}```\n", path.display(), src);
    }

    Ok(())
}
