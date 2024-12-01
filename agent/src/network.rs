use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::fs;
use std::io::BufReader;
use std::fs::File;

use crate::libs::config::CONFIG;



pub fn send_normalized_log_file(path: &PathBuf) -> io::Result<()> {
    let server_address = CONFIG.server_ip.clone();

    let mut stream = TcpStream::connect(server_address)?;
    // Si le serveur ne repond pas, on renvoie une erreur
    if stream.peer_addr()?.ip().is_unspecified() {
        return Err(io::Error::new(io::ErrorKind::ConnectionRefused, "Le serveur ne répond pas"));
    }

    // Extraire le nom de fichier du chemin
    let file_name = path.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Chemin invalide"))?;
    let file_name = file_name.to_string_lossy();

    // Lire le contenu du fichier
    let _content = fs::read_to_string(path)?;

    // Envoyer le nom du fichier
    stream.write_all(file_name.as_bytes())?;
    stream.write_all(b"\n")?; // Delimiteur de message
    stream.flush()?;

    // Envoyer le contenu du fichier
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024]; // Taille du buffer

    // Lire et envoyer le contenu du fichier par morceaux
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // Fin du fichier
        }
        stream.write_all(&buffer[..bytes_read])?;
    }
    stream.write_all(b"\n")?; // Delimiteur de fin de fichier
    stream.flush()?;

    Ok(())

}
