fn main() {
    let include_libjxl =
        std::env::var("DEP_JPEGLI_STATIC_INCLUDE_LIBJXL").expect("missing libjxl include dir");
    let include_libjpeg_turbo = std::env::var("DEP_JPEGLI_STATIC_INCLUDE_LIBJPEG_TURBO")
        .expect("missing libjpeg turbo include dir");
    let include_jpegli =
        std::env::var("DEP_JPEGLI_STATIC_INCLUDE_JPEGLI").expect("missing jpegli include dir");
    let include_nd_jpegli_wrapper = std::env::var("DEP_JPEGLI_STATIC_INCLUDE_ND_JPEGLI_WRAPPER")
        .expect("missing nd jpegli wrapper include dir");

    let mut build = cc::Build::new();
    build
        .include(include_libjxl)
        .include(include_libjpeg_turbo)
        .include(include_jpegli)
        .include(include_nd_jpegli_wrapper)
        .file("c/nd_jpegli_rs.c");
    build.compile("nd-jpegli");
}
