mod cmake_configure_parser;
mod gni_parser;

use self::cmake_configure_parser::parse_cmake_configure;
use self::cmake_configure_parser::Node as CmakeDefineNode;
use self::gni_parser::parse_gni;
use self::gni_parser::Expr as GniExpr;
use anyhow::bail;
use anyhow::Context;
use camino::Utf8PathBuf;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

#[derive(argh::FromArgs)]
#[argh(
    subcommand,
    name = "generate-custom-jpegli",
    description = "generate the custom jpegli src files"
)]
pub struct Options {}

pub fn exec(metadata: cargo_metadata::Metadata, _options: Options) -> anyhow::Result<()> {
    let nd_jpegli_sys_dir = metadata.workspace_root.join("nd-jpegli-sys");

    // Input dirs
    let libjxl_dir = nd_jpegli_sys_dir.join("libjxl");
    let libjxl_lib_dir = libjxl_dir.join("lib");
    let highway_dir = {
        let mut path = libjxl_dir.clone();
        path.extend(["third_party", "highway"]);
        path
    };
    let libjpeg_turbo_dir = {
        let mut path = libjxl_dir.clone();
        path.extend(["third_party", "libjpeg-turbo"]);
        path
    };

    // Output Dirs
    let out_dir = nd_jpegli_sys_dir.join("custom-jpegli");
    // Include Dirs
    let out_include_dir = out_dir.join("include");
    // Src Dirs
    let out_src_jpegli_dir = out_dir.join("jpegli");

    match std::fs::remove_dir_all(&out_dir) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).with_context(|| format!("failed to delete \"{out_dir}\""));
        }
    }
    std::fs::create_dir(&out_dir).with_context(|| format!("failed to create \"{out_dir}\""))?;
    std::fs::create_dir(&out_include_dir)
        .with_context(|| format!("failed to create \"{out_include_dir}\""))?;
    std::fs::create_dir(&out_src_jpegli_dir)
        .with_context(|| format!("failed to create \"{out_src_jpegli_dir}\""))?;

    // Load lib.gni
    let lib_gni_path = libjxl_lib_dir.join("lib.gni");
    let lib_gni_str = std::fs::read_to_string(&lib_gni_path)
        .with_context(|| format!("failed to read \"{lib_gni_path}\""))?;
    let lib_gni =
        parse_gni(&lib_gni_str).with_context(|| format!("failed to parse \"{lib_gni_path}\""))?;

    // Load hwy.gni
    let hwy_gni_path = highway_dir.join("hwy.gni");
    let hwy_gni_str = std::fs::read_to_string(&hwy_gni_path)
        .with_context(|| format!("failed to read \"{hwy_gni_path}\""))?;
    let hwy_gni =
        parse_gni(&hwy_gni_str).with_context(|| format!("failed to parse \"{hwy_gni_path}\""))?;

    // Get lib.gni info
    let libjxl_jpegli_sources = extract_gni_string_array(&lib_gni, "libjxl_jpegli_sources")?;
    let libjxl_base_sources = extract_gni_string_array(&lib_gni, "libjxl_base_sources")?;
    let libjxl_public_headers = extract_gni_string_array(&lib_gni, "libjxl_public_headers")?;

    // Get hwy.gni info
    let hwy_public = hwy_gni
        .get("hwy_public")
        .context("missing hwy_public key")?
        .as_list()
        .context("hwy_public is not a list")?;
    let hwy_sources = hwy_gni
        .get("hwy_sources")
        .context("missing hwy_sources key")?
        .as_list()
        .context("hwy_sources is not a list")?;

    let mut highway_src_files = Vec::new();
    for file in hwy_public.iter().chain(hwy_sources.iter()) {
        let file = file
            .as_string()
            .context("entry should be a string")?
            .replace("$_hwy", "hwy");
        let extension = file.rsplit_once('.').context("missing extension")?.1;

        let input_path = {
            let mut path = highway_dir.clone();
            path.extend(file.split('/'));
            path
        };
        let output_path = match extension {
            "h" => {
                let mut path = out_include_dir.clone();
                path.extend(file.split('/'));
                path
            }
            "cc" => {
                let mut path = out_dir.clone();
                path.extend(file.split('/'));

                highway_src_files.push(
                    path.strip_prefix(&nd_jpegli_sys_dir)
                        .context("failed to strip path")?
                        .to_path_buf(),
                );

                path
            }
            _ => bail!("Unknown extension \"{extension}\""),
        };

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create \"{parent}\""))?;
        }

        std::fs::copy(input_path, output_path)?;
    }

    // Looks like highway forgot to include a file?
    {
        let input_path = {
            let mut path = highway_dir.clone();
            path.extend(["hwy", "ops", "tuple-inl.h"]);
            path
        };
        let output_path = {
            let mut path = out_dir.clone();
            path.extend(["hwy", "ops", "tuple-inl.h"]);
            path
        };

        highway_src_files.push(
            output_path
                .strip_prefix(&nd_jpegli_sys_dir)
                .context("failed to strip path")?
                .to_path_buf(),
        );

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create \"{parent}\""))?;
        }

        std::fs::copy(&input_path, &output_path)?;
    }

    let mut jpegli_src_files = Vec::new();
    for file in libjxl_jpegli_sources
        .iter()
        .chain(libjxl_base_sources.iter())
    {
        let extension = file.rsplit_once('.').context("missing extension")?.1;

        let input_path = {
            let mut path = libjxl_lib_dir.clone();
            path.extend(file.split('/'));
            path
        };
        let output_path = match extension {
            "h" => {
                let mut path = out_include_dir.clone();
                path.extend(file.split('/'));
                path
            }
            "cc" => {
                let mut path = out_dir.clone();
                path.extend(file.split('/'));

                jpegli_src_files.push(
                    path.strip_prefix(&nd_jpegli_sys_dir)
                        .context("failed to strip path")?
                        .to_path_buf(),
                );

                path
            }
            _ => bail!("Unknown extension \"{extension}\""),
        };

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create \"{parent}\""))?;
        }

        copy_rewrite(input_path, output_path)?;
    }

    for file in libjxl_public_headers.iter() {
        let input_path = {
            let mut path = libjxl_lib_dir.clone();
            path.extend(file.split('/'));
            path
        };
        let output_path = {
            let mut path = out_dir.clone();
            path.extend(file.split('/'));
            path
        };

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create \"{parent}\""))?;
        }

        copy_rewrite(input_path, output_path)?;
    }

    // jpegli will copy these files verbatim while building.
    let libjpeg_turbo_files = ["jpeglib.h", "jmorecfg.h"];
    for file in libjpeg_turbo_files {
        let input_path = libjpeg_turbo_dir.join(file);
        let output_path = {
            let mut path = out_include_dir.clone();
            path.extend(["jpegli", file]);
            path
        };

        std::fs::copy(input_path, output_path)?;
    }

    // jpegli configures this file with specific parameters.
    {
        let input_path = {
            let mut path = libjpeg_turbo_dir.clone();
            path.extend(["jconfig.h.in"]);
            path
        };
        let output_path = {
            let mut path = out_include_dir.clone();
            path.extend(["jpegli", "jconfig.h"]);
            path
        };

        let input_data = std::fs::read_to_string(&input_path)
            .with_context(|| format!("failed to read \"{input_path}\""))?;

        let nodes = parse_cmake_configure(&input_data)
            .with_context(|| format!("failed to parse \"{input_path}\""))?;

        let mut output_data = String::with_capacity(input_data.len());
        for node in nodes {
            match node {
                CmakeDefineNode::Text(text) => {
                    output_data.push_str(text.as_str());
                }
                CmakeDefineNode::Variable(name) => {
                    let value = match name.as_str() {
                        "JPEG_LIB_VERSION" => {
                            // This can be configured by the user to an extent.
                            // TODO: Investigate this parameter.
                            "80"
                        }
                        "VERSION" => {
                            // jpegli does not set this.
                            ""
                        }
                        "LIBJPEG_TURBO_VERSION_NUMBER" => {
                            // jpegli does not set this.
                            ""
                        }
                        "BITS_IN_JSAMPLE" => {
                            // jpegli forces this to be 8.
                            "8"
                        }
                        _ => {
                            bail!("Unknown variable name \"{name}\"");
                        }
                    };

                    output_data.push_str(value);
                }
                CmakeDefineNode::CmakeDefine { key, value } => {
                    // To my knowledge, jpegli sets these vars and unsets the rest,
                    // providing not control to the user.
                    let enabled = match key.as_str() {
                        "C_ARITH_CODING_SUPPORTED" => false,
                        "D_ARITH_CODING_SUPPORTED" => false,
                        "MEM_SRCDST_SUPPORTED" => true,
                        "WITH_SIMD" => false,
                        "RIGHT_SHIFT_IS_UNSIGNED" => false,
                        _ => {
                            bail!("Unknown cmake variable name \"{key}\"");
                        }
                    };

                    if enabled {
                        write!(&mut output_data, "#define {key} {value}")?;
                    } else {
                        write!(&mut output_data, "/* #undef {key} */")?;
                    }
                }
            }
        }

        std::fs::write(&output_path, output_data.as_bytes())
            .with_context(|| format!("failed to write \"{output_path}\""))?;
    }

    let mut custom_jpegli_file_str = String::new();
    custom_jpegli_file_str.push_str("// Automatically generated by xtask. DO NOT EDIT.\n\n");
    custom_jpegli_file_str.push_str("pub const BASE_DIR: &str = \"custom-jpegli\";\n");
    // custom_jpegli_file_str.push_str("pub const INCLUDE_DIR: &str = \"custom-jpegli/include\"");
    custom_jpegli_file_str.push_str("pub const JPEGLI_SRCS: &[&str] = &[\n");
    for file in jpegli_src_files {
        let file_components: Vec<_> = file
            .components()
            .map(|component| component.as_str())
            .collect();
        let file = file_components.join("/");

        writeln!(&mut custom_jpegli_file_str, "    \"{file}\",")?;
    }
    custom_jpegli_file_str.push_str("];");

    let custom_jpegli_file_path = {
        let mut path = nd_jpegli_sys_dir.clone();
        path.extend(["src", "build", "custom_jpegli.rs"]);
        path
    };
    std::fs::write(custom_jpegli_file_path, custom_jpegli_file_str.as_bytes())?;

    Ok(())
}

