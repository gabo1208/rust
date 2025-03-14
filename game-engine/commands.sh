#!/bin/bash
# to build the program targeting wasm
cargo build --example game --target wasm32-unknown-unknown

# to run a local devserver in other tab but in the root dir
devserver # install using cargo install devserver before
