use std::{
    io::{Error, ErrorKind, Result},
    net::{IpAddr, SocketAddr},
};

use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug, PartialEq)]
pub struct Scanner {
    address: SocketAddr,
    tries: u8,
}

#[derive(Debug, PartialEq)]
pub enum ScanStatus {
    Open,
    Closed,
    // TODO: add more options
}

#[derive(Debug)]
pub struct ScanResult {
    status: ScanStatus,
    tries: u8,
    address: IpAddr,
    port: u16,
}

impl ScanResult {
    fn new(status: ScanStatus, tries: u8, socket: SocketAddr) -> Self {
        let address = socket.ip();
        let port = socket.port();
        Self {
            status,
            tries,
            address,
            port,
        }
    }

    pub fn status(&self) -> &ScanStatus {
        &self.status
    }

    pub fn tries(&self) -> u8 {
        self.tries
    }

    pub fn address(&self) -> IpAddr {
        self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Scanner {
    pub async fn new(address: impl ToSocketAddrs, tries: u8) -> Result<Self> {
        if tries == 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid tries input"));
        }

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

    pub async fn scan(&self) -> Result<ScanResult> {
        let mut last_error: Option<Error> = None;
        for i in 0..self.tries {
            match TcpStream::connect(self.address).await {
                Ok(_stream) => {
                    return Ok(ScanResult::new(ScanStatus::Open, i + 1, self.address));
                }
                Err(e) => match e.kind() {
                    ErrorKind::ConnectionRefused => {
                        return Ok(ScanResult::new(ScanStatus::Closed, i + 1, self.address));
                    }
                    _ => {
                        last_error = Some(e);
                    }
                },
            }
        }
        Err(last_error.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_new() {
        let result = Scanner::new("127.0.0.1:31234", 3).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_new() {
        let result = Scanner::new("127.0.0.1:31234", 0).await;
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn test_port_scan() {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();

        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:31234")
                .await
                .unwrap();
            tx.send(()).unwrap();
            while let Ok((_, _)) = listener.accept().await {}
        });

        rx.await.unwrap();

        let result = Scanner::new("127.0.0.1:31234", 3)
            .await
            .unwrap()
            .scan()
            .await
            .unwrap();
        assert_eq!(result.status, ScanStatus::Open);
    }
}
