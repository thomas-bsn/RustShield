use std::fs::{self, File};
use std::path::PathBuf;
use log::info;

use crate::libs::config::CONFIG;
use crate::libs::func::LOG_FILES_AND_CONFIGS;
use crate::libs::structures::LogConfig;
use crate::normalizer::format_logs_to_json;

pub fn parse_log(path: PathBuf) -> Result<String, String> {

    info!("Parsing logs...");
    for (log_file, config_file) in LOG_FILES_AND_CONFIGS.iter() {
        let log_file_path = PathBuf::from(log_file);
        let log_file_str = log_file_path.to_str().unwrap_or_default();

        if let Some(ref p_str) = path.to_str() {
            if p_str.ends_with(log_file_str) {
                if let Ok(log_contents) = read_log_file(log_file) {
                    if let Ok(config) = read_config_file(config_file) {
                        let formatted_json = format_logs_to_json(&log_contents, &config);
                        let output_file = format!("{}/log-{}.json", CONFIG.normalized_log_path, extract_file_name(log_file));
                        
                        if fs::write(&output_file, formatted_json).is_err() {
                            return Err(format!("Failed to write JSON file: {}", output_file));
                        }
                        
                        return Ok(output_file); // Retourne le chemin du fichier de log normalisé
                    }
                }
                break;
            }
        }
    }

    Err("Log file not found or could not be parsed".to_string())
}

fn read_log_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn read_config_file(file_path: &str) -> Result<LogConfig, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let config: LogConfig = serde_json::from_reader(file)?;
    Ok(config)
}

fn extract_file_name(file_path: &str) -> String {
    file_path.split('/').last().unwrap_or_default().replace(".log", "")
    }