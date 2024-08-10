use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, Write};
use std::sync::{Arc, Mutex};
use tokio::task;
use super::cache_store::CacheStore;
use super::codec::RespCodec;
use super::model::RespValue;

pub struct RedisServer {
    host: String,
    port: u16,
    data_store: Arc<Mutex<CacheStore>>,
}

impl RedisServer {
    pub fn new(host: String, port: u16) -> Self {
        RedisServer {
            host,
            port,
            data_store: Arc::new(Mutex::new(CacheStore::new())),
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        println!("Listening on {}:{}", self.host, self.port);

        loop {
            let (stream, _) = listener.accept()?;
            let data_store = Arc::clone(&self.data_store);
            println!("Accepted connection");
            task::spawn(async move {
                if let Err(e) = handle_client(stream, data_store).await {
                    eprintln!("Error handling client: {}", e);
                }
            });
        }
    }
}


async fn handle_client(stream: TcpStream, data_store: Arc<Mutex<CacheStore>>) -> std::io::Result<()> {
    let mut redis_reader = BufReader::new(&stream);
    let mut redis_writer = BufWriter::new(&stream);

    loop {
        match RespCodec::decode(&mut redis_reader) {
            Ok(RespValue::Array(commands)) => {
                println!("handle_client: redis_reader: {:?}\n commands: {:?} \n \n", redis_reader, commands);
                let response = process_command(commands, &data_store);
                // writer.write_all(&RespCodec::encode(&response))?;
                redis_writer.write_all(&RespCodec::encode(&response))?;
                // redis_writer.write_all(&RespCodec::encode(&response))?;
                redis_writer.flush()?;
                println!("handle_client: response: {:?} \n \n", response);
            }
            Ok(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Expected array")),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
fn process_command(commands: Vec<RespValue>, data_store: &Arc<Mutex<CacheStore>>) -> RespValue {
    if commands.is_empty() {
        return RespValue::Error("ERR no command specified".to_string());
    }

    let command = match &commands[0] {
        RespValue::BulkString(s) | RespValue::SimpleString(s) => s.to_uppercase(),
        RespValue::BinaryBulkString(b) => {
            match String::from_utf8(b.clone()) {
                Ok(s) => s.to_uppercase(),
                Err(_) => return RespValue::Error("ERR invalid command".to_string()),
            }
        },
        _ => return RespValue::Error("ERR invalid command".to_string()),
    };

    match command.as_str() {
        "PING" => RespValue::SimpleString("PONG".to_string()),
        "SET" => {
            if commands.len() < 3 {
                return RespValue::Error("ERR wrong number of arguments for 'set' command".to_string());
            }
            let key = match &commands[1] {
                RespValue::BulkString(s) | RespValue::SimpleString(s) => s.clone(),
                RespValue::BinaryBulkString(b) => match String::from_utf8(b.clone()) {
                    Ok(s) => s,
                    Err(_) => return RespValue::Error("ERR invalid key".to_string()),
                },
                _ => return RespValue::Error("ERR invalid key".to_string()),
            };
            let value = match &commands[2] {
                RespValue::BulkString(s) | RespValue::SimpleString(s) => s.clone(),
                RespValue::BinaryBulkString(b) => match String::from_utf8(b.clone()) {
                    Ok(s) => s,
                    Err(_) => return RespValue::Error("ERR invalid value".to_string()),
                },
                _ => return RespValue::Error("ERR invalid value".to_string()),
            };
            let mut store = data_store.lock().unwrap();
            store.set(key, value, None);
            RespValue::SimpleString("OK".to_string())
        }
        "GET" => {
            if commands.len() < 2 {
                return RespValue::Error("ERR wrong number of arguments for 'get' command".to_string());
            }
            let key = match &commands[1] {
                RespValue::BulkString(s) | RespValue::SimpleString(s) => s.clone(),
                RespValue::BinaryBulkString(b) => match String::from_utf8(b.clone()) {
                    
                    Ok(s) => s,
                    Err(_) => return RespValue::Error("ERR invalid key".to_string()),
                },
                _ => return RespValue::Error("ERR invalid key".to_string()),
            };
            let store = data_store.lock().unwrap();
            match store.get(&key) {
                Some(value) => RespValue::BulkString(value),
                None => RespValue::Null,
            }
        }
        "ECHO" => {
            if commands.len() < 2 {
                return RespValue::Error("ERR wrong number of arguments for 'echo' command".to_string());
            }
            commands[1].clone()
        }
        _ => RespValue::Error("ERR unknown command".to_string()),
    }
}