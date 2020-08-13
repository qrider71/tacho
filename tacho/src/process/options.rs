static DEFAULT_REG_EX: &str = r"\[(\-?\d+[\.,]?\d*)\s?(s|ms|ns)\]";

#[derive(Clone, Debug)]
pub struct TachoOptions {
    pub tag: String,
    pub show_output: bool,
    pub filter_ascii: bool,
    pub threads: i32,
    pub repeat: i32,
    pub show_details: bool,
    pub regex_opt: Option<String>,
}

fn get_value_as_string(x: &str) -> Option<String> {
    let tokens: Vec<_> = x.split('=').filter(|k| !k.is_empty()).collect();
    match tokens.as_slice() {
        [_key, value] => Some((*value).to_string()),
        _ => None,
    }
}

fn get_value_as_string_or_default(x: &str, default: &str) -> Option<String> {
    let tokens: Vec<_> = x.split('=').filter(|k| !k.is_empty()).collect();
    match tokens.as_slice() {
        [_key, value] => Some((*value).to_string()),
        _ => Some(default.to_string()),
    }
}

fn get_value_as_int(x: &str) -> Option<i32> {
    let tokens: Vec<_> = x.split('=').filter(|k| !k.is_empty()).collect();
    match tokens.as_slice() {
        [_key, value] => {
            let i = value.parse::<i32>();
            match i {
                Ok(v) => Some(v),
                _ => None,
            }
        }
        _ => None,
    }
}

fn find_in_args(args: &[&str], start_key: &str) -> Option<usize> {
    args.iter().position(|x| x.starts_with(start_key))
}

fn get_tacho_options(args: &[&str]) -> TachoOptions {
    TachoOptions {
        tag: find_in_args(&args, "-tachoTag")
            .and_then(|n| get_value_as_string(args[n]))
            .unwrap_or_else(|| "".to_string()),

        threads: find_in_args(&args, "-tachoThreads")
            .and_then(|n| get_value_as_int(args[n]))
            .unwrap_or(1),

        repeat: find_in_args(&args, "-tachoRepeat")
            .and_then(|n| get_value_as_int(args[n]))
            .unwrap_or(1),

        show_details: find_in_args(&args, "-tachoShowDetails")
            .map(|_n| true)
            .unwrap_or(false),

        show_output: find_in_args(&args, "-tachoShowOutput")
            .map(|_n| true)
            .unwrap_or(false),

        filter_ascii: find_in_args(&args, "-tachoASCII")
            .map(|_n| true)
            .unwrap_or(false),

        regex_opt: find_in_args(&args, "-tachoRegEx")
            .and_then(|n| get_value_as_string_or_default(args[n], DEFAULT_REG_EX)),
    }
}

fn get_non_tacho_options(args: Vec<&str>) -> Vec<String> {
    args.into_iter()
        .filter(|s| !s.starts_with("-tacho"))
        .map(String::from)
        .collect()
}

pub fn get_command_line(args: Vec<&str>) -> Result<(String, Vec<String>, TachoOptions), String> {
    let tacho_options = get_tacho_options(&args);
    let cmd_with_params = get_non_tacho_options(args);

    if cmd_with_params.is_empty() {
        return Err(String::from("Missing command"));
    }

    let cmd = String::from(""); // cmd_with_params.get(0).unwrap();
    let params: Vec<String> = cmd_with_params.into_iter().skip(1).collect();

    Ok((cmd, params, tacho_options))
}
