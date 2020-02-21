use std::process::{Command, Output};
use std::io::{self, Write};
use std::time::Instant;

mod options;

pub fn run_process(cmd_name: &str, args: Vec<&str>) -> Result<(u128, Output), std::io::Error> {
    let start = Instant::now();
    let output = Command::new(cmd_name).args(args).output();
    let duration = start.elapsed().as_millis();

    return match output {
        Ok(out) => Ok((duration, out)),
        Err(e) => Err(e),
    };
}

pub fn run_command(args: Vec<&str>) -> Result<(), String> {
    let command_line = options::get_command_line(args);

    match command_line {
        Ok((cmd, params, tacho_options)) => {
            let output_result = run_process(cmd, params);
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
        },
        Err(s) => return Err(s)
    }
}
