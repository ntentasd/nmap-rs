mod report;
mod scanner;

use clap::Parser;
use scanner::Scanner;
use std::io::Result;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let sc = Scanner::new(format!("{}:{}", args.address, args.port), args.tries).await?;

    sc.scan().await?;

    Ok(())
}
