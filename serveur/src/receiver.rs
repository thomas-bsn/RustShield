use std::io::prelude::*;
use std::io::Result;
use std::io;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;
use log::info;
use std::fs::File;
use std::path::PathBuf;
use crate::CONFIG;

fn handle_connection(stream: TcpStream) -> io::Result<String> {
    let mut reader = BufReader::new(&stream);

    // Lecture du premier message pour le nom de fichier
    let mut file_name = String::new();
    reader.read_line(&mut file_name).unwrap();
    let file_name = file_name.trim_end(); // Nettoyer le délimiteur et les espaces
    info!("Nom du fichier reçu : {}", file_name);

    // Construire le chemin complet dans le répertoire 'tests/'

    // SI le répertoire n'existe pas, le créer
    if !PathBuf::from(CONFIG.normalized_log_path.clone()).exists() {
        std::fs::create_dir(PathBuf::from(CONFIG.normalized_log_path.clone())).unwrap();
    }
    let mut path = PathBuf::from(CONFIG.normalized_log_path.clone());
    path.push(file_name);

    // Création et ouverture du fichier pour l'écriture
    let mut file = File::create(&path).unwrap();

    // Lecture et écriture du contenu du fichier par morceaux
    let mut buffer = [0; 1024]; // Taille du buffer
    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break; // Fin du fichier ou fin de la transmission
        }
        file.write_all(&buffer[..bytes_read]).unwrap();
    }

    info!("Fichier '{}' créé avec le contenu", file_name);
    Ok(file_name.to_string())
}




pub fn create_listener(addr: String) -> Result<String> {
    let listener = TcpListener::bind(addr)?;
    
    // Attendre et traiter la première connexion réussie
    if let Ok((stream, _)) = listener.accept() {
        match handle_connection(stream) {
            Ok(file_name) => {
                // Retourner le nom du fichier traité
                Ok(file_name)
            },
            Err(e) => {
                info!("Erreur lors de la manipulation de la connexion: {}", e);
                Err(e)
            }
        }
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Aucune connexion entrante"))
    }
}

