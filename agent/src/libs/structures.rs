use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Config 
{
    pub watch_paths: String,
    pub normalized_log_path: String,
    pub logs_config: String,
    pub server_ip: String,
    pub check_frequency_seconds: u32,
    pub ignore_patterns: Vec<String>,
}

// Structure pour représenter la configuration de chaque type de log
#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub regex: String,
    pub matches: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NormalizedLog {
    pub host: Host,
    pub source: Endpoint,
    pub destination: Endpoint,
    pub process: Process,
    pub network: Network,
    pub event: Event,
    pub user: User,
    pub file: File,
    pub action: Action,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    pub name: String,
    pub environment: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    pub ip: String,
    pub port: u16,
    pub domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    pub name: String,
    pub command_line: String,
    pub pid: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub protocol: String,
    pub direction: String,
    pub bytes: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub category: String,
    pub type_: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub hash: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub id: String,
    pub status: String,
    pub description: String,
    pub author: String,
    pub date: String,
    pub criticality: u8,
    pub detection: Detection,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Detection {
    pub selection: HashMap<String, Vec<String>>,
    pub condition: String,
}
