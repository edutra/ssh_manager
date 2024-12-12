# SSH Manager CLI

A simple CLI tool for managing your SSH connections.

## Overview

SSH Manager allows you to easily add, list, delete, and open SSH connections through a command-line interface. This tool is designed to streamline SSH management, making it easier to work with multiple servers or remote machines.

## Installation

Clone the repository and install the tool:

```bash
$ git clone https://github.com/edutra/ssh_manager
$ cd ssh_manager
$ cargo build --release
```

## Usage

The general command format for SSH Manager is:

```bash
$ ssh_manager <COMMAND>
```

### Commands

- `Add a connection`\
  Add a new SSH connection. You will be prompted to enter the name and details of the connection.

  ```bash
  $ ssh_manager [--add|-a] <NAME> <HOST> <PORT> <USERNAME> [WELCOMME_MESSAGE]
  ```

- `List all connections`\
  List all saved SSH connections.

  ```bash
  $ ssh_manager [--list|-l]
  ```

- `Delete a connectiion`\
  Delete an SSH connection by its name.

  ```bash
  $ ssh_manager [--delete|-d] <NAME>
  ```

- `Open a connectionn`\
  Open an SSH connection by its name.

  ```bash
  $ ssh_manager [--open|-o] <NAME>
  ```

  - `Open an SSH connection by its name.`\
  Edit a connection's propertyy
  ```bash
  $ ssh_manager [--edit|-e] <NAME.PRPERTY> <NEW_VALUE>
  ```


- `help`\
  Print help information for the available commands.

  ```bash
  $ ssh_manager help
  ```

### Options

- `-h`, `--help`\
  Print help for the tool or the specific subcommand.

## Example

To add a new SSH connection:

```bash
$ ssh_manager add name foo.bar 6969 myUser
```

To connect to `name`:

```bash
$ ssh_manager open name
```

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue for suggestions or bug reports.

