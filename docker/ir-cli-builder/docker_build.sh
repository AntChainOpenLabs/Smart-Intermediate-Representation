#!/usr/bin/env bash
# Note: run the script under the root directory of the project
# This script allow 1 optional input argument:
# - if $1 is "prepare", only prepare the dependencies for building
# - otherwise(by default), do build a image

# 1. copy the cargo.toml file from ir_cli, and delete following lines, since they are relative dependencies and can not be reached in docker build context
# smart_ir_macro = { path = "../smart_ir_macro", version = "0.3.0" }
# smart_ir = { path = "../smart_ir" }
input_file="ir_cli/Cargo.toml"
output_file="docker/ir-cli-builder/Cargo.toml"
rm -rf $output_file

while IFS= read -r line
do
  if [[ "$line" =~ ^smart_ir_macro\ =\ \{[[:space:]]*path\ =\ \"\.\.\/smart_ir_macro\"  || "$line" =~ ^smart_ir\ =\ \{[[:space:]]*path\ =\ \"\.\.\/smart_ir\" ]]
  then
    continue
  fi
  echo "$line" >> "$output_file"
done < "$input_file"

if [ "$1" = "prepare" ]; then
  echo "done prepare docker build context"
  ls $output_file
else
  # 2. docker build
  # for macOS Arm64 environment, it's required to add the `--platform linux/amd64` option to the docker build/run commands.
  cd docker/ir-cli-builder
  docker build . --platform linux/x86_64 -f Dockerfile -t smartir/smart-ir-builder:v0.1.0
  cd ../../

  # 3. clean tmp files
  rm $output_file
fi

