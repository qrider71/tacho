use std::process::Command;
use std::time::Instant;

mod options;

pub enum TachoOutput {
    NoOutput,
    FullOutput(String),
}

pub struct TachoResult {
    duration: u128,
    output: TachoOutput,
}

pub fn run_process(
    cmd_name: &str,
    args: Vec<&str>,
    tacho_options: &options::TachoOptions,
) -> Result<TachoResult, std::io::Error> {
    let start = Instant::now();
    let output = Command::new(cmd_name).args(args).output();
    let duration = start.elapsed().as_millis();

    return match output {
        Ok(out) => Ok(TachoResult {
            duration: duration,
            output: if tacho_options.quiet {
                TachoOutput::NoOutput
            } else {
                TachoOutput::FullOutput(String::from_utf8(out.stdout).unwrap())
            },
        }),
        Err(e) => Err(e),
    };
}

pub fn run_command(args: Vec<&str>) -> Result<(), String> {
    let command_line = options::get_command_line(args);
    match command_line {
        Ok((cmd, params, tacho_options)) => {
            let run_result = run_process(cmd, params, &tacho_options);
            match run_result {
                Ok(result) => return process_output(result, &tacho_options),
                Err(e) => return Err(e.to_string()),
            };
        }
        Err(s) => return Err(s),
    }
}

fn process_output(
    result: TachoResult,
    tacho_options: &options::TachoOptions,
) -> Result<(), String> {
    match result.output {
        TachoOutput::FullOutput(s) => println!("{}", s),
        TachoOutput::NoOutput => (),
    }
    println!(
        "[ Tacho: {} duration[{}ms] ]",
        tacho_options.tag, result.duration
    );
    return Ok(());
}