fn extract_gni_string_array(
    gni: &HashMap<String, GniExpr>,
    key: &str,
) -> anyhow::Result<Vec<String>> {
    let value = gni
        .get(key)
        .with_context(|| format!("missing key \"{key}\""))?
        .as_list()
        .with_context(|| format!("\"{key}\" is not a list"))?;

    let mut ret = Vec::new();
    for value in value.iter() {
        let value = value.as_string().context("value is not a string")?;
        ret.push(value.to_string());
    }

    Ok(ret)
}

fn copy_rewrite(input_path: Utf8PathBuf, output_path: Utf8PathBuf) -> anyhow::Result<()> {
    let input_file =
        File::open(&input_path).with_context(|| format!("failed to open \"{input_path}\""))?;
    let output_file = File::create_new(&output_path)
        .with_context(|| format!("failed to open \"{output_path}\""))?;

    let input_file = BufReader::new(input_file);
    let mut output_file = BufWriter::new(output_file);

    for line in input_file.lines() {
        let mut line = line?;

        // #include "{file}" || #include "{file}"
        if let Some(file) = line.strip_prefix("#include").and_then(|line| {
            let line = line.trim_start_matches(" ");
            let line = line.strip_prefix('"')?;
            let end_index = line.as_bytes().iter().position(|b| *b == b'"')?;

            Some(&line[..end_index])
        }) {
            if file.starts_with("lib/jpegli") || file.starts_with("lib/jxl") {
                let new_file = file.strip_prefix("lib/").context("missing prefix")?;
                line = line.replace(file, new_file);
            }
        }

        // #define HWY_TARGET_INCLUDE "{file}"
        if let Some(file) = line
            .strip_prefix("#define HWY_TARGET_INCLUDE")
            .and_then(|line| {
                let line = line.trim_start_matches(" ");
                let line = line.strip_prefix('"')?;
                let end_index = line.as_bytes().iter().position(|b| *b == b'"')?;

                Some(&line[..end_index])
            })
        {
            if file.starts_with("lib/jpegli") {
                let new_file = file.strip_prefix("lib/").context("missing prefix")?;
                line = line.replace(file, new_file);
            }
        }

        line.push('\n');
        output_file.write_all(line.as_bytes())?;
    }

    output_file.flush()?;
    let output_file = output_file.into_inner()?;
    output_file.sync_all()?;
    drop(output_file);

    Ok(())
}
