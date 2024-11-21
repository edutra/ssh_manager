use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SshConnection {
    name: String,
    host: String,
    port: u16,
    username: String,
}

fn config_path() -> PathBuf {
    let mut path = home_dir().expect("Could not determine home directory");
    path.push(".ssh_manager");
    path.push("config.json");
    path
}

fn load_connections(path: PathBuf) -> Vec<SshConnection> {
    if path.exists() {
        let data = fs::read_to_string(path).expect("Failed to read config file");
        serde_json::from_str(&data).expect("Failed to parse JSON")
    } else {
        Vec::new()
    }
}

fn save_connections(path: PathBuf, connections: &[SshConnection]) {
    // let path = config_path();
    fs::create_dir_all(path.parent().unwrap()).expect("Failed to create config directory");
    let data = serde_json::to_string_pretty(connections).expect("Failed to serialize connections");
    fs::write(path, data).expect("Failed to write config file");
}

#[derive(Parser)]
#[command(name = "SSH Manager")]
#[command(about = "A CLI tool to manage SSH connections", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        host: String,
        port: u16,
        username: String,
    },
    List,
    Delete { name: String },
    Open { name: String },
}

fn main() {
    let cli = Cli::parse();

    let mut connections = load_connections(config_path());

    match cli.command {
        Commands::Add {
            name,
            host,
            port,
            username,
        } => {
            connections.push(SshConnection {
                name,
                host,
                port,
                username,
            });
            save_connections(config_path(), &connections);
            println!("Connection added!");
        }
        Commands::List => {
            if connections.is_empty() {
                println!("No SSH connections found.");
            } else {
                for conn in &connections {
                    println!(
                        "Name: {}, Host: {}, Port: {}, Username: {}",
                        conn.name, conn.host, conn.port, conn.username
                    );
                }
            }
        }
        Commands::Delete { name } => {
            connections.retain(|conn| conn.name != name);
            save_connections(config_path(), &connections);
            println!("Connection deleted!");
        }
        Commands::Open { name } => {
            if let Some(conn) = connections.iter().find(|c| c.name == name) {
                let ssh_command = format!("ssh {}@{} -p {}", conn.username, conn.host, conn.port);
                println!("Opening SSH connection: {}", ssh_command);

                let mut process = Command::new("ssh")
                    .arg(format!("{}@{}", conn.username, conn.host))
                    .arg("-p")
                    .arg(conn.port.to_string())
                    .spawn()
                    .expect("Failed to start SSH process");

                process
                    .wait()
                    .expect("Failed to wait on SSH process");
            } else {
                println!("Connection '{}' not found.", name);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    #[test]
    fn test_config_path() {
        let path = config_path();
        assert_eq!(path, home_dir().unwrap().join(".ssh_manager/config.json"));
    }
    #[test]
    fn test_load_connections() {

        let empty_path = Path::new("test_files/empty.json").to_path_buf();
        let connections = load_connections(empty_path);
        assert_eq!(connections.len(), 0);
    }
    #[test]
    fn test_save_connections() {
        let connections = vec![SshConnection {
            name: "test".to_string(),
            host: "example.com".to_string(),
            port: 22,
            username: "user".to_string(),
        }];
        let path = PathBuf::from("test_files/save.json");
        save_connections(path.clone(), &connections);
        let loaded = load_connections(path);
        assert_eq!(connections, loaded);
    }
}
