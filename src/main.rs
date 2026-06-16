mod config;

use clap::Parser;
use config::Config;
use std::path::PathBuf;

#[derive(clap::Parser)]
struct Cli {
    #[arg(default_value = "config.yaml")]
    config: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let yaml = match std::fs::read_to_string(&cli.config) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("failed to read {}: {err}", cli.config.display());
            std::process::exit(1);
        }
    };

    let deserializer = serde_yaml_bw::Deserializer::from_str(&yaml);
    let config: Config = match serde_path_to_error::deserialize(deserializer) {
        Ok(config) => config,
        Err(err) => {
            let inner = err.inner();
            if let Some(loc) = inner.location() {
                eprintln!(
                    "error in {}:{}:{}: {inner}",
                    cli.config.display(),
                    loc.line(),
                    loc.column(),
                );
            } else {
                eprintln!("error in {}: {inner}", cli.config.display());
            }
            std::process::exit(1);
        }
    };

    println!("{config:#?}");
}
