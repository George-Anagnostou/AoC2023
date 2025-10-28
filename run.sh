#!/bin/zsh

echo "running: cargo run -p day-$1 --bin part$2"
cargo run -p day-$1 --bin part$2
