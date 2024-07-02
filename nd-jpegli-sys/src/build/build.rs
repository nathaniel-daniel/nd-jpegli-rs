mod custom_jpegli;

use crate::custom_jpegli::BASE_DIR;
use crate::custom_jpegli::JPEGLI_SRCS;

fn main() {
    println!("cargo::rerun-if-changed=wrapper/wrapper.c");

    const SRC: &[&str] = &[
        "fake-jpegli/hwy/aligned_allocator.cc",
        "fake-jpegli/hwy/per_target.cc",
        "fake-jpegli/hwy/print.cc",
        "fake-jpegli/hwy/targets.cc",
    ];
    let mut build = cc::Build::new();
    build
        .include("custom-jpegli/include")
        .include("custom-jpegli/include/jpegli")
        .include("custom-jpegli");
    for src in SRC {
        build.file(src);
    }
    for src in JPEGLI_SRCS {
        build.file(src);
    }
    build.compile("jpegli-static");

    let mut build = cc::Build::new();
    build
        //.include("libjxl")
        //.include("libjxl/third_party/libjpeg-turbo")
        // .include(&jpegli_include_path)
        .include("fake-jpegli/include")
        .include("fake-jpegli/include/libjpeg-turbo")
        .file("wrapper/wrapper.c");
    build.compile("nd-jpegli");

    // println!("cargo::rustc-link-lib=jpegli-static");
    // println!("cargo::rustc-link-lib=hwy");

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    println!("cargo::rustc-link-lib=c++");

    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_env = "msvc"
    )))]
    println!("cargo::rustc-link-lib=stdc++");

    let current_dir = std::env::current_dir().expect("failed to get current_dir");
    println!(
        "cargo::metadata=include_libjxl={}",
        current_dir.join("libjxl").display()
    );
    println!(
        "cargo::metadata=include_libjpeg_turbo={}",
        current_dir
            .join("libjxl/third_party/libjpeg-turbo")
            .display()
    );
    //println!(
    //    "cargo::metadata=include_jpegli={}",
    //   jpegli_include_path.display()
    //);
    println!(
        "cargo::metadata=include_nd_jpegli_wrapper={}",
        current_dir.join("wrapper").display()
    );
}
