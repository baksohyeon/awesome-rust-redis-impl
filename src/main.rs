use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to port 6379");

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().unwrap();
        // A new task is spawned for each inbound socket.
        // The socket is moved to the new task and processed there.
        tokio::spawn(async move { handle_connection_process(socket).await });
    }
}

pub async fn handle_connection_process(stream: TcpStream) {
  println!("accepted new connection");
  println!("{:?}", stream);
  let mut tcp_stream = stream;

    let mut buffer = [0; 1024];

  loop {
      match tcp_stream.read(&mut buffer) {
          Ok(0) => break,
          Ok(_) => {
            let req = String::from_utf8_lossy(&buffer);
            let res = process_request(&req);
              tcp_stream
                  .write_all(res.as_bytes())
                  .expect("Failed to write to stream");
          }
          Err(e) => {
              println!("multiple Ping response error: {}", e);
              break;
          }
      }
  }

    println!("read from stream: {}", String::from_utf8_lossy(&buffer));
}


fn process_request(request: &str) -> String {
  let parts: Vec<&str> = request.split("\r\n").collect();
  if parts.len() > 2 && parts[2] == "ECHO" {
      let message = parts[4];
      return format!("${}\r\n{}\r\n", message.len(), message)
  }
  "+PONG\r\n".to_string()
}