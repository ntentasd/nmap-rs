mod args;
mod report;
mod scanner;

use args::Args;
use clap::Parser;
use scanner::Scanner;
use std::io::Result;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    for port in args.ports {
        let sc = Scanner::new(format!("{}:{}", args.address, port), args.tries).await?;

        let res = sc.scan(Duration::from_millis(args.timeout)).await?;

        println!("{}", res);
    }

    Ok(())
}
