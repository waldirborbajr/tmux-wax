WARNING This project is in its very initial development stage, not all features are implemented yet, usage API still subjected to change until 1.0.0

# TMUX-WAX

<p align="center">
  <img width="256" height="256" src="https://github.com/user-attachments/assets/0c390b7e-1b04-4ece-bebe-2adddaa98875" />
</p>

TMUX-WAX is a Rust CLI application that connects to a remote server using SSH and retrieves Docker container statistics. It can display the results in a TMUX status bar or at the command prompt.

## Features

- Secure SSH connection to remote server
- Retrieval of Docker container statistics
- Display of total containers, running containers, stopped containers, and failed containers
- Output formatting for TMUX status bar or command prompt

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```
   git clone https://github.com/yourusername/tmux-wax.git
   ```
3. Navigate to the project directory:
   ```
   cd tmux-wax
   ```
4. Build the project:
   ```
   cargo build --release
   ```
5. The binary will be available at `target/release/tmux-wax`

## Configuration

Create a `.tmux-wax-env` file in your home directory with the following content:

```toml
username = "your_ssh_username"
password = "your_ssh_password"
host = "your_remote_host"
port = 22
```

Replace the values with your actual SSH credentials and remote host information.

## Usage

To display Docker stats in the terminal:

```
tmux-wax
```

To output Docker stats for TMUX status bar:

```
tmux-wax --tmux
```

## TMUX Integration

To integrate with TMUX, add the following line to your `.tmux.conf` file:

```
set -g status-right '#(tmux-wax --tmux)'
```

## Security Considerations

- The application uses SSH for secure communication with the remote server.
- Credentials are stored in a separate configuration file for better security management.
- The SSH2 crate is used with the `vendored-openssl` feature to ensure a consistent and up-to-date OpenSSL implementation.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
