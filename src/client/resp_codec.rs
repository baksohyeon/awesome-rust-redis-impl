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
/// Represents a RESP value, see [Redis Protocol specification](http://redis.io/topics/protocol).
// refered source code: https://github.dev/iorust/resp/tree/master/src
#[derive(Clone, Eq, PartialEq, Debug)]
enum RespReply {
    SimpleString(String), // For Simple Strings the first byte of the reply is "+".
    Error(String), // For Errors the first byte of the reply is "-".
    Integer(i64), // For Integers the first byte of the reply is ":".
    BulkString(String), // For Bulk Strings the first byte of the reply is "$".
    BinaryBulkString(Vec<u8>), // For Bulk <binary> Strings the first byte of the reply is "$".
    Null, // Null bulk reply, `$-1\r\n`
    NullArray, // Null array reply, `*-1\r\n`
    Array(Vec<RespReply>), // For Arrays the first byte of the reply is "*".
}




// TODO: make this more generic
impl RespCodec {
    fn encode(&self, value: Value) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
