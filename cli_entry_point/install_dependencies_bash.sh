#!/bin/bash

curl --proto '=https' -sSf https://sh.rustup.rs | sh
cargo install cargo-edit
cd ../repo_analyzer
cargo add reqwest
cargo add serde
cargo add serde_json
cargo add substring
echo "6 dependencies installed..."
