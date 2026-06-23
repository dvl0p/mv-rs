use std::env;
use std::fmt;
use std::process::exit;

mod moves;

#[derive(Debug)]
enum ArgsError {
    MissingSourcePath,
    MissingDestinationPath,
    UnknownArg(String),
    UnknownFlag(String),
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgsError::MissingSourcePath => {
                write!(f, "missing move source path")
            }
            ArgsError::MissingDestinationPath => {
                write!(f, "missing move destination path")
            }
            ArgsError::UnknownArg(arg) => {
                write!(f, "unknown argument \"{arg}\"")
            }
            ArgsError::UnknownFlag(flag) => {
                write!(f, "unknown flag \"{flag}\"")
            }
        }
    }
}

fn main() {
    let config = match parse_args(env::args().collect()) {
        Ok(cfg) => cfg,
        Err(error) => {
            eprintln!("error: {error}");
            exit(1)
        }
    };
    if let Err(error) = execute(config) {
        eprintln!("error: {error}");
        exit(1);
    };
}

fn execute(config: moves::MoveConfig) -> Result<(), moves::MoveError> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<moves::MoveConfig, ArgsError> {
    let mut verbose = false;
    let mut src = String::new();
    let mut dst = String::new();
    for arg in args.iter().skip(1) {
        if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else if arg.starts_with('-') {
            return Err(ArgsError::UnknownFlag(arg.clone()));
        } else if src.is_empty() {
            src = arg.clone();
        } else if dst.is_empty() {
            dst = arg.clone();
        } else {
            return Err(ArgsError::UnknownArg(arg.clone()));
        }
    }
    if src.is_empty() {
        return Err(ArgsError::MissingSourcePath);
    }
    if dst.is_empty() {
        return Err(ArgsError::MissingDestinationPath);
    }
    Ok(moves::MoveConfig::new(verbose, src, dst))
}
