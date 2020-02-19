pub fn display_help() -> Result<(), String> {
    let help = "
        Tacho V1.0 (c)2020 by Markus Reith

        Tacho executes a command and measures the elapsed time in milliseconds.
        Tacho can be used either in (1) command line mode or in (2) config file mode.

        Usage:
        (1) tacho [-tachoOptions] <command> [-params]
        (2) tacho -tachoFile <filename.yml>

        1) Command line mode
        Tacho executes <command> and passes the optional parameters [-params] to the command.

        2) Config file mode
        The config files specifies an execution plan for sophisticated performance measurement

    ";
    println!("{}", help);
    return Ok(());
}