use anyhow::{Context, Result};
use serde::Deserialize;
use ssh2::Session;
use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Deserialize)]
struct Config {
    username: String,
    password: String,
    host: String,
    port: u16,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "tmux-wax", about = "Docker container status for TMUX")]
struct Opt {
    #[structopt(short, long, help = "Output for TMUX status bar")]
    tmux: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    let config = read_config().context("Failed to read config")?;
    let stats = get_docker_stats(&config).context("Failed to get Docker stats")?;

    if opt.tmux {
        println!("{}", format_for_tmux(&stats));
    } else {
        println!("{}", format_for_prompt(&stats));
    }

    Ok(())
}

fn read_config() -> Result<Config> {
    let home = std::env::var("HOME").context("HOME environment variable not set")?;
    let config_path = PathBuf::from(home).join(".tmux-wax-env");
    let config_str = fs::read_to_string(config_path).context("Failed to read config file")?;
    let config: Config = toml::from_str(&config_str).context("Failed to parse config file")?;
    Ok(config)
}

fn get_docker_stats(config: &Config) -> Result<(usize, usize, usize, usize)> {
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

fn format_for_tmux(stats: &(usize, usize, usize, usize)) -> String {
    format!("🐳 T:{} U:{} D:{} F:{}", stats.0, stats.1, stats.2, stats.3)
}

fn format_for_prompt(stats: &(usize, usize, usize, usize)) -> String {
    format!(
        "Docker Containers:\nTotal: {}\nUp: {}\nDown: {}\nFailed: {}",
        stats.0, stats.1, stats.2, stats.3
    )
}
