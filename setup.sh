#!/bin/bash

rustup default stable
rustup component add llvm-tools-preview --toolchain stable-x86_64-unknown-linux-gnu
cargo +stable install cargo-llvm-cov --locked
