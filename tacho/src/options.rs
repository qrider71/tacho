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

pub fn get_tacho_options(args: Vec<&str>) -> TachoOptions {
    return TachoOptions {
        tag: args
            .iter()
            .position(|x| x.starts_with("-tachoTag"))
            .and_then(|n| get_value_as_string(args[n]))
            .unwrap_or("".to_string()),

        quiet: args
            .iter()
            .position(|x| x == &"-tachoQuiet")
            .map(|_n| true)
            .unwrap_or(false),

        repeat: args
            .iter()
            .position(|x| x.starts_with("-tachoRepeat"))
            .and_then(|n| get_value_as_int(args[n]))
            .unwrap_or(1),
    };
}

pub fn get_non_tacho_options(args: Vec<&str>) -> Vec<&str> {
    return args
        .into_iter()
        .filter(|s| !s.starts_with("-tacho"))
        .collect();
}
