// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

extern crate lalrpop;
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
