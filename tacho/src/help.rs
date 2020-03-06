pub fn display_help() -> Result<(), String> {
    let help = "
        PerfTacho V 0.4.0 (c)2020 by Markus Reith

        PerfTacho executes a command and measures the elapsed time in milliseconds.

        Usage:
        perftacho [-tachoOptions] <command> [params]

        Available options:
         -tachoRepeat=<n>       repeats the execution n times and calculates statistics
         -tachoShowDetails      together  with -tachoRepeat: show each individual duration
         -tachoShowOutput       displays the output of the executed command
         -tachoASCII            together with -tachoShowOutput: filter out non ASCII characters in output
         -tachoTag=<tag>        adds an informational tag to the output
         -tachoRegEx[=<re>]     capture durations from output (see documentation)

        Example: 
            perftacho -tachoTag=MyTest -tachoRepeat=5 -tachoShowDetail curl https://www.google.com 
            perftacho -tachoShowOutput ls -l

        Further information: https://github.com/qrider71/tacho
        
    ";
    println!("{}", help);
    Ok(())
}
