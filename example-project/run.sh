#!/bin/bash
VA_DIR=~/Desktop/visualad-workspace/visualad
rm -f output.png
compiler=$($VA_DIR/target/release/toml-cli get va_config.toml compiler_path | tr -d '"')
$compiler main.vizzy output.png