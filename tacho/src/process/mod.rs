use std::error::Error;
use std::process::{Command, Output};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

mod options;
mod regex_parse;
mod stats;

use options::TachoOptions;

pub enum TachoOutput {
    NoOutput,
    FullOutput(String),
}

pub struct TachoResult {
    duration: Vec<f64>,
    output: TachoOutput,
}

fn to_ascii(v: Vec<u8>) -> Vec<u8> {
    v.into_iter().filter(|&b| b < 128u8).collect()
}

fn create_tacho_result(
    out: Output,
    duration: f64,
    tacho_options: TachoOptions,
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
                duration: tacho_options
                    .regex_opt
                    .as_ref()
                    .map(|regex| regex_parse::parse_durations(duration, &s, &regex))
                    .unwrap_or_else(|| vec![duration]),
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
            duration: vec![duration],
            output: TachoOutput::NoOutput,
        })
    }
}

fn run_processes(
    cmd_name: String,
    args: Vec<String>,
    tacho_options: TachoOptions,
) -> Result<Vec<TachoResult>, String> {
    let mut results: Vec<TachoResult> = Vec::new();

    let (tx, rx): (Sender<TachoResult>, Receiver<TachoResult>) = mpsc::channel();
    let mut children = Vec::new();

    let nThreads = tacho_options.threads;
    let nRepeats = tacho_options.repeat;
    let cmd = "";

    for _i in 0..nThreads {
        let thread_tx = tx.clone();
        let child = thread::spawn(move || {
            for _j in 0..nRepeats {
                let start = Instant::now();
                let output = Command::new(cmd).args(vec![""]).output();
                let duration = start.elapsed().as_millis() as f64;

                let tacho_options_cloned = TachoOptions {
                    tag: String::from(""),
                    threads: nThreads,
                    repeat: 1,
                    show_details: false,
                    show_output: false,
                    filter_ascii: false,
                    regex_opt: None,
                };

                let res = match output {
                    Ok(out) => create_tacho_result(out, duration, tacho_options_cloned.clone()),
                    Err(e) => Err(e.description().to_owned()),
                };

                match res {
                    Ok(tacho_result) => thread_tx.send(tacho_result).unwrap(),
                    Err(msg) => panic!("Calling command failed {}", msg),
                }
            }
        });
        children.push(child);
    }
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    Ok(results)
}

pub fn run_command(args: Vec<&str>) -> Result<(), String> {
    let command_line = options::get_command_line(args);
    match command_line {
        Ok((cmd, params, tacho_options)) => match run_processes(cmd, params, tacho_options.clone())
        {
            Ok(results) => process_results(results, tacho_options),
            Err(msg) => Err(msg),
        },
        Err(s) => Err(s),
    }
}

fn process_results(results: Vec<TachoResult>, tacho_options: TachoOptions) -> Result<(), String> {
    match results.as_slice() {
        [] => (),
        [result] => process_single_result(result, tacho_options),
        _ => process_result_list(results, tacho_options),
    }
    Ok(())
}

fn process_single_result(result: &TachoResult, tacho_options: TachoOptions) {
    match &result.output {
        TachoOutput::FullOutput(s) => println!("{}", s),
        TachoOutput::NoOutput => (),
    }
    println!(
        "[ Tacho: {} duration[{}ms] ]",
        tacho_options.tag, result.duration[0]
    );
}

fn process_result_list(results: Vec<TachoResult>, tacho_options: TachoOptions) {
    if tacho_options.show_output {
        for res in &results {
            process_single_result(&res, tacho_options.clone());
        }
    }

    if tacho_options.show_details {
        println!("Tacho {}: duration in ms", tacho_options.tag);
        for res in &results {
            for d in &res.duration {
                print!("{:.2} \t", d);
            }
            println!();
        }
    }

    let sizes: Vec<usize> = results.iter().map(|x| x.duration.len()).collect();
    let min_size = sizes.iter().min().unwrap_or_else(|| &0);
    for i in 0..*min_size {
        let durations: Vec<f64> = results.iter().map(|x| x.duration[i]).collect();
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
}
