use rayon::prelude::*;
use std::env;
use std::ffi::OsStr;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use std::str;
use walkdir::WalkDir;

fn process_image(path: &Path, compression_level: Option<u8>) -> Result<(), Error> {
    let compression_arg = match compression_level {
        Some(level) => format!("-{}", level),
        None => String::new(),
    };

    let output = Command::new("ect")
        .arg(&compression_arg)
        .arg("--strict")
        .arg("-keep")
        .arg("--mt-file")
        .arg("-q")
        .arg(path)
        .output()?;

    let stdout_str =
        str::from_utf8(&output.stdout).unwrap_or("Failed to convert stdout output to UTF-8");

    if !stdout_str.trim().is_empty() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Command execution failed: {}", stdout_str),
        ));
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input-path>... [-1 to -9]", args[0]);
        std::process::exit(1);
    }

    let mut input_paths: Vec<&str> = Vec::new();
    let mut compression_level: Option<u8> = None;

    for arg in &args[1..] {
        if arg.starts_with('-') && arg.len() > 1 && compression_level.is_none() {
            compression_level = arg[1..].parse::<u8>().ok();
        } else {
            input_paths.push(arg);
        }
    }

    input_paths.par_iter().for_each(|input_path_str| {
        let input_path = Path::new(input_path_str);
        if input_path.is_dir() {
            let paths: Vec<_> = WalkDir::new(input_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file() && e.path().extension() == Some(OsStr::new("png")))
                .map(|e| e.path().to_owned())
                .collect();

            paths.par_iter().for_each(|path| {
                if let Err(e) = process_image(path, compression_level) {
                    eprintln!("Failed to process {}: {}", path.display(), e);
                }
            });
        } else if input_path.is_file() {
            if input_path.extension() == Some(OsStr::new("png")) {
                if let Err(e) = process_image(input_path, compression_level) {
                    eprintln!("Failed to process {}: {}", input_path.display(), e);
                }
            } else {
                eprintln!("File is not a PNG: {}", input_path.display());
            }
        } else {
            eprintln!("Invalid input path: {}", input_path.display());
        }
    });
}
