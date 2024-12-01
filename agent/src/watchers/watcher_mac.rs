use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub fn init_watcher_mac() -> std::io::Result<()> 
{
    let process = Command::new("log")
        .args(&["stream", "--level", "info"])
        .stdout(Stdio::piped()) // Dirigez la sortie standard vers un tuyau
        .spawn()?; // Démarrez le processus

    let reader = BufReader::new(process.stdout.unwrap());

    for line in reader.lines() {
        let line = line?;
        println!("Log: {}", line);
        // @todo Ajouter une logique pour analyser / filtrer les lignes de log
    }

    Ok(())
}
