// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

#[inline]
fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

#[inline]
fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

#[inline]
fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

/// Get the executable root.
fn get_executable_lib_path() -> String {
    let p = std::env::current_exe().unwrap();
    let p = p.parent().unwrap().parent().unwrap().join("./lib");
    p.to_str().unwrap().to_string()
}

/// Return the platform string e.g., "macos", "linux" and "windows".
fn platform_string() -> &'static str {
    if is_windows() {
        "windows"
    } else if is_macos() {
        "macos"
    } else if is_linux() {
        "linux"
    } else {
        panic!("un-supported platform");
    }
}

/// Returns the clang-rt static library directory.
pub fn get_clang_rt_lib_dir() -> String {
    let lib = get_executable_lib_path();
    let platform = platform_string();
    PathBuf::from(lib)
        .join("wasi")
        .join(platform)
        .to_str()
        .unwrap()
        .to_string()
}
