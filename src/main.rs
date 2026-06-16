mod config;

use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = std::fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml_bw::from_str(&yaml)?;
    println!("{config:#?}");
    Ok(())
}
