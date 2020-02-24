pub fn display_help() -> Result<(), String> {
    let help = "
        Tacho (c)2020 by Markus Reith

        Tacho executes a command and measures the elapsed time in milliseconds.

        Usage:
        tacho [-tachoOptions] <command> [params]

        Available options:
         -tachoRepeat=<n>       repeats the execution n times and calculates statistics
         -tachoShowOutput       displays the output of the executed command
         -tachoASCII            together with -tachoShowOutput: filter out non ASCII characters in output
         -tachTag=<tag>         adds an informational tag to the output 

        Example: 
         tacho -tachoTag=MyTest -tachoRepeat=5 curl https://www.google.com 
         tacho -tachoShowOutput ls -l

        Further information: https://github.com/qrider71/tacho
        
    ";
    println!("{}", help);
    return Ok(());
}
