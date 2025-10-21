use std::net::{TcpListener, TcpStream};
use std::io::Read;

struct Server {
    address: String,
}

impl Server {
    // Create a new server
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    // Bind to the port
    fn bind(&self) -> TcpListener {
        TcpListener::bind(&self.address).expect("Failed to bind to address")
    }

    // Listen and accept one client
    fn accept_client(&self, listener: &TcpListener) -> TcpStream {
        let (stream, addr) = listener.accept().expect("Failed to accept client");
        println!("Client connected: {}", addr);
        stream
    }

    // Read data from a client
    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from client");
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", message);
    }
}

fn main() {
    // Create server
    let server = Server::new("127.0.0.1:7878");

    // Bind to port
    let listener = server.bind();
    println!("Server running on {}", server.address);

    // Accept a client
    let stream = server.accept_client(&listener);

    // Handle the client's message
    server.handle_client(stream);
}
