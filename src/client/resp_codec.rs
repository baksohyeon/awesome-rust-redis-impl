pub struct RespCodec {
    buffer: BytesMut,
}


// RESP2 specification
// https://redis.io/topics/protocol

enum Types {
    SIMPLE_STRING = 43, // +
    ERROR = 45, // -
    INTEGER = 58, // :
    BULK_STRING = 36, // $
    ARRAY = 42 // *
}

enum ASCII {
    CR = 13, // \r
    ZERO = 48,
    MINUS = 45
}

// TODO: make this more generic
impl RespCodec {
    fn encode(&self, value: Value) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
