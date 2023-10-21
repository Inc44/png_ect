use png_ect::{process_directory, AppError};
use std::env;
use std::path::PathBuf;
use std::process;

fn parse_args() -> Result<(PathBuf, Option<u8>), AppError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        return Err(AppError {
            message: format!("Usage: {} <input-path> [-1 to -9]", args[0]),
        });
    }
    let compression_level = if args.len() == 3 {
        args[2][1..].parse::<u8>().ok()
    } else {
        None
    };
    Ok((PathBuf::from(&args[1]), compression_level))
}

fn main() {
    match parse_args()
        .and_then(|(path, compression_level)| process_directory(&path, compression_level))
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
