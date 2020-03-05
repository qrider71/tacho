[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/qrider71/tacho) 

# PerfTacho

PerfTacho is a small application for measuring performance of an executable

PerfTacho executes a command and measures the elapsed time in milliseconds.

# Usage

        perftacho [-tachoOptions] <command> [params]

        Available options:
         -tachoRepeat=<n>       repeats the execution n times and calculates statistics
         -tachoShowDetails       together  with -tachoRepeat: show each individual duration
         -tachoShowOutput       displays the output of the executed command
         -tachoASCII            together with -tachoShowOutput: filter out non ASCII characters in output
         -tachoTag=<tag>        adds an informational tag to the output 

## Example

         perftacho -tachoTag=MyTest -tachoRepeat=5 -tachoShowDetail curl https://www.google.com 
         perftacho -tachoShowOutput ls -l

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
- Grabbing perfromance data from the output of the executable with regular expressions

## V.2.0

Version 2.0 provides features for comparing the performance of several execurtables. It can be used for benchmarking
different implementation. The execution plans can be specifyed in a configuration file.

- Configuration file (yaml) based performance testing plans for several executables
- Exporting the results in different formats

## V.3.0

Version 3.0 provides multi-threading options for running the perfromance tests. The focus is on measuring
the performance under multi-threaded access and stress test conditions

- multi-threading options for running the performance tests
