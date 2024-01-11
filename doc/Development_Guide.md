
## Installation

Firstly, install [Rust 1.60+](https://www.rust-lang.org/).

Now we suggest use Rust 1.70.

Secondly, you'll need LLVM installed and `llvm-config` in your `PATH`. Just download from [LLVM 15](https://releases.llvm.org/download.html) or install `llvm@15` using `brew`.

```sh
$brew install llvm@15
# when use brew, llvm@15 installed to ${HOMEBREW_PREFIX}/Cellar/llvm@15/15.0.7.
# The default path for ${HOMEBREW_PREFIX} is:
#   /usr/local for macOS on Intel,
#   /opt/homebrew for macOS on Apple Silicon/ARM, and
#   /home/linuxbrew/.linuxbrew for Linux.
```

Add the LLVM installation location to `LLVM_SYS_150_PREFIX` and the `$PATH`.

```sh
export LLVM_SYS_150_PREFIX=<your LLVM 15 install location>
export PATH=<your LLVM 15 install location>/bin:$PATH
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