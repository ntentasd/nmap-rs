mod args;
mod report;
mod scanner;

use args::Args;
use clap::Parser;
use scanner::Scanner;
use std::{io::Result, sync::Arc};
use tokio::{sync::Semaphore, task::JoinSet, time::Duration};

use crate::scanner::ScanStatus;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut js = JoinSet::new();
    let size = args.ports.len();

    let semaphore = Arc::new(Semaphore::new(args.concurrency));

    for port in args.ports {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let address = args.address.clone();
        js.spawn(async move {
            let _permit = permit;
            let sc = Scanner::new(format!("{}:{}", address, port), args.tries).await?;
            sc.scan(Duration::from_millis(args.timeout)).await
        });
    }

    let mut results = Vec::with_capacity(size);

    while let Some(res) = js.join_next().await {
        match res {
            Ok(Ok(scan_result)) => results.push(scan_result),
            Ok(Err(e)) => eprintln!("error: {}", e),
            Err(e) => eprintln!("task panicked: {}", e),
        }
    }

    results.sort_by_key(|r| r.port());
    for res in &results {
        if args.all || res.status() == &ScanStatus::Open {
            println!("{res}");
        }
    }

    Ok(())
}
