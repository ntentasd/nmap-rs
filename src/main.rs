mod report;
mod scanner;

use clap::Parser;
use scanner::Scanner;
use std::io::Result;
use tokio::time::Duration;

/// Simple program to scan ports
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address of the port to scan
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    address: String,

    /// Port to scan
    #[arg(short, long)]
    port: u16,

    /// Number of tries
    #[arg(short, long, default_value_t = 1)]
    tries: u8,

    /// Timeout duration in milliseconds
    #[arg(long, default_value_t = 3)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let sc = Scanner::new(format!("{}:{}", args.address, args.port), args.tries).await?;

    let res = sc.scan(Duration::from_millis(args.timeout)).await?;

    println!("{}", res);

    Ok(())
}
