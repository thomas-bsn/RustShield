use serde_json;
use regex::Regex;
use serde_json::Value;

use crate::libs::structures::LogConfig;


pub fn format_logs_to_json(log_content: &str, config: &LogConfig) -> String
{
    let re = Regex::new(&config.regex).unwrap();
    let mut json_lines = Vec::new();
    for line in log_content.lines() {
        if let Some(caps) = re.captures(line) {
            let mut log_entry = serde_json::Map::new();
            for (key, value) in &config.matches {
                if let Some(matched) = caps.get(key.parse().unwrap()) {
                    log_entry.insert(value.clone(), Value::String(matched.as_str().to_string()));
                }
            }
            json_lines.push(Value::Object(log_entry));
        }
    }
    
    serde_json::to_string_pretty(&json_lines).unwrap_or_else(|_| "[]".to_string())
}
