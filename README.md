[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/qrider71/tacho) 

# PerfTacho

PerfTacho is a small application for computing performance statistics of an executable.

The executable can be any binary file or a shell script. PerfTacho measures the total execution
time and can also capture execution time output from the executable. PerfTacho can run the executable
multiple times and compute statistics from the recorded performance data.

# Usage

        perftacho [-tachoOptions] <command> [params]

        Available options:
         -tachoRepeat=<n>       repeats the execution n times and calculates statistics
         -tachoShowDetails      together  with -tachoRepeat: show each individual duration
         -tachoShowOutput       displays the output of the executed command
         -tachoASCII            together with -tachoShowOutput: filter out non ASCII characters in output
         -tachoTag=<tag>        adds an informational tag to the output
         -tachoRegEx[=<re>]     capture durations from output (see documentation below)

## Example

         perftacho -tachoTag=MyTest -tachoRepeat=5 -tachoShowDetail curl https://www.google.com 
         perftacho -tachoShowOutput ls -l

# Typical use cases

## Evaluating performance of an executable and its algorithms

A software developer wants to evaluate the performance of an executable which comprises of several algorithms.
Typcally, the developer wants to measure the total execution time and the individual execution time for the
algorithms to track where the elapsed time is spent. For this purpose the developer just needs to surround
the execution of the algorithms with execution time logging and write the measured execution time to stdout.

Example (Java):

```Java
long startTime = System.currentTimeMillis();
runMyAlgorithm();
long duration = System.currentTimeMillis() - startTime;
System.out.printf("Duration MyAlgorithm [%d ms]", duration);
```

The excutable should produce the following output:

        Duration MyAlgorithm [321 ms]

If the multiple algorithms shall be measured just add the output accordingly, i.e:

        Algo1 [321 ms] ... some other output ... Algo2 [456 ms]

PerfTacho parses the output of the executable and caputures the performance data with a regular
expression. With the current default regular expression 

        "\[(\-?\d+[\.,]?\d*)\s?(s|ms|ns)\]"

it matches the following output examples:

        [123.0 ms] [123.0ms] [123 s] [123,45 s] [12345ns]

The follwing command runs the executable in multiple passes and computes statistics from the recorded
performance data:

        perftacho -tachoRepeat=5 -tachoShowDetails -tachoRegEx  MyProgram

The output might look like this:

        Tacho : duration in ms
        1012.00         102.00  100.02  119.00 
        1014.00         116.00  100.01  107.00 
        1014.00         114.00  100.02  115.00 
        1013.00         105.00  100.01  103.00 
        1014.00         112.00  100.02  105.00 
        Tacho : avg: 1013.40ms / 95% conf. interval 0.78 / min: 1012ms / max: 1014ms / stddev 0.89 ms / n_recommended 1
        Tacho : avg: 109.80ms / 95% conf. interval 5.27 / min: 102ms / max: 116ms / stddev 6.02 ms / n_recommended 5
        Tacho : avg: 100.01ms / 95% conf. interval 0.00 / min: 100.009ms / max: 100.018ms / stddev 0.00 ms / n_recommended 1
        Tacho : avg: 109.80ms / 95% conf. interval 6.02 / min: 103ms / max: 119ms / stddev 6.87 ms / n_recommended 6

In this example the executable "MyProgramm" was run 5 times. PerTacho captured 3 performance data items
from the output. From this data the table as shown above is generated (and displayed because of the -tachoShowDetails command line option).

The first column shows the total execution time, the last 3 columns show
the captured execution time data from the output. All data is transformed to milliseconds.

# Installation

Perftacho is a self contained binary file without any dependencies. You have the following options:

## Build and install the binary from GitHub sources

You need to have Rust and Cargo installed
Please consult https://doc.rust-lang.org/cargo/getting-started/installation.html

        Get the sources:
        git clone https://github.com/qrider71/tacho.git

        cd tacho
        cargo build --release
        cd target/release

        Copy the perftacho binary to your binary folder (wgich should be in your path),
        e.g. on Linux:

        sudo cp perftacho /usr/local/bin/

## Get and build the latest release from crates

You need to have Rust and Cargo installed
Please consult https://doc.rust-lang.org/cargo/getting-started/installation.html

        cargo install perftacho

Cargo installs the compiled binary into your bin folder

## Mac OSX

You can install from sources as described above or download the binary from Github:
https://github.com/qrider71/tacho/releases

You should pick the file perftacho-osx-x.y.z and copy it to your bin folder (which should be in your PATH)

Alternatively, you can install with homebrew (https://brew.sh/)

        brew tab qrider71/perftacho
        brew install perftacho

## Linux

You can install from sources as described above or download the binary from Github:
https://github.com/qrider71/tacho/releases

You should pick the file perftacho-linux-x.y.z and copy it to your bin folder (which should be in your PATH)

## Windows

You can install from sources as described above or download the binary from Github:
https://github.com/qrider71/tacho/releases

You should pick the file perftacho-windows-x.y.z.exe and copy it to your bin folder (which should be in your PATH)

# Roadmap

I plan the following roadmap. Please contact me if you have any comments or ideas for useful features:

## V.1.0

Version 1.0 provides all command line tool features for measurung the performance of a single executable

- Simple measurement of the runtime duration (implemented)
- Statistical analysis of multiple runs (implemented)
- Grabbing perfromance data from the output of the executable with regular expressions (implemented)

## V.2.0

Version 2.0 provides features for comparing the performance of several execurtables. It can be used for benchmarking
different implementation. The execution plans can be specifyed in a configuration file.

- Configuration file (yaml) based performance testing plans for several executables
- Exporting the results in different formats

## V.3.0

Version 3.0 provides multi-threading options for running the perfromance tests. The focus is on measuring
the performance under multi-threaded access and stress test conditions

- multi-threading options for running the performance tests
