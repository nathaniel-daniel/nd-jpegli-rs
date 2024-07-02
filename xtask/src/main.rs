mod commands;

#[derive(argh::FromArgs)]
#[argh(description = "xtask")]
struct Options {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    GenerateCustomJpegli(self::commands::generate_custom_jpegli::Options),
    GenerateBindings(self::commands::generate_bindings::Options),
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;

    match options.subcommand {
        Subcommand::GenerateCustomJpegli(options) => {
            self::commands::generate_custom_jpegli::exec(metadata, options)
        }
        Subcommand::GenerateBindings(options) => {
            self::commands::generate_bindings::exec(metadata, options)
        }
    }
}
