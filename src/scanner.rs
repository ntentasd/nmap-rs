use std::{io::Result, net::SocketAddr};

use tokio::net::ToSocketAddrs;

pub struct Scanner {
    address: SocketAddr,
    tries: u8,
}

impl Scanner {
    pub async fn new(address: impl ToSocketAddrs, tries: u8) -> Result<Self> {
        let address = tokio::net::lookup_host(address)
            .await?
            .next()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "could not resolve address",
                )
            })?;
        Ok(Scanner { address, tries })
    }

    pub async fn scan(&self) -> Result<()> {
        let _stream = tokio::net::TcpStream::connect(self.address).await?;
        let _ = self.tries;
        Ok(())
    }
}
