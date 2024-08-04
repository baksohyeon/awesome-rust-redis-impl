pub trait Codec {
    fn encode(&self, value: &str) -> String;
    fn decode(&self, value: &str) -> String;
}

pub struct RedisCodec {
    pub host: String,
    pub port: i16,
}

impl Codec for RedisCodec {
    fn encode(&self, value: &str) -> String {
        format!("*{}", value.len())
    }

    fn decode(&self, value: &str) -> String {
        format!("*{}", value.len())
    }
}
