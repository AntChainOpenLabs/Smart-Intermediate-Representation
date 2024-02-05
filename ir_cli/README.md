## Introduction

This crate introduces a default command line tool with a pre-defined mock runtime (wasm runtime and Host APIs) of Smart Intermediate Representation.

## How to Use

First, build the binary of `ir_cli`

```shell
cd ir_cli
make
```

Then, there is a binary called `ir_cli` in the directory `target/debug`

Second, build the wasm bytecode of the textual IR file. For example:

```shell
./target/debug/ir_cli build ../ir_example/hello_world.ir
```

Third, run the bytecode in the pre-defined mock runtime environment:

```shell
./target/debug/ir_cli run --func greeting a.out.wasm a.out.abi.json
./target/debug/ir_cli run --func greeting2 a.out.wasm a.out.abi.json [string]
```

### Demo

Run the function `greeting2` of example `ir_example/hello_world.ir`, you will get following result:

```shell
./target/debug/ir_cli run --func greeting2 a.out.wasm a.out.abi.json test    
CALL PRINTLN: Hello Smart Intermediate Representation
result: test
```

## AI Intent

In order to use the capabilities of AI Intent, some preparations are required.

First, you need to install `solcjs` dependencies. We need to use `solcjs` to compile the solidity contract. Need to be executed in the root directory of this project

```shell
npm install
```

Second, need to build ir_cli according to the above steps and add the `ir_cli` to PATH.

Finally, execute the subcommand of ir_cli to generate tensor data of AI Intent.

```shell
ir_cli sol2tensor $input_dir_absolute_path
```

```
├- input
| ├- file1.json
| ├- file2.json
| └- ...
└- output
  ├- source_info -- infomation about contract
  ├- standard_input -- solc compiler standard input json
  ├- standard_output -- solc compiler standard output json
  ├- yul -- yul src code
  ├- sir -- Smart IR
  └- tensor -- tensor data

```