use std::{num::ParseIntError, str::FromStr};

use clap::Parser;

/// Simple program to scan ports
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of the port to scan
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub address: String,

    /// Ports to scan
    #[arg(short, long)]
    pub ports: Ports,

    /// Number of tries
    #[arg(short, long, default_value_t = 1)]
    pub tries: u8,

    /// Timeout duration in milliseconds
    #[arg(long, default_value_t = 1000)]
    pub timeout: u64,

    /// Show all ports, including closed and timed out
    #[arg(long, default_value_t = false)]
    pub all: bool,

    /// Number of concurrent scans
    #[arg(short = 'j', long, default_value_t = 5)]
    pub concurrency: usize,
}

#[derive(Clone, Debug)]
pub struct Ports(Vec<u16>);

impl Ports {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for Ports {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('-') {
            let mut split = s.split('-');
            let (Some(start), Some(end), None) = (split.next(), split.next(), split.next()) else {
                return Err("invalid range format".to_string());
            };
            let start: u16 = start.parse().map_err(|e: ParseIntError| e.to_string())?;
            let end: u16 = end.parse().map_err(|e: ParseIntError| e.to_string())?;

            if start > end {
                return Err("invalid range".to_string());
            }

            Ok(Ports((start..=end).collect()))
        } else {
            let ports = s
                .split(',')
                .map(|s| s.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()
                .map_err(|e| e.to_string())?;

            Ok(Ports(ports))
        }
    }
}

impl IntoIterator for Ports {
    type Item = u16;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
