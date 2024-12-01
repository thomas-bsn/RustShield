use std::sync::mpsc::channel;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::time::Duration;
use log::info;
use std::path::PathBuf;

use crate::libs::config::CONFIG;
use crate::network::send_normalized_log_file;
use crate::parsing::parse_log;

pub fn init_watcher_linux()
{

    let (tx, _rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(CONFIG.check_frequency_seconds.into())).unwrap();

    let chemin = PathBuf::from(CONFIG.watch_paths.clone());
    for path in chemin.read_dir().unwrap() // Si le chemin n'existe pas, on ignore l'erreur
    {
        if let Ok(path) = path 
        {
            if let Err(e) = watcher.watch(path.path(), RecursiveMode::Recursive) 
            {
                info!("Erreur lors de la surveillance du chemin {:?} : {:?}", path.path(), e);
            }
        }
    }
    info!("Watcher initialisé");
    loop 
    {
        info!("Waiting for event...");
        match _rx.recv() {
            Ok(event) => 
            {
                info!("Event detected, processing...");
                let path = match event 
                {
                    DebouncedEvent::Create(path) => path,
                    DebouncedEvent::Write(path) => path,
                    DebouncedEvent::Remove(path) => path,
                    DebouncedEvent::Rename(path, _) => path,
                    _ => continue,
                };
                handle_event_linux(path);
            }
            Err(e) => info!("Erreur de surveillance: {:?}", e),
        }
    }
    
}


fn handle_event_linux(path: PathBuf) {
    match parse_log(path) {
        Ok(log_file_path) => {
            let log_path = PathBuf::from(log_file_path);
            info!("Fichier de log normalisé généré");
            if let Err(e) = send_normalized_log_file(&log_path) {
                println!("Erreur lors de l'envoi du fichier de log normalisé : {:?}", e);
            }
            info!("Fichier de log normalisé envoyé");
        },
        Err(e) => println!("Erreur lors du parsing du log : {:?}", e),
    }
}

