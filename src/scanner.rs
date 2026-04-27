use std::{
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
};

use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug, PartialEq)]
pub struct Scanner {
    address: SocketAddr,
    tries: u8,
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

    pub async fn scan(&self) -> Result<()> {
        let mut last_err: Option<Error> = None;
        for _ in 0..self.tries {
            match TcpStream::connect(self.address).await {
                Ok(_stream) => return Ok(()),
                Err(e) => {
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap())
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
            .await;
        assert!(result.is_ok());
    }
}
