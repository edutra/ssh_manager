# SSH Manager CLI

A powerful and user-friendly CLI tool to manage your SSH connections effectively.

## Overview

SSH Manager simplifies managing multiple SSH connections by allowing you to add, list, delete, and open connections directly from the command line. With additional features like editing connection details, running scripts remotely, and executing commands from URLs, SSH Manager is a versatile tool for developers and system administrators.

## Features

- **Add new SSH connections** with custom details like name, host, port, username, and optional welcome messages.
- **List saved connections** in a formatted output.
- **Delete or edit existing connections** easily by name.
- **Open connections** seamlessly using pre-configured settings.
- **Execute remote scripts** or download and execute commands from URLs.
- Simple **JSON-based configuration** for storing and retrieving connection details.

## Installation

Clone the repository and build the tool:

```bash
$ git clone https://github.com/your_username/ssh_manager
$ cd ssh_manager
$ cargo build --release
```

## Usage

The general command format for SSH Manager is:

```bash
$ ssh_manager <COMMAND> [OPTIONS]
```

### Commands

#### Add a new connection:
```bash
$ ssh_manager [--add|-a] <NAME> <HOST> <PORT> <USERNAME> [WELCOME_MESSAGE]
```

#### List all connections:
```bash
$ ssh_manager [--list|-l]
```

#### Delete a connection:
```bash
$ ssh_manager [--delete|-d] <NAME>
```

#### Open a connection:
```bash
$ ssh_manager [--open|-o] <NAME>
```

#### Edit a connection's property:
```bash
$ ssh_manager [--edit|-e] <NAME.PROPERTY> <NEW_VALUE>
```

#### Run a script remotely:
```bash
$ ssh_manager [--snippet|-s] <NAME> <PATH_TO_SCRIPT>
```

#### Execute a URL command:
```bash
$ ssh_manager [--url|-u] <NAME> <URL>
```

### Options

- `-h`, `--help`: Display help for the tool or specific subcommands.

## Example

To add a new SSH connection:
```bash
$ ssh_manager --add myserver example.com 22 myuser "Welcome to My Server"
```

To open the connection:
```bash
$ ssh_manager --open myserver
```

To edit a connection:
```bash
$ ssh_manager --edit myserver.username newusername
```

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue to suggest features, report bugs, or improve the documentation.
