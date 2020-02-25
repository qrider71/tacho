use std::env;
use std::str;

mod help;
mod process;

static DEFAULT_TACHO_FILE: &str = "tacho.yml";

fn process_tacho_file(file: &str) -> Result<(), String> {
    println!("Processing tacho file '{}'", file);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args_str: Vec<&str> = args.iter().map(|t| &t[..]).collect();
    let res = match args_str.as_slice() {
        [] => help::display_help(),
        ["help"] => help::display_help(),
        ["-tachoFile", file] => process_tacho_file(file),
        ["-tachoFile"] => process_tacho_file(DEFAULT_TACHO_FILE),
        _ => process::run_command(args_str),
    };

    if let Err(message) = res {
        println!("Error, Tacho failed: {} !", message)
    }
}
