#!/bin/bash
set -e
cargo fmt --all
cargo clippy --all-targets -- -D warnings
