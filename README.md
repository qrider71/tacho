[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/qrider71/tacho) 

# tacho

Tacho is a small application for measuring performance of an executable


Tacho executes a command and measures the elapsed time in milliseconds.

## Usage
        
        tacho [-tachoOptions] <command> [params]

        Available options:
         -tachoRepeat=<n>       repeats the execution n times and calculates statistics
         -tachoShowOutput       displays the output of the executed command
         -tachTag=<tag>         adds an informational tag to the output 

## Example 

         tacho -tachoTag=MyTest -tachoRepeat=5 curl https://www.google.com 
         tacho -tachoShowOutput ls -l

