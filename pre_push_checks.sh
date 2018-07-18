#!/bin/bash -e

cargo fmt --all
cargo clippy -- -Dclippy -Wclippy_pedantic
cargo test -- --test-threads=1
