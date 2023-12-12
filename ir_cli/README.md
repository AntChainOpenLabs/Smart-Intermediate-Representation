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
