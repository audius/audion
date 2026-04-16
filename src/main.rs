use std::error::Error;
use std::fmt;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use chrono::{Local};
use std::time::Duration;
use surge_ping::{Client, Config, SurgeError, PingIdentifier, PingSequence};
use tokio::time::timeout;
use seahorse::{App, Context, Flag, FlagType};

const DEFAULT_TARGET: &str = "127.0.0.1";
const DEFAULT_OUTPUT_FILE: &str = "output.txt";
const DEFAULT_TIMEOUT_MS: isize = 1000;
const DEFAULT_INTERVAL_SECS: isize = 60;

#[derive(Debug)]
enum CustomError {
    HostResolutionFailure,
    SurgeError(SurgeError),
    IoError(std::io::Error),
    TimeoutError,
}

impl From<SurgeError> for CustomError {
    fn from(error: SurgeError) -> Self {
        CustomError::SurgeError(error)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IoError(error)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::HostResolutionFailure => write!(f, "Host resolution failure"),
            CustomError::SurgeError(e) => write!(f, "Surge error: {:?}", e),
            CustomError::IoError(e) => write!(f, "I/O error: {}", e),
            CustomError::TimeoutError => write!(f, "Timeout error"),
        }
    }
}

impl Error for CustomError {}

async fn ping_host(target_host: &str, output_file: &str, timeout_ms: u64, interval_secs: u64, verbose: bool) -> Result<(), CustomError> {
    let ip_addr = match target_host.parse::<Ipv4Addr>() {
        Ok(ip) => IpAddr::V4(ip),
        Err(_) => {
            let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
            let error_msg = format!("{} Invalid IP address provided\n", timestamp);
            append_to_file(output_file, &error_msg)?;
            return Err(CustomError::HostResolutionFailure);
        }
    };

    let config = Config::default();
    let client = Client::new(&config).map_err(CustomError::IoError)?;
    let mut pinger = client.pinger(ip_addr, PingIdentifier(0)).await;

    loop {
        let result = match timeout(Duration::from_millis(timeout_ms), pinger.ping(PingSequence(0), &[0])).await {
            Ok(Ok((_packet, _duration))) => {
                format!("Host {} is reachable", target_host)
            }
            Ok(Err(_)) => {
                format!("Host {} is unreachable", target_host)
            }
            Err(_) => {
                return Err(CustomError::TimeoutError);
            }
        };

        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
        let content = format!("{} {}", timestamp, result);
        append_to_file(output_file, &content)?;
        if verbose {
            println!("{}", content);
        }

        tokio::time::sleep(Duration::from_secs(interval_secs)).await;
    }
}

fn append_to_file(file_path: &str, content: &str) -> Result<(), CustomError> {
    if !fs::metadata(file_path).is_ok() {
        let _ = OpenOptions::new().create(true).write(true).open(file_path)?;
    }

    let mut file = OpenOptions::new().append(true).open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new("audion")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("audion [args]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flag(Flag::new("target", FlagType::String).description("Target host to ping (default: 127.0.0.1)").alias("t"))
        .flag(Flag::new("output", FlagType::String).description("Output file to write results (default: output.txt)").alias("o"))
        .flag(Flag::new("timeout", FlagType::Int).description("Timeout in milliseconds (default: 1000)").alias("to"))
        .flag(Flag::new("interval", FlagType::Int).description("Interval between pings in seconds (default: 60)").alias("i"))
        .flag(Flag::new("verbose", FlagType::Bool).description("Print output to stdout").alias("v"));

    app.run(args);
}

fn action(context: &Context) {
    let target = context.string_flag("target").unwrap_or(DEFAULT_TARGET.to_string());
    let output_file = context.string_flag("output").unwrap_or(DEFAULT_OUTPUT_FILE.to_string());
    let timeout_ms = context.int_flag("timeout").unwrap_or(DEFAULT_TIMEOUT_MS) as u64;
    let interval_secs = context.int_flag("interval").unwrap_or(DEFAULT_INTERVAL_SECS) as u64;
    let verbose = context.bool_flag("verbose");

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        if let Err(err) = ping_host(&target, &output_file, timeout_ms, interval_secs, verbose).await {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    });
}
