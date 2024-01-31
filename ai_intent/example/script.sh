#!/bin/bash

script_dir=$(cd "$(dirname "$0")" && pwd)

input_dir="$script_dir/input"
root=$(cd "$script_dir/../.." && pwd)

# cd "$root" & npm install

ir_cli sol2tensor "$input_dir"
