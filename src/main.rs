mod config;

use config::Config;

fn main() {
    let yaml = match std::fs::read_to_string("config.yaml") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("не удалось прочитать config.yaml: {err}");
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
                    "ошибка в config.yaml (путь: {}, строка {}, колонка {}): {inner}",
                    err.path(),
                    loc.line(),
                    loc.column(),
                );
            } else {
                eprintln!(
                    "ошибка в config.yaml (путь: {}): {inner}",
                    err.path(),
                );
            }
            std::process::exit(1);
        }
    };

    println!("{config:#?}");
}
