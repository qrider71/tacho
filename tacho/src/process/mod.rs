use std::process::Command;
use std::time::Instant;

mod options;

use options::TachoOptions;

pub enum TachoOutput {
    NoOutput,
    FullOutput(String),
}

pub struct TachoResult {
    duration: u128,
    output: TachoOutput,
}

pub fn run_processes(
    cmd_name: &str,
    args: Vec<&str>,
    tacho_options: &TachoOptions,
) -> Result<Vec<TachoResult>, std::io::Error> {
    let mut results: Vec<TachoResult> = Vec::new();

    for _i in 0..tacho_options.repeat {
        let start = Instant::now();
        let output = Command::new(cmd_name).args(&args).output();
        let duration = start.elapsed().as_millis();

        match output {
            Ok(out) => results.push(TachoResult {
                duration: duration,
                output: if tacho_options.show_output {
                    TachoOutput::FullOutput(String::from_utf8(out.stdout).unwrap())
                } else {
                    TachoOutput::NoOutput
                },
            }),
            Err(e) => return Err(e),
        };
    }
    return Ok(results);
}

pub fn run_command(args: Vec<&str>) -> Result<(), String> {
    let command_line = options::get_command_line(args);
    return match command_line {
        Ok((cmd, params, tacho_options)) => match run_processes(cmd, params, &tacho_options) {
            Ok(results) => process_results(results, &tacho_options),
            Err(e) => Err(e.to_string()),
        },
        Err(s) => Err(s),
    };
}

fn process_results(results: Vec<TachoResult>, tacho_options: &TachoOptions) -> Result<(), String> {
    match results.as_slice() {
        [] => (),
        [result] => process_single_result(result, tacho_options),
        _ => process_result_list(results, tacho_options),
    }
    return Ok(());
}

fn process_single_result(result: &TachoResult, tacho_options: &TachoOptions) {
    match &result.output {
        TachoOutput::FullOutput(s) => println!("{}", s),
        TachoOutput::NoOutput => (),
    }
    println!(
        "[ Tacho: {} duration[{}ms] ]",
        tacho_options.tag, result.duration
    );
}

fn process_result_list(results: Vec<TachoResult>, tacho_options: &TachoOptions) {
    if tacho_options.show_output {
        for res in &results {
            process_single_result(&res, tacho_options);
        }
    }

    let durations:Vec<u128> = results.iter().map(|x| x.duration).collect();
    let min = durations.iter().min().unwrap_or(&0);
    let max = durations.iter().max().unwrap_or(&0);
    let n = &results.len();
    let nn = *n as f64;
    let sum:u128 = durations.iter().sum();
    let avg = sum as f64 / nn;
    let diff_sqt:f64 = durations.iter().map(|x| *x as f64 - avg).map(|x| x*x).sum();
    let variance = diff_sqt / (nn - 1.0); // Bessel correction in variance
    let stddev = variance.sqrt();
     
    println!("avg: {:.2}ms / min: {}ms / max: {}ms / stddev {:.2}ms", avg, min, max, stddev);

}
