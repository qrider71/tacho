#!/bin/bash

echo Running clippy ...
cargo clippy
echo Running formatter ...
cargo fmt
echo Building Rust ...
cargo clean
cargo build --release

echo "Prepare for homebrew"
cd target/release
tar -czf pertacho-osx.tar.gz perftacho
shasum -a 256 pertacho-osx.tar.gz > perftacho.sha256
