use std::env;
use std::process::exit;

#[derive(Debug)]
enum MoveError {
    NoSRC,
    NoDST,
    UnknownFlag(String),
    UnknownArg(String),
}

#[derive(Debug)]
struct MoveConfig {
    verbose: bool,
    src: String,
    dst: String,
}

fn main() {
    match parse_args(env::args().collect()) {
        Ok(cfg) => {
            println!("{:#?}", cfg);
        }
        Err(MoveError::NoSRC) => {
            eprintln!("error: missing source file path");
            exit(1);
        }
        Err(MoveError::NoDST) => {
            eprintln!("error: missing destination path");
            exit(1);
        }
        Err(MoveError::UnknownFlag(flag)) => {
            eprintln!("error: unknown option: '{flag}'");
            exit(1);
        }
        Err(MoveError::UnknownArg(arg)) => {
            eprintln!("error: incorrect positional argument: '{arg}'");
            exit(1);
        }
    }
}

fn parse_args(args: Vec<String>) -> Result<MoveConfig, MoveError> {
    let mut verbose = false;
    let mut src = String::new();
    let mut dst = String::new();
    for arg in args.iter().skip(1) {
        if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else if arg.starts_with('-') {
            return Err(MoveError::UnknownFlag(arg.clone()));
        } else if src.is_empty() {
            src = arg.clone();
        } else if dst.is_empty() {
            dst = arg.clone();
        } else {
            return Err(MoveError::UnknownArg(arg.clone()));
        }
    }
    if src.is_empty() {
        return Err(MoveError::NoSRC);
    }
    if dst.is_empty() {
        return Err(MoveError::NoDST);
    }
    Ok(MoveConfig { verbose, src, dst })
}
