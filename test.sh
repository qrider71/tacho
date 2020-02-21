#!/bin/bash

echo Building Rust ...
cargo clean
cargo build --release
./target/release/tacho ./tacho/test/test.sh -tachoRepeat=5 -tachoTag=TachoTest
