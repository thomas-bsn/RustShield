extern crate simple_logger;
extern crate log;
extern crate serde;
extern crate lazy_static;
extern crate serde_json;
extern crate notify;
extern crate regex;

mod libs{pub mod func; pub mod config; pub mod structures;}
mod watchers{pub mod watcher_mac; pub mod watcher_linux;}
mod network;
mod normalizer;
mod parsing;

use libs::func::get_os_type;
use log::info;
use watchers::watcher_mac::init_watcher_mac;
use watchers::watcher_linux::init_watcher_linux;

use libs::config::CONFIG;

// Temporaire
use std::path::PathBuf;
use crate::network::send_normalized_log_file;

fn main() 
{
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).with_module_level("foo", log::LevelFilter::Warn).without_timestamps().init().unwrap();
    info!("Démarrage de l'agent...");

    if !CONFIG.watch_paths.is_empty() {
        info!("Configuration loaded!");
    } else {
        info!("Configuration not loaded.");
        return;
    }

    let os_type = get_os_type();
    info!("OS type: {}", os_type);

    let test = false; 
    /*
    A changer pour True si on veut envoyer le fichier de template au serveur, 
    cela permet de tester le fonctionnement du serveur en envoyant directement une log deja parser et normaliser.
    */
    
    if test == true {
        info!("Envoi du fichier de template au serveur...");
        let mut normalized_log_path = PathBuf::from(CONFIG.normalized_log_path.clone());
        normalized_log_path.push("log-mysql_error.json");

        let _ = send_normalized_log_file(&normalized_log_path);
        info!("Fichier de template envoyé !");
    } else {
        match os_type {
            "Linux" => 
            {
                init_watcher_linux();
            },
    
            "Mac" => 
            {
                if let Err(e) = init_watcher_mac()
                {
                    eprintln!("Error watching logs: {}", e);
                }
            },
    
            "Windows" => 
            {
                println!("Prochainement :)"); 
            },
    
            _ => 
            {
                eprintln!("Système d'exploitation non reconnu. Veuillez choisir entre 'linux', 'mac' ou 'windows'.");
            },
        }
    }
    
}

