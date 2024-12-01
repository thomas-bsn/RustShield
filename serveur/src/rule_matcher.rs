use std::fs;
use std::path::Path;
use log::info;

use crate::libs::config::CONFIG;
use crate::libs::structures::{NormalizedLog, Rule};
use crate::analyzer::{convert_access_log_to_normalized, convert_apt_history_log_to_normalized, convert_audit_log_to_normalized, convert_auth_log_to_normalized, convert_daemon_log_to_normalized, convert_dpkg_log_to_normalized, convert_fail2ban_log_to_normalized, convert_kern_log_to_normalized, convert_mysqlerror_log_to_normalized, convert_nginxerror_log_to_normalized};
use crate::alert_manager::load_and_check_rules;


// Fonction pour verifier si un log correspond à une regle
pub fn check_log_against_rule(log: &NormalizedLog, rule: &Rule) -> bool {
    let mut match_selections = Vec::new();

    for (key, values) in &rule.detection.selection {
        let key_parts: Vec<&str> = key.split('|').collect();
        let field = key_parts[0];
        let operator = if key_parts.len() > 1 {
            key_parts[1..].join("|")
        } else {
            "".to_string()
        };


        match field {
            "host.name" => match_selections.push(check_field(&log.host.name, &operator, values)),
            "host.environment" => match_selections.push(check_field(&log.host.environment, &operator, values)),
            "host.location" => match_selections.push(check_field(&log.host.location, &operator, values)),
            "source.ip" => match_selections.push(check_field(&log.source.ip, &operator, values)),
            "source.port" => match_selections.push(check_field(&log.source.port.to_string(), &operator, values)),
            "source.domain" => match_selections.push(check_field(&log.source.domain, &operator, values)),
            "destination.ip" => match_selections.push(check_field(&log.destination.ip, &operator, values)),
            "destination.port" => match_selections.push(check_field(&log.destination.port.to_string(), &operator, values)),
            "destination.domain" => match_selections.push(check_field(&log.destination.domain, &operator, values)),
            "process.name" => match_selections.push(check_field(&log.process.name, &operator, values)),
            "process.command_line" => match_selections.push(check_field(&log.process.command_line, &operator, values)),
            "process.pid" => match_selections.push(check_field(&log.process.pid.to_string(), &operator, values)),
            "network.protocol" => match_selections.push(check_field(&log.network.protocol, &operator, values)),
            "network.direction" => match_selections.push(check_field(&log.network.direction, &operator, values)),
            "network.bytes" => match_selections.push(check_field(&log.network.bytes.to_string(), &operator, values)),
            "event.category" => match_selections.push(check_field(&log.event.category, &operator, values)),
            "event.type" => match_selections.push(check_field(&log.event.type_, &operator, values)),
            "event.action" => match_selections.push(check_field(&log.event.action, &operator, values)),
            "user.name" => match_selections.push(check_field(&log.user.name, &operator, values)),
            "file.path" => match_selections.push(check_field(&log.file.path, &operator, values)),
            "file.name" => match_selections.push(check_field(&log.file.name, &operator, values)),
            "file.size" => match_selections.push(check_field(&log.file.size.to_string(), &operator, values)),
            "file.hash.md5" => match_selections.push(check_field(log.file.hash.get("md5").unwrap_or(&String::new()), &operator, values)),
            "action.id" => match_selections.push(check_field(&log.action.id, &operator, values)),
            _ => {}
        }
    }

    assert!(check_field(
        "test string with multiple values", 
        "contains|all", 
        &vec!["test".to_string(), "multiple".to_string(), "values".to_string()]
    ));
    assert!(!check_field(
        "test string with some values", 
        "contains|all", 
        &vec!["test".to_string(), "notpresent".to_string(), "values".to_string()]
    ));
    // info!("match_selections: {:?}", match_selections);

    // @todo revoir la fonction pour que eval_condition fonctionne avec plusieurs conditions
    //eval_condition(&rule.detection.condition, &match_selections)

    match_selections.iter().all(|&m| m)
}

fn check_field(field_value: &str, operator: &str, values: &[String]) -> bool {
    match operator {
        "contains|all" => values.iter().all(|v| field_value.contains(v)),
        "contains" => values.iter().any(|v| field_value.contains(v)),
        "" => values.contains(&field_value.to_string()),
        _ => false,
    }
}



// Fonction pour charger et désérialiser la règle à partir d'un fichier YAML
pub fn load_rule<P: AsRef<Path>>(path: P) -> Rule {
    let rule_yaml = fs::read_to_string(path).expect("Failed to read rule file");
    serde_yaml::from_str(&rule_yaml).expect("Failed to parse rule")
}

fn parse_json_logs() -> Result<Vec<NormalizedLog>, Box<dyn std::error::Error>> {
    let mut normalized_logs = Vec::new();
    for entry in std::fs::read_dir(&CONFIG.normalized_log_path)? {
        let path = entry?.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let contents = std::fs::read_to_string(&path)?;
            let json = serde_json::from_str::<serde_json::Value>(&contents)?;

            let log_type = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
            if let Some(log_array) = json.as_array() {
                for log_entry in log_array {
                    match log_type {
                        "log-access" => normalized_logs.push(convert_access_log_to_normalized(log_entry)),
                        "log-apt_history" => normalized_logs.push(convert_apt_history_log_to_normalized(log_entry)),
                        "log-audit" => normalized_logs.push(convert_audit_log_to_normalized(log_entry)),
                        "log-auth" => normalized_logs.push(convert_auth_log_to_normalized(log_entry)),
                        "log-daemon" => normalized_logs.push(convert_daemon_log_to_normalized(log_entry)),
                        "log-dpkg" => normalized_logs.push(convert_dpkg_log_to_normalized(log_entry)),
                        "log-fail2ban" => normalized_logs.push(convert_fail2ban_log_to_normalized(log_entry)),
                        "log-kern" => normalized_logs.push(convert_kern_log_to_normalized(log_entry)),
                        "log-mysqlerror" => normalized_logs.push(convert_mysqlerror_log_to_normalized(log_entry)),
                        "log-nginxerror" => normalized_logs.push(convert_nginxerror_log_to_normalized(log_entry)),
                        _ => {}
                    }
                }
            }
        }
    }
    // info!("normalized_logs: {:?}", normalized_logs);
    Ok(normalized_logs)
}



pub fn parse_and_check_rules() {
    match parse_json_logs() {
        Ok(normalized_logs) => {
            if let Err(e) = load_and_check_rules(&normalized_logs) {
                info!("Error checking rules: {}", e);
            }
        }
        Err(e) => info!("Failed to parse JSON logs: {}", e),
    }
}
