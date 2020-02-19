use std::str;
use std::env;
use std::io::{self, Write};


mod process;

static DEFAULT_TACHO_FILE: &str = "tacho.yml";

fn display_help() -> Result<(), String> {
    let help = "
        Tacho V1.0 (c)2020 by Markus Reith

        Tacho executes a command and measures the elapsed time in milliseconds.
        Tacho can be used either in (1) command line mode or in (2) config file mode.

        Usage:
        (1) tacho [-tachoOptions] <command> [-params]
        (2) tacho -tachoFile <filename.yml>

        1) Command line mode
        Tacho executes <command> and passes the optional parameters [-params] to the command.

        2) Config file mode
        The config files specifies an execution plan for sophisticated performance measurement

    ";
    println!("{}", help);
    return Ok(());
}

fn process_tacho_file(file:&str) -> Result<(), String> {
    println!("Processing tacho file '{}'", file);
    return Ok(());
}

struct TachoOptions {
    tag: String,
    quiet: bool,
    repeat: i32
}

fn get_value_as_string (x:&str) -> Option<String> {
    let tokens:Vec<_> = x.split("=").filter(|k| !k.is_empty()).collect();
    return match tokens.as_slice() {
        [_key,value] => Some(value.to_string() ),
        _ => None
    };
}

fn get_value_as_int (x:&str) -> Option<i32> {
    let tokens:Vec<_> = x.split("=").filter(|k| !k.is_empty()).collect();
    return match tokens.as_slice() {
        [_key,value] => {
            let i = value.parse::<i32>();
            return match i {
                Ok(v) => Some(v),
                _ => None
            }
        },
        _ => None
    };
}

fn get_tacho_options(args:Vec<&str>) -> TachoOptions {

    let mut tacho_options = TachoOptions{
        tag: String::from(""),
        quiet: false,
        repeat: 1
    };

    let tacho_options_vec:Vec<&str> = args.into_iter().filter(|s| s.starts_with("-tacho")).collect();  
    
    let pos_tacho_repeat = tacho_options_vec.iter().position(|x| x.starts_with("-tachoTag") );
    match pos_tacho_repeat {
        Some(n) => tacho_options.tag = get_value_as_string(tacho_options_vec[n]).unwrap_or("".to_string()),
        None => ()
    }

    let pos_tacho_quiet = tacho_options_vec.iter().position(|x| x == &"-tachoQuiet" );
    if pos_tacho_quiet.is_some() {
        tacho_options.quiet = true;
    }

    let pos_tacho_repeat = tacho_options_vec.iter().position(|x| x.starts_with("-tachoRepeat") );
    match pos_tacho_repeat {
        Some(n) => tacho_options.repeat = get_value_as_int(tacho_options_vec[n]).unwrap_or(1),
        None => ()
    }
    
    return tacho_options;
}

fn get_non_tacho_options(args:Vec<&str>) -> Vec<&str> {
    return args.into_iter().filter(|s| !s.starts_with("-tacho")).collect();    
}

fn run_command(args:Vec<&str>) -> Result<(), String> {
    let tacho_options = get_tacho_options(args.clone());
    let cmd_with_params = get_non_tacho_options(args);

    if cmd_with_params.len() == 0 {
        return Err(String::from("Missing command"));
    }

    let cmd = cmd_with_params[0];
    let params:Vec<&str> = cmd_with_params.into_iter().skip(1).collect();

    let output_result = process::run_process(cmd, params);

    match output_result {
        Ok((duration, output)) => {
            if !tacho_options.quiet {
                print!("[ Tacho: {} duration[{}ms] ]", tacho_options.tag, duration);
                io::stdout().write_all(&output.stdout).unwrap();
            } else {
                println!("[ Tacho: {} duration[{}ms] ]", tacho_options.tag, duration);
            }
            return Ok(());
        },
        Err(e) => return Err(e.to_string())
    };
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args_str: Vec<&str> = args.iter().map(|t| &t[..]).collect();
    let res = match args_str.as_slice() {
        [] => display_help(),
        ["help"] => display_help(),
        ["-tachoFile", file] => process_tacho_file(file),
        ["-tachoFile"] => process_tacho_file(DEFAULT_TACHO_FILE),
        _ => run_command(args_str)
    };

    match res {
        Err(message) => println!("Error, Tacho failed: {} !", message),
        _ => return
    };
}
