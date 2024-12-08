use serde::{Deserialize, Serialize};
use std::fs;
use clap::{Parser, Subcommand};
use std::path::{PathBuf, Path};
use std::process::Command;
use dirs::home_dir;
use ansi_term::Colour::{Green, Blue};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SshConnection {
    name: String,
    host: String,
    port: u16,
    username: String,
    welcome_message: Option<String>
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
    #[command(name = "--add", alias = "-a")]
    Add {
        name: String,
        host: String,
        port: u16,
        username: String,
        welcome_message: Option<String>
    },
    #[command(name = "--list", alias = "-l")]
    List,
    #[command(name = "--delete", alias = "-d")]
    Delete { name: String },
    #[command(name = "--open", alias = "-o")]
    Open { name: String },
    #[command(name = "--edit", alias = "-e")]
    Edit {
        nameproperty: String,
        value: String
    },
    // Only runs bash script
    #[command(name = "--snippet", alias = "-s")]
    Snippet {
        name: String,
        path: Box<Path>
    }
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
            welcome_message
        } => {
            connections.push(SshConnection {
                name,
                host,
                port,
                username,
                welcome_message
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
                        Green.bold().paint(conn.name.clone()), Green.paint(conn.host.clone()), Green.paint(conn.port.to_string()), Green.paint(conn.username.clone())
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
                let ssh_command = format!("ssh -t {}@{} -p {}", conn.username, conn.host, conn.port);
                println!("Opening SSH connection: {}", ssh_command);
                let mut process = Command::new("ssh")
                    .arg("-t")
                    .arg(format!("{}@{}", conn.username, conn.host))
                    .arg("-p")
                    .arg(conn.port.to_string())
                    .arg(format!("echo {:?} ;", conn.welcome_message.clone().unwrap_or("".to_string())))
                    .arg("exec $SHELL || bash || zsh || sh || /bin/sh")
                    .spawn()
                    .expect("Failed to start SSH process");

                process
                    .wait()
                    .expect("Failed to wait on SSH process");
            } else {
                println!("Connection '{}' not found.", name);
            }
        }
        Commands::Edit { nameproperty, value } => {
            let vector = nameproperty.split('.').collect::<Vec<&str>>();
            let name = vector.clone()[0];
            let property = vector.clone()[1];
            let new_value = value.clone();
            if let Some(conn) = connections.iter_mut().find(|c| c.name == name) {
                match property {
                    "name" => conn.name = value,
                    "host" => conn.host = value,
                    "port" => conn.port = value.parse().expect("Port must be a number"),
                    "username" => conn.username = value,
                    "welcome_message" => conn.welcome_message = Some(value),
                    _ => println!("Invalid property: {}", property),
                }
                save_connections(config_path(), &connections);
                println!("Property {} of {} connection updated to {}.", property, Blue.paint(name),  Green.paint(new_value));
            } else {
                println!("Connection '{}' not found.", name);
            }
        }
        Commands::Snippet { name, path } => {
            if let Some(conn) = connections.iter().find(|c| c.name == name) {
                let data = fs::read_to_string(&path).expect("Failed to read snippet file");
                let ssh_command = format!(
                    "cat > ~/received_script.sh && chmod +x ~/received_script.sh && ./received_script.sh && rm ~/received_script.sh"
                );

                let mut process = Command::new("ssh")
                    .arg(format!("{}@{}", conn.username, conn.host))
                    .arg(ssh_command)
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .expect("Failed to start SSH process");

                // Write the snippet content to the process's standard input
                if let Some(ref mut stdin) = process.stdin {
                    use std::io::Write;
                    stdin
                        .write_all(data.as_bytes())
                        .expect("Failed to write snippet data to SSH process");
                }

                // Wait for the process to complete
                let status = process
                    .wait()
                    .expect("Failed to wait on SSH process");

                if status.success() {
                    println!("Snippet executed successfully on the remote host.");
                } else {
                    eprintln!("Error: Snippet execution failed with status: {}", status);
                }
            } else {
                eprintln!("Connection with name '{}' not found.", name);
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
            welcome_message: None,
        }];
        let path = PathBuf::from("test_files/save.json");
        save_connections(path.clone(), &connections);
        let loaded = load_connections(path);
        assert_eq!(connections, loaded);
    }
}
