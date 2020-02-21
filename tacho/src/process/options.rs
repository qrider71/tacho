pub struct TachoOptions {
    pub tag: String,
    pub quiet: bool,
    pub repeat: i32,
}

fn get_value_as_string(x: &str) -> Option<String> {
    let tokens: Vec<_> = x.split("=").filter(|k| !k.is_empty()).collect();
    return match tokens.as_slice() {
        [_key, value] => Some(value.to_string()),
        _ => None,
    };
}

fn get_value_as_int(x: &str) -> Option<i32> {
    let tokens: Vec<_> = x.split("=").filter(|k| !k.is_empty()).collect();
    return match tokens.as_slice() {
        [_key, value] => {
            let i = value.parse::<i32>();
            return match i {
                Ok(v) => Some(v),
                _ => None,
            };
        }
        _ => None,
    };
}

fn find_in_args(args: &Vec<&str>, start_key:&str) -> Option<usize> {
    return args.iter().position(|x| x.starts_with(start_key));
}

fn get_tacho_options(args: &Vec<&str>) -> TachoOptions {
    return TachoOptions {
        tag: find_in_args(&args, "-tachoTag")
            .and_then(|n| get_value_as_string(args[n]))
            .unwrap_or("".to_string()),

        quiet: find_in_args(&args,"-tachoQuiet")
            .map(|_n| true)
            .unwrap_or(false),

        repeat: find_in_args(&args,"-tachoRepeat")
            .and_then(|n| get_value_as_int(args[n]))
            .unwrap_or(1),
    };
}

fn get_non_tacho_options(args: Vec<&str>) -> Vec<&str> {
    return args
        .into_iter()
        .filter(|s| !s.starts_with("-tacho"))
        .collect();
}

pub fn get_command_line(args: Vec<&str>) -> Result<(&str, Vec<&str>, TachoOptions),String> {
    let tacho_options = get_tacho_options(&args);
    let cmd_with_params = get_non_tacho_options(args);

    if cmd_with_params.len() == 0 {
        return Err(String::from("Missing command"));
    }

    let cmd = cmd_with_params[0];
    let params: Vec<&str> = cmd_with_params.into_iter().skip(1).collect();

    return Ok((cmd, params, tacho_options));
}


