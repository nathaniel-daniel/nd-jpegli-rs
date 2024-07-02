fn main() {
    println!("cargo::rerun-if-changed=c/nd_jpegli_rs.c");

    let include = std::env::var("DEP_JPEGLI_STATIC_INCLUDE").expect("missing include dir");
    let include_jpegli =
        std::env::var("DEP_JPEGLI_STATIC_INCLUDE_JPEGLI").expect("missing jpegli include dir");
    let include_nd_jpegli = std::env::var("DEP_JPEGLI_STATIC_INCLUDE_ND_JPEGLI")
        .expect("missing nd-jpegli include dir");

    let mut build = cc::Build::new();
    build
        .include(include)
        .include(include_jpegli)
        .include(include_nd_jpegli)
        .file("c/nd_jpegli_rs.c");
    build.compile("nd-jpegli");
}
