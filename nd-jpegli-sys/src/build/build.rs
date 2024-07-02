mod custom_jpegli;

use crate::custom_jpegli::BASE_DIR;
use crate::custom_jpegli::HIGHWAY_SRCS;
use crate::custom_jpegli::INCLUDE_DIR;
use crate::custom_jpegli::JPEGLI_INCLUDE_DIR;
use crate::custom_jpegli::JPEGLI_SRCS;

fn main() {
    println!("cargo::rerun-if-changed=wrapper/wrapper.c");

    let current_dir = std::env::current_dir().expect("failed to get current_dir");

    // Build highway
    let mut build = cc::Build::new();
    build.include(BASE_DIR).include(INCLUDE_DIR);
    for src in HIGHWAY_SRCS {
        build.file(src);
    }
    build.compile("hwy");

    // Build jpegli
    let mut build = cc::Build::new();
    build
        .include(BASE_DIR)
        .include(INCLUDE_DIR)
        .include(JPEGLI_INCLUDE_DIR);
    for src in JPEGLI_SRCS {
        build.file(src);
    }
    build.compile("jpegli-static");

    // Compile wrapper
    let mut build = cc::Build::new();
    build
        .include(INCLUDE_DIR)
        .include(JPEGLI_INCLUDE_DIR)
        .file("wrapper/wrapper.c");
    build.compile("nd-jpegli");

    // Link c++ stdlib
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    println!("cargo::rustc-link-lib=c++");
    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_env = "msvc"
    )))]
    println!("cargo::rustc-link-lib=stdc++");

    // Setup Metadata
    println!(
        "cargo::metadata=include={}",
        current_dir.join(INCLUDE_DIR).display()
    );
    println!(
        "cargo::metadata=include_jpegli={}",
        current_dir.join(JPEGLI_INCLUDE_DIR).display()
    );
    println!(
        "cargo::metadata=include_nd_jpegli={}",
        current_dir.join("wrapper").display()
    );
}
