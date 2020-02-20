use std::env;
use std::io::{self, Write};
use std::str;

mod help;
mod process;
mod options;

static DEFAULT_TACHO_FILE: &str = "tacho.yml";

fn process_tacho_file(file: &str) -> Result<(), String> {
    println!("Processing tacho file '{}'", file);
    return Ok(());
}

fn run_command(args: Vec<&str>) -> Result<(), String> {
    let tacho_options = options::get_tacho_options(args.clone());
    let cmd_with_params = options::get_non_tacho_options(args);

    if cmd_with_params.len() == 0 {
        return Err(String::from("Missing command"));
    }

    let cmd = cmd_with_params[0];
    let params: Vec<&str> = cmd_with_params.into_iter().skip(1).collect();

    let output_result = process::run_process(cmd, params);

    match output_result {
        Ok((duration, output)) => {
            if !tacho_options.quiet {
                io::stdout().write_all(&output.stdout).unwrap();
            }
            println!("[ Tacho: {} duration[{}ms] ]", tacho_options.tag, duration);
            return Ok(());
        }
        Err(e) => return Err(e.to_string()),
    };
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args_str: Vec<&str> = args.iter().map(|t| &t[..]).collect();
    let res = match args_str.as_slice() {
        [] => help::display_help(),
        ["help"] => help::display_help(),
        ["-tachoFile", file] => process_tacho_file(file),
        ["-tachoFile"] => process_tacho_file(DEFAULT_TACHO_FILE),
        _ => run_command(args_str),
    };

    match res {
        Err(message) => println!("Error, Tacho failed: {} !", message),
        _ => return,
    };
}
