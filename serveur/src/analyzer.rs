use std::collections::HashMap;

use crate::libs::structures::{NormalizedLog, Host, Endpoint, Process, Network, Event, User, File, Action};

pub fn convert_access_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  String::new(),
            environment:  String::new(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: json["ip"].as_str().unwrap_or_default().to_string(),
            port: 0,
            domain: json["referer"].as_str().unwrap_or_default().to_string(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: String::new(),
            command_line: String::new(),
            pid: 0,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: String::new(),
            type_: String::new(),
            action: String::new(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: String::new(),
        },
        timestamp: json["date"].as_str().unwrap_or_default().to_string(),
    }
}

// OK
pub fn convert_apt_history_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  String::new(),
            environment:  String::new(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: String::new(),
            command_line: json["details"].as_str().unwrap_or_default().to_string(),
            pid: 0,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: String::new(),
            type_: json["action"].as_str().unwrap_or_default().to_string(),
            action: String::new(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: String::new(),
        },
        timestamp: String::new(),
    }
}

// OK
pub fn convert_audit_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  String::new(),
            environment:  String::new(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: String::new(),
            command_line: json["message"].as_str().unwrap_or_default().to_string(),
            pid: 0,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: String::new(),
            type_: json["type"].as_str().unwrap_or_default().to_string(),
            action: String::new(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: String::new(),
        },
        timestamp: json["timestamp"].as_str().unwrap_or_default().to_string(),
    }
}

// OK
pub fn convert_auth_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  json["server_process"].as_str().unwrap_or_default().to_string(),
            environment:  String::new(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: String::new(),
            command_line: json["message_type"].as_str().unwrap_or_default().to_string(),
            pid: 0,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: String::new(),
            type_: json["message_content"].as_str().unwrap_or_default().to_string(),
            action: String::new(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: json["process_id"].as_str().unwrap_or_default().to_string(),
        },
        timestamp: json["timestamp"].as_str().unwrap_or_default().to_string(),
    }
}

// OK
pub fn convert_daemon_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  json["hostname"].as_str().unwrap_or_default().to_string(),
            environment:  String::new(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: json["service"].as_str().unwrap_or_default().to_string(),
            command_line: json["message"].as_str().unwrap_or_default().to_string(),
            pid: json["process_id"].as_u64().unwrap_or_default() as u32,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: String::new(),
            type_: String::new(),
            action: String::new(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: String::new(),
        },
        timestamp: json["timestamp"].as_str().unwrap_or_default().to_string(),
    }
}

// OK
pub fn convert_dpkg_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  String::new(),
            environment:  json["version"].as_str().unwrap_or_default().to_string(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: String::new(),
            command_line: String::new(),
            pid: 0,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: json["package"].as_str().unwrap_or_default().to_string(),
            type_: json["status"].as_str().unwrap_or_default().to_string(),
            action: json["action"].as_str().unwrap_or_default().to_string(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: String::new(),
        },
        timestamp: json["timestamp"].as_str().unwrap_or_default().to_string(),
    }
}

// OK
pub fn convert_fail2ban_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  String::new(),
            environment:  String::new(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: String::new(),
            command_line: json["message"].as_str().unwrap_or_default().to_string(),
            pid: 0,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: json["jail"].as_str().unwrap_or_default().to_string(),
            type_: json["action"].as_str().unwrap_or_default().to_string(),
            action: String::new(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: json["action"].as_str().unwrap_or_default().to_string(),
        },
        timestamp: json["timestamp"].as_str().unwrap_or_default().to_string(),
    }
}

// OK
pub fn convert_kern_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  json["hostname"].as_str().unwrap_or_default().to_string(),
            environment:  String::new(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: String::new(),
            command_line: json["message"].as_str().unwrap_or_default().to_string(),
            pid: 0,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: String::new(),
            type_: String::new(),
            action: String::new(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: json["uptime"].as_str().unwrap_or_default().to_string(),
        },
        timestamp: json["timestamp"].as_str().unwrap_or_default().to_string(),
    }
}

// OK
pub fn convert_mysqlerror_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  String::new(),
            environment:  json["server_component"].as_str().unwrap_or_default().to_string(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: String::new(),
        },
        process: Process {
            name: String::new(),
            command_line: String::new(),
            pid: json["thread_id"].as_u64().unwrap_or_default() as u32,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: json["log_level"].as_str().unwrap_or_default().to_string(),
            type_: json["message"].as_str().unwrap_or_default().to_string(),
            action: String::new(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: json["error_code"].as_str().unwrap_or_default().to_string(),
        },
        timestamp: json["timestamp"].as_str().unwrap_or_default().to_string(),
    }
}

// OK
pub fn convert_nginxerror_log_to_normalized(json: &serde_json::Value) -> NormalizedLog {
    NormalizedLog {
        host: Host { 
            name:  json["host"].as_str().unwrap_or_default().to_string(),
            environment:  String::new(),
            location:  String::new(),
        },
        source: Endpoint {
            ip: json["client_ip"].as_str().unwrap_or_default().to_string(),
            port: 0,
            domain: json["referrer"].as_str().unwrap_or_default().to_string(),
        },
        destination: Endpoint {
            ip: String::new(),
            port: 0,
            domain: json["server"].as_str().unwrap_or_default().to_string(),
        },
        process: Process {
            name: String::new(),
            command_line: String::new(),
            pid: json["pid"].as_u64().unwrap_or_default() as u32,
        },
        network: Network {
            protocol: String::new(),
            direction: String::new(),
            bytes: 0,
        },
        event: Event {
            category: json["log_level"].as_str().unwrap_or_default().to_string(),
            type_: json["message"].as_str().unwrap_or_default().to_string(),
            action: json["request"].as_str().unwrap_or_default().to_string(),
        },
        user: User {
            name: String::new(),
        },
        file: File {
            path: String::new(),
            name: String::new(),
            size: 0,
            hash: HashMap::new(),
        },
        action: Action {
            id: String::new(),
        },
        timestamp: json["timestamp"].as_str().unwrap_or_default().to_string(),
    }
}