use rayon::prelude::*;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::Path;
use std::process::Command;
use std::str;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}

pub fn process_image(path: &Path, compression_level: Option<u8>) -> Result<(), Box<dyn Error>> {
    let compression_arg = match compression_level {
        Some(level) => format!("-{}", level),
        None => String::from("-9"),
    };

    let output = Command::new("ect")
        .arg(&compression_arg)
        .arg("--strict")
        .arg("-keep")
        .arg("--mt-file")
        .arg("-q")
        .arg(path)
        .output()?;

    let stdout_str = str::from_utf8(&output.stdout).map_err(|_| {
        io::Error::new(
            io::ErrorKind::Other,
            "Failed to convert stdout output to UTF-8",
        )
    })?;

    if !stdout_str.trim().is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("Command execution failed: {}", stdout_str),
        )));
    }

    Ok(())
}

pub fn process_directory(input_path: &Path, compression_level: Option<u8>) -> Result<(), AppError> {
    if input_path.is_dir() {
        let mut paths: Vec<_> = WalkDir::new(input_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "png")
            })
            .map(|e| e.path().to_owned())
            .collect();
        paths.sort();
        paths.par_iter().for_each(|path| {
            if let Err(e) = process_image(path, compression_level) {
                eprintln!("Failed to process {}: {}", path.display(), e);
            }
        });
    } else if input_path.is_file() {
        if input_path.extension().map_or(false, |ext| ext == "png") {
            if let Err(e) = process_image(input_path, compression_level) {
                eprintln!("Failed to process {}: {}", input_path.display(), e);
            }
        } else {
            eprintln!("File is not a PNG: {}", input_path.display());
        }
    } else {
        return Err(AppError {
            message: format!("Invalid input path: {}", input_path.display()),
        });
    }
    Ok(())
}
