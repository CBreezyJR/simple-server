use std::net::{TcpListener, TcpStream};
use std::io::Read;
use anyhow::{anyhow, Result};

struct Server {
    address: String,
}

impl Server {
    // Create a new server instance
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    // Example function showing anyhow error
    fn my_function() -> Result<()> {
        Err(anyhow!("Something went wrong"))
    }

    // Bind to the port
    fn bind(&self) -> Result<TcpListener> {
        let listener = TcpListener::bind(&self.address)?;
        Ok(listener)
    }

    // Listen and accept one client
    fn accept_client(&self, listener: &TcpListener) -> Result<TcpStream> {
        let (stream, _) = listener.accept()?; // returns (TcpStream, SocketAddr)
        Ok(stream)
    }

    // Read data from a client
    fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", message);
        Ok(())
    }
}

fn main() -> Result<()> {
    // Create server
    let server = Server::new("127.0.0.1:7878");

    Server::my_function()?;

    // Bind to port
    let listener = server.bind()?;
    println!("Server running on {}", server.address);

    // Accept a client
    let stream = server.accept_client(&listener)?;
    println!("🔗 Client connected!");

    // Handle the client's message
    server.handle_client(stream)?;

    Ok(())
}
