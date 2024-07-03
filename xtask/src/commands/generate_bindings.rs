use anyhow::bail;
use anyhow::Context;

#[derive(argh::FromArgs)]
#[argh(
    subcommand,
    name = "generate-bindings",
    description = "generate bindings for the sys crate"
)]
pub struct Options {}

pub fn exec(metadata: cargo_metadata::Metadata, _options: Options) -> anyhow::Result<()> {
    let nd_jpegli_sys_dir = metadata.workspace_root.join("nd-jpegli-sys");
    let custom_jpegli_dir = nd_jpegli_sys_dir.join("custom-jpegli");

    let custom_jpegli_include_dir = custom_jpegli_dir.join("include");

    if !custom_jpegli_dir.try_exists()? {
        bail!("generated custom jpegli src does not exist");
    }

    let bindings = bindgen::Builder::default()
        .header(nd_jpegli_sys_dir.join("bindgen-wrapper.h"))
        .allowlist_type("jpeg_decompress_struct")
        .allowlist_type("jpeg_compress_struct")
        .allowlist_var("JPEG_(SUSPENDED|HEADER_OK|HEADER_TABLES_ONLY)")
        .newtype_enum("J_COLOR_SPACE")
        .newtype_enum("J_DCT_METHOD")
        .clang_args([
            format!("-I{custom_jpegli_include_dir}"),
            format!("-I{}", custom_jpegli_include_dir.join("jpegli")),
        ])
        // Bindgen's layout tests are not cross platform.
        .layout_tests(false)
        .generate()
        .context("failed to generate bindings")?;

    let bindings_path = {
        let mut path = nd_jpegli_sys_dir.clone();
        path.extend(["src", "bindings.rs"]);
        path
    };
    bindings
        .write_to_file(bindings_path)
        .context("failed to write bindings")?;

    Ok(())
}
