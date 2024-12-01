use lazy_static::lazy_static;
use crate::libs::config::CONFIG;


lazy_static! {
    pub static ref LOG_FILES_AND_CONFIGS: Vec<(String, String)> = { // Linux
        vec![
            (format!("{}access.log", CONFIG.watch_paths), format!("{}access_config.json", CONFIG.logs_config)),
            (format!("{}auth.log", CONFIG.watch_paths), format!("{}auth_config.json", CONFIG.logs_config)),
            (format!("{}daemon.log", CONFIG.watch_paths), format!("{}daemon_config.json", CONFIG.logs_config)),
            (format!("{}kern.log", CONFIG.watch_paths), format!("{}kern_config.json", CONFIG.logs_config)),
            (format!("{}nginx_error.log", CONFIG.watch_paths), format!("{}nginx_error_config.json", CONFIG.logs_config)),
            (format!("{}mysql_error.log", CONFIG.watch_paths), format!("{}mysql_error_config.json", CONFIG.logs_config)),
            (format!("{}audit.log", CONFIG.watch_paths), format!("{}audit_config.json", CONFIG.logs_config)),
            (format!("{}ufw.log", CONFIG.watch_paths), format!("{}ufw_config.json", CONFIG.logs_config)),
            (format!("{}fail2ban.log", CONFIG.watch_paths), format!("{}fail2ban_config.json", CONFIG.logs_config)),
            (format!("{}apt_history.log", CONFIG.watch_paths), format!("{}apt_history_config.json", CONFIG.logs_config)),
            (format!("{}dpkg.log", CONFIG.watch_paths), format!("{}dpkg_config.json", CONFIG.logs_config)),
            ("/var/log/apache2/access.log".into(), format!("{}access_config.json", CONFIG.logs_config)),
            ("/var/log/nginx/access.log".into(), format!("{}access_config.json", CONFIG.logs_config)),
            ("/var/log/auth.log".into(), format!("{}auth_config.json", CONFIG.logs_config)),
            ("/var/log/daemon.log".into(), format!("{}daemon_config.json", CONFIG.logs_config)),
            ("/var/log/kern.log".into(), format!("{}kern_config.json", CONFIG.logs_config)),
            ("/var/log/nginx/error.log".into(), format!("{}nginx_error_config.json", CONFIG.logs_config)),
            ("/var/log/mysql/error.log".into(), format!("{}mysql_error_config.json", CONFIG.logs_config)),
            ("/var/log/audit/audit.log".into(), format!("{}audit_config.json", CONFIG.logs_config)),
            ("/var/log/ufw.log".into(), format!("{}ufw_config.json", CONFIG.logs_config)),
            ("/var/log/fail2ban.log".into(), format!("{}fail2ban_config.json", CONFIG.logs_config)),
            ("/var/log/apt/history.log".into(), format!("{}apt_history_config.json", CONFIG.logs_config)),
            ("/var/log/dpkg.log".into(), format!("{}dpkg_config.json", CONFIG.logs_config)),
        ]
    };
}


pub fn get_os_type() -> &'static str 
{
    if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "android") {
        "Android"
    } else if cfg!(target_os = "ios") {
        "iOS"
    } else {
        "unknown"
    }
}
