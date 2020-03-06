pub fn parse_durations(total_duration: f64, output: &str, regex_str: &str) -> Vec<f64> {
    let mut durations = vec![total_duration];
    let re_result = regex::Regex::new(regex_str);
    match re_result {
        Ok(re) => {
            for cap in re.captures_iter(output) {
                let n = cap.len();
                match n {
                    2 => {
                        let c = cap[1].replace(",", ".");
                        let d_res = c.parse::<f64>();
                        if let Ok(d) = d_res {
                            durations.push(d)
                        }
                    }
                    3 => {
                        let c = cap[1].replace(",", ".");
                        let d_res = c.parse::<f64>();
                        let u = &cap[2];

                        match (d_res, u) {
                            (Ok(d), "s") => durations.push(d * 1000.0),
                            (Ok(d), "ms") => durations.push(d),
                            (Ok(d), "ns") => durations.push(d / 1000.0),
                            (Err(e), _) => println!("Failed parsing duration: {}", e),
                            (_, u) => {
                                println!("Unknown time unit: {}, valid units are s, ms, ns", u)
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Err(e) => {
            println!("Ignoring regular expression with error:\n {}", e);
        }
    }

    durations
}
