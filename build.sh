#!/bin/bash

echo Building Rust ...
cargo clean
cargo build --release
