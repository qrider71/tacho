use std::error::Error;
use std::process::{Command, Output};
use std::time::Instant;

mod options;
mod stats;

use options::TachoOptions;

pub enum TachoOutput {
    NoOutput,
    FullOutput(String),
}

pub struct TachoResult {
    duration: f64,
    output: TachoOutput,
}

fn to_ascii(v: Vec<u8>) -> Vec<u8> {
    v.into_iter().filter(|&b| b < 128u8).collect()
}

fn create_tacho_result(
    out: Output,
    duration: f64,
    tacho_options: &TachoOptions,
) -> Result<TachoResult, String> {
    if tacho_options.show_output || tacho_options.regex_opt.is_some() {
        let v = if tacho_options.filter_ascii {
            to_ascii(out.stdout)
        } else {
            out.stdout
        };

        let res = String::from_utf8(v);
        match res {
            Ok(s) => Ok(TachoResult {
                duration,
                output: if tacho_options.show_output {
                    TachoOutput::FullOutput(s)
                } else {
                    TachoOutput::NoOutput
                },
            }),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Ok(TachoResult {
            duration,
            output: TachoOutput::NoOutput,
        })
    }
}

fn run_process(
    cmd_name: &str,
    args: &[&str],
    tacho_options: &TachoOptions,
) -> Result<TachoResult, String> {
    let start = Instant::now();
    let output = Command::new(cmd_name).args(args).output();
    let duration = start.elapsed().as_millis() as f64;

    match output {
        Ok(out) => create_tacho_result(out, duration, tacho_options),
        Err(e) => Err(e.description().to_owned()),
    }
}

fn run_processes(
    cmd_name: &str,
    args: &[&str],
    tacho_options: &TachoOptions,
) -> Result<Vec<TachoResult>, String> {
    let mut results: Vec<TachoResult> = Vec::new();

    for _i in 0..tacho_options.repeat {
        let res = run_process(cmd_name, args, tacho_options);
        match res {
            Ok(tacho_result) => results.push(tacho_result),
            Err(msg) => return Err(msg),
        }
    }
    Ok(results)
}

pub fn run_command(args: Vec<&str>) -> Result<(), String> {
    let command_line = options::get_command_line(args);
    match command_line {
        Ok((cmd, params, tacho_options)) => match run_processes(cmd, &params, &tacho_options) {
            Ok(results) => process_results(results, &tacho_options),
            Err(msg) => Err(msg),
        },
        Err(s) => Err(s),
    }
}

fn process_results(results: Vec<TachoResult>, tacho_options: &TachoOptions) -> Result<(), String> {
    match results.as_slice() {
        [] => (),
        [result] => process_single_result(result, tacho_options),
        _ => process_result_list(results, tacho_options),
    }
    Ok(())
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

    if tacho_options.show_details {
        println!("Tacho {}: duration in ms", tacho_options.tag);
        for res in &results {
            println!("{:.2}", res.duration);
        }
    }

    let durations: Vec<f64> = results.iter().map(|x| x.duration).collect();
    let stats::Stats {
        avg,
        conf_interval_95,
        min,
        max,
        stddev,
        n_recommended,
    } = stats::calculate_stats(&durations);

    println!(
        "Tacho {}: avg: {:.2}ms / 95% conf. interval {:.2} / min: {}ms / max: {}ms / stddev {:.2} ms / n_recommended {:.0}",
        tacho_options.tag, avg, conf_interval_95, min, max, stddev, n_recommended
    );
}
