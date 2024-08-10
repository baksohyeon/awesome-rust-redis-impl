use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, Write};
use std::sync::{Arc, Mutex};
use tokio::task;
use super::composer::DataStore;
use super::codec::RespCodec;
use super::model::RespValue;

pub struct RedisServer {
    host: String,
    port: u16,
    data_store: Arc<Mutex<DataStore>>,
}

impl RedisServer {
    pub fn new(host: String, port: u16) -> Self {
        RedisServer {
            host,
            port,
            data_store: Arc::new(Mutex::new(DataStore::new())),
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        println!("Listening on {}:{}", self.host, self.port);

        loop {
            let (stream, _) = listener.accept()?;
            let data_store = Arc::clone(&self.data_store);
            task::spawn(async move {
                if let Err(e) = handle_client(stream, data_store).await {
                    eprintln!("Error handling client: {}", e);
                }
            });
        }
    }
}

async fn handle_client(stream: TcpStream, data_store: Arc<Mutex<DataStore>>) -> std::io::Result<()> {
    let mut redis_reader = BufReader::new(&stream);
    let mut redis_writer = BufWriter::new(&stream);

    loop {
        match RespCodec::decode(&mut redis_reader) {
            Ok(RespValue::Array(commands)) => {
                let response = process_command(commands, &data_store);
                // writer.write_all(&RespCodec::encode(&response))?;
                redis_writer.write_all(&RespCodec::encode(&response))?;
                redis_writer.flush()?;
            }
            Ok(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Expected array")),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn process_command(commands: Vec<RespValue>, data_store: &Arc<Mutex<DataStore>>) -> RespValue {
    if let Some(RespValue::BulkString(command)) = commands.get(0) {
        match command.to_uppercase().as_str() {
            "PING" => RespValue::SimpleString("PONG".to_string()),
            "SET" => {
                if let (Some(RespValue::BulkString(key)), Some(RespValue::BulkString(value))) = (commands.get(1), commands.get(2)) {
                    let mut store = data_store.lock().unwrap();
                    store.set(key.clone(), value.clone(), None);
                    RespValue::SimpleString("OK".to_string())
                } else {
                    RespValue::Error("ERR wrong number of arguments for 'set' command".to_string())
                }
            }
            "GET" => {
                if let Some(RespValue::BulkString(key)) = commands.get(1) {
                    let store = data_store.lock().unwrap();
                    match store.get(key) {
                        Some(value) => RespValue::BulkString(value),
                        None => RespValue::Null,
                    }
                } else {
                    RespValue::Error("ERR wrong number of arguments for 'get' command".to_string())
                }
            }
            _ => RespValue::Error("ERR unknown command".to_string()),
        }
    } else {
        RespValue::Error("ERR invalid command".to_string())
    }
}