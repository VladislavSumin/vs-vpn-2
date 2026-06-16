use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_level: LogLevel::Info,
        }
    }
}

#[allow(dead_code)]
pub fn generate_schema() -> schemars::Schema {
    schemars::schema_for!(Config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_schema_file() {
        let schema = generate_schema();
        let json = serde_json::to_string_pretty(&schema).unwrap();
        std::fs::write("schema.json", json + "\n").unwrap();
    }
}
