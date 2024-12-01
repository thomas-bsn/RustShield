use crate::libs::structures::NormalizedLog;
use std::io;
use glob::glob;
use log::info;

use crate::rule_matcher::{load_rule, check_log_against_rule};
use crate::libs::config::CONFIG;

pub fn load_and_check_rules(normalized_logs: &[NormalizedLog]) -> io::Result<()> {
    let rule_files: Vec<_> = glob(&format!("{}*.json", CONFIG.rules_path))
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect();

    let mut file_has_malicious_behavior = false;

    for log in normalized_logs {
        let mut log_matches_rule = false;

        for path in &rule_files {
            let rule = load_rule(path);
            if check_log_against_rule(log, &rule) {
                file_has_malicious_behavior = true;
                log_matches_rule = true;
                break; // Arrête de vérifier d'autres règles si une correspondance est trouvée pour ce log
            }
        }

        // Vous pouvez retirer cette ligne si vous ne voulez pas de feedback log par log
        if log_matches_rule {
            info!("Un log correspond à une règle: Log ID {}", log.action.id);
        }
    }

    if file_has_malicious_behavior {
        info!("Le fichier présente un comportement malveillant.");
    } else {
        info!("Aucun comportement malveillant détecté dans le fichier.");
    }

    Ok(())
}
