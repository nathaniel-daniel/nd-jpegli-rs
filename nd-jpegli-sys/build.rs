fn main() {
    println!("cargo:rerun-if-env-changed=CMAKE_GENERATOR");
    println!("cargo::rerun-if-changed=wrapper/wrapper.c");
    println!("cargo::rerun-if-changed=wrapper/nd_jpegli_rs.c");

    let mut config = cmake::Config::new("libjxl");

    // On Windows MSVC, [Rust will always link to the Release CRT](https://github.com/rust-lang/rust/issues/39016).
    // However, cmake will link to the Debug CRT if it is building for debug.
    // As a result, let's just force Release mode if we are building for Debug.
    // This solution also allows opt-level='s' and friends to work the same as before.
    if config.get_profile() == "Debug" {
        config.profile("Release");
    }

    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("JPEGXL_ENABLE_DOXYGEN", "OFF")
        .define("JPEGXL_ENABLE_MANPAGES", "OFF")
        .define("JPEGXL_ENABLE_BENCHMARK", "OFF")
        .define("JPEGXL_ENABLE_JNI", "OFF")
        .define("JPEGXL_ENABLE_EXAMPLES", "OFF")
        .define("JPEGXL_ENABLE_TOOLS", "OFF")
        .define("BUILD_TESTING", "OFF")
        // .define("ENABLE_STATIC", "ON")
        .define("JPEGXL_ENABLE_JPEG", "ON")
        .define("JPEGXL_ENABLE_JPEGLI", "ON")
        .define("JPEGXL_INSTALL_JPEGLI_LIBJPEG", "ON")
        .define("JPEGXL_ENABLE_SJPEG", "OFF")
        .define("JPEGXL_ENABLE_OPENEXR", "OFF")
        .define("JPEGXL_ENABLE_FUZZERS", "OFF")
        .define("JPEGXL_ENABLE_DEVTOOLS", "OFF")
        .define("JPEGXL_ENABLE_TRANSCODE_JPEG", "OFF")
        .define("JPEGLI_LIBJPEG_LIBRARY_SOVERSION", "8")
        .define("JPEGLI_LIBJPEG_LIBRARY_VERSION", "8.2.2")
        .build_target("jpegli-static");

    let libjxl_path = config.build().join("build");
    let libjxl_lib_path = libjxl_path.join("lib");
    let jpegli_include_path = libjxl_lib_path.join("include").join("jpegli");

    // Add jpegli search paths.
    println!(
        "cargo::rustc-link-search=native={}",
        libjxl_lib_path.display()
    );
    // The Visual Studio Generator places files here.
    println!(
        "cargo::rustc-link-search=native={}",
        libjxl_lib_path.join(config.get_profile()).display()
    );

    // Add highway search paths.
    let highway_include_path = libjxl_path.join("third_party").join("highway");
    println!(
        "cargo::rustc-link-search=native={}",
        highway_include_path.display()
    );
    // The Visual Studio Generator places files here.
    println!(
        "cargo::rustc-link-search=native={}",
        highway_include_path.join(config.get_profile()).display()
    );

    let mut build = cc::Build::new();
    build
        .include("libjxl")
        .include("libjxl/third_party/libjpeg-turbo")
        .include(&jpegli_include_path)
        .file("wrapper/wrapper.c");
    build.compile("nd-jpegli");

    println!("cargo::rustc-link-lib=jpegli-static");
    println!("cargo::rustc-link-lib=hwy");

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
    println!(
        "cargo::metadata=include_jpegli={}",
        jpegli_include_path.display()
    );
    println!(
        "cargo::metadata=include_nd_jpegli_wrapper={}",
        current_dir.join("wrapper").display()
    );
}
