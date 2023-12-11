// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

extern crate lalrpop;

fn main() {
    use std::process::Command;
    Command::new("make").status().unwrap();

    #[cfg(feature = "ir_release")]
    // Static link LLVM libs
    static_link_llvm();

    lalrpop::process_root().unwrap();
}

#[allow(dead_code)]
fn static_link_llvm() {
    use std::process::Command;

    println!("Use Smart Intermediate Representation Static Link");
    // compile our linker
    let cxxflags = Command::new("llvm-config")
        .args(["--cxxflags"])
        .output()
        .expect("could not execute llvm-config");

    let cxxflags = String::from_utf8(cxxflags.stdout).unwrap();

    let mut build = cc::Build::new();

    build.file("src/linker/linker.cpp").cpp(true);

    if !cfg!(target_os = "windows") {
        build.flag("-Wno-unused-parameter");
    }

    for flag in cxxflags.split_whitespace() {
        build.flag(flag);
    }

    build.compile("liblinker.a");

    // add the llvm linker
    let libdir = Command::new("llvm-config")
        .args(["--libdir"])
        .output()
        .unwrap();
    let libdir = String::from_utf8(libdir.stdout).unwrap();

    println!("cargo:libdir={libdir}");
    for lib in &["lldELF", "lldCommon", "lldWasm"] {
        //  "lldCore", "lldDriver", in llvm-12
        println!("cargo:rustc-link-lib=static={lib}");
    }

    // Add all the symbols were not using, needed by Windows and debug builds
    for lib in &["lldMachO", "lldMinGW", "lldCOFF"] {
        // "lldReaderWriter", "lldYAML",  in llvm-12
        println!("cargo:rustc-link-lib=static={lib}");
    }

    let output = Command::new("git")
        .args(["describe", "--tags"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={git_hash}");

    // Make sure we have an 8MiB stack on Windows. Windows defaults to a 1MB
    // stack, which is not big enough for debug builds
    #[cfg(windows)]
    println!("cargo:rustc-link-arg=/STACK:8388608");
}
