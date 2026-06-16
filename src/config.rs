use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Default, JsonSchema, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Default, JsonSchema, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub log_level: LogLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_schema_file() {
        let schema = schemars::schema_for!(Config);
        let json = serde_json::to_string_pretty(&schema).unwrap();
        std::fs::write("schema.json", json + "\n").unwrap();
    }
}
