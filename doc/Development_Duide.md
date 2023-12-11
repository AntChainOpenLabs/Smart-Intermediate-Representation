
## Installation

Firstly, install [Rust 1.60+](https://www.rust-lang.org/).

Now we suggest use Rust 1.69.

Secondly, you'll need LLVM installed and `llvm-config` in your `PATH`. Just download from [LLVM 14](https://releases.llvm.org/download.html) or install `llvm@14` using `brew`.

```sh
$brew install llvm@14
# when use brew, llvm@14 installed to /usr/local/Cellar/llvm@14/14.0.6 by default
```

Add the LLVM installation location to `LLVM_SYS_140_PREFIX` and the `$PATH`.

```sh
export LLVM_SYS_140_PREFIX=<your LLVM 14 install location>
export PATH=<your LLVM 14 install location>/bin:$PATH
```

Go to the `ir_cli` directory:

```sh
$cd ir_cli
```

Next, install wasm target dependencies.

```sh
$make install-rustc-wasm
```

Last, build the project:

```sh
$make
```

## Release

```
install Rust 1.60+ and llvm following the document's "Installation" section

$cd ir_cli

$make release
```

## Run

* Linux: IR linux binary require centos 8 version. Otherwise you can install libtinfo.so.6 and glibc 6 manually.

For more information, please refer to [ir_cli](../ir_cli/README.md)