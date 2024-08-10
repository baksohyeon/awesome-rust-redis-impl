use std::net::TcpListener;
use std::net::TcpStream;

pub trait Connection {
    fn new(options: RedisConfig) -> Self;
    fn create_listener(&self) -> TcpListener;
    fn create_stream(&self, listener: TcpListener) -> TcpStream;
}

pub struct RedisConfig {
    pub host: String,
    pub port: i16,
}

pub struct ConnectionImpl {
    host: String,
    port: i16,
}


impl Connection for ConnectionImpl {
    fn new(options: RedisConfig) -> Self {
        Self {
            host: options.host,
            port: options.port,
        }
    }

    fn create_listener(&self) -> TcpListener {
        let address = format!("{}:{}", self.host, self.port);
        println!("Creating listener, {}", address);
        TcpListener::bind(address).expect("Failed to bind to socket")
    }

    fn create_stream(&self, listener: TcpListener) -> TcpStream {
        loop {
            let (socket, _) = listener.accept().expect("Failed to accept connection");
            println!("Connection accepted: {:?}", socket);
            break socket;
        }
    }
}
