use std::process::{Command, Output};
use std::time::Instant;

pub fn run_process(cmd_name: &str, args: Vec<&str>) -> Result<(u128, Output), std::io::Error> {
    let start = Instant::now();
    let output = Command::new(cmd_name).args(args).output();
    let duration = start.elapsed().as_millis();

    return match output {
        Ok(out) => Ok((duration, out)),
        Err(e) => Err(e),
    };
}
