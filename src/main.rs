use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug)]
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

fn load_connections() -> Vec<SshConnection> {
    let path = config_path();
    if path.exists() {
        let data = fs::read_to_string(path).expect("Failed to read config file");
        serde_json::from_str(&data).expect("Failed to parse JSON")
    } else {
        Vec::new()
    }
}

fn save_connections(connections: &[SshConnection]) {
    let path = config_path();
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

    let mut connections = load_connections();

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
            save_connections(&connections);
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
            save_connections(&connections);
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
