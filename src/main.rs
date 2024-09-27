use anyhow::{Context, Result};
use serde::Deserialize;
use ssh2::Session;
use std::fs;
use std::io::Read;
use std::net::{Shutdown, TcpStream};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "tmux-wax", about = "Docker container status for TMUX")]
pub struct Opt {
    #[structopt(short, long, help = "Output for TMUX status bar")]
    pub tmux: bool,
}

pub fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();
    let config = read_config().context("Failed to read config")?;

    // Check if server or Docker is down before getting Docker stats
    if !check_server_status(&config) {
        display_server_down_message(opt.tmux);
        return Ok(()); // Exit after showing the error message
    }

    let stats = get_docker_stats(&config).context("Failed to get Docker stats")?;
    if opt.tmux {
        println!("{}", format_for_tmux(&stats));
    } else {
        println!("{}", format_for_prompt(&stats));
    }
    Ok(())
}

pub fn read_config() -> Result<Config> {
    let home = std::env::var("HOME").context("HOME environment variable not set")?;
    let config_path = PathBuf::from(home).join(".tmux-wax-env");
    let config_str = fs::read_to_string(config_path).context("Failed to read config file")?;
    let config: Config = toml::from_str(&config_str).context("Failed to parse config file")?;
    Ok(config)
}

pub fn check_server_status(config: &Config) -> bool {
    if let Ok(stream) = TcpStream::connect((config.host.as_str(), config.port)) {
        // Check if Docker daemon is responsive by attempting a handshake
        if let Ok(mut sess) = Session::new() {
            sess.set_tcp_stream(stream.try_clone().expect("Failed to clone TCP stream"));
            if sess.handshake().is_ok() {
                // Close the connection properly after handshake
                stream
                    .shutdown(Shutdown::Both)
                    .expect("Failed to shutdown stream");
                return true;
            }
        }
    }
    false // Server or Docker is unreachable
}

pub fn display_server_down_message(tmux: bool) {
    if tmux {
        println!("#[fg=red,bold]🔴 Server or container is down#[fg=default,nobold]");
    } else {
        println!("\x1b[31;1m🔴 Server or container is down\x1b[0m");
    }
}

pub fn get_docker_stats(config: &Config) -> Result<(usize, usize, usize, usize)> {
    let tcp = TcpStream::connect((config.host.as_str(), config.port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(&config.username, &config.password)?;
    let mut channel = sess.channel_session()?;
    channel.exec("docker ps -a --format '{{.State}}'")?;
    let mut output = String::new();
    channel.read_to_string(&mut output)?;

    let mut total = 0;
    let mut up = 0;
    let mut down = 0;
    let mut failed = 0;

    for state in output.lines() {
        total += 1;
        match state {
            "running" => up += 1,
            "exited" => down += 1,
            _ => failed += 1,
        }
    }
    Ok((total, up, down, failed))
}

pub fn format_for_tmux(stats: &(usize, usize, usize, usize)) -> String {
    format!(
        "T:{} U:{} D:{} #[fg=red,bold]F:{}#[fg=default,nobold]",
        stats.0, stats.1, stats.2, stats.3
    )
}

pub fn format_for_prompt(stats: &(usize, usize, usize, usize)) -> String {
    format!(
        "Docker Containers:\nTotal: {}\nUp: {}\nDown: {}\nFailed: {}",
        stats.0, stats.1, stats.2, stats.3
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use std::net::TcpListener;
    use std::thread;

    fn setup_server(port: u16) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        thread::spawn(move || for _ in listener.incoming() {});
    }

    #[test]
    fn test_read_config() {
        // Prepare a mock configuration file
        let config_content = r#"
        username = "testuser"
        password = "testpass"
        host = "127.0.0.1"
        port = 22
        "#;
        let home = std::env::var("HOME").unwrap();
        let config_path = PathBuf::from(home).join(".tmux-wax-env");
        fs::write(&config_path, config_content).unwrap();

        // Attempt to read the config
        let config = read_config().expect("Failed to read config");
        assert_eq!(config.username, "testuser");
        assert_eq!(config.password, "testpass");
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 22);

        // Clean up
        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_check_server_status() {
        setup_server(2222);
        let config = Config {
            username: "testuser".to_string(),
            password: "testpass".to_string(),
            host: "127.0.0.1".to_string(),
            port: 2222,
        };
        assert!(check_server_status(&config));
    }

    #[test]
    fn test_display_server_down_message() {
        // Test the non-tmux output case
        let mut cmd = Command::cargo_bin("tmux-wax").unwrap();
        cmd.arg("--tmux=false")
            .assert()
            .stdout(predicates::str::contains(
                "\x1b[31;1m🔴 Server or container is down\x1b[0m",
            ));
    }

    #[test]
    fn test_get_docker_stats() {
        // Simulate a Docker server that returns known output
        setup_server(2222);
        let config = Config {
            username: "testuser".to_string(),
            password: "testpass".to_string(),
            host: "127.0.0.1".to_string(),
            port: 2222,
        };

        // Assuming the command would return known states, we could mock the output
        // This test should ideally involve mocking the Session and Channel.
        let result = get_docker_stats(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_for_tmux() {
        let stats = (10, 5, 3, 2);
        let formatted = format_for_tmux(&stats);
        assert_eq!(
            formatted,
            "T:10 U:5 D:3 #[fg=red,bold]F:2#[fg=default,nobold]"
        );
    }

    #[test]
    fn test_format_for_prompt() {
        let stats = (10, 5, 3, 2);
        let formatted = format_for_prompt(&stats);
        assert_eq!(
            formatted,
            "Docker Containers:\nTotal: 10\nUp: 5\nDown: 3\nFailed: 2"
        );
    }
}
