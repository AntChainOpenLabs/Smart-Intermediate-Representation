# Compiler-rt for wasm32 target

## How to use

1. Copy full folder of `lib` to your project manually.
2. Replace the appropriate `src` and `dst` args of `copy_dir_all` in your own `build.rs`, following code is an example:

```rust
// build.rs
use std::{fs, io, path::Path};

fn main() {
    // Copy static libs to the target folder.
    copy_libs().unwrap();
}

/// Copy all std lib deps
fn copy_libs() -> io::Result<()> {
    // Copy static libs to the target folder.
    copy_dir_all("../lib", "./target/lib")?;
    copy_dir_all("../lib", "./target/debug/lib")?;
    copy_dir_all("../lib", "./target/release/lib")?;
    copy_dir_all("../lib", "./target/llvm-cov-target/release/lib")?;
    Ok(())
}

/// Copy all files in a folder from `src` to `dst`.
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
```

Please also refer to crate `ir_cli` for more [details](../ir_cli).