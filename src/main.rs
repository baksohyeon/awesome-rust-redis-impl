mod client;

use std::env;

use client::connection::RedisServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Redis 서버 시작 중...");

    let port: u16 = env::args().nth(2).unwrap_or("6379".to_string()).parse().unwrap();


    let server = RedisServer::new("127.0.0.1".to_string(), port);
    server.run().await
}
