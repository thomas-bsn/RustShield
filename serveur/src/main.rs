extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate log;
extern crate evalexpr;
extern crate simple_logger;
extern crate glob;


mod libs{pub mod config; pub mod structures;}
mod receiver;
mod analyzer;
mod rule_matcher;
mod alert_manager;


use log::info;
use crate::libs::config::CONFIG;
use crate::receiver::create_listener;
use crate::rule_matcher::parse_and_check_rules;

fn main()  
{
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).with_module_level("foo", log::LevelFilter::Warn).without_timestamps().init().unwrap();
    info!("Démarrage du serveur");
    loop {
        let server_address = CONFIG.server_ip.clone();
        info!("Waiting for logs...");
        // On lance le serveur et recoit les logs
        let _ = create_listener(server_address).expect("Erreur lors de la création du listener");

        // Je lis le fichier de log normalisé et le match avec les règles
        parse_and_check_rules();
    }
}
