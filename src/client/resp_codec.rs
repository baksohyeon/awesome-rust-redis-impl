pub struct RespCodec {
    write_fn: fn(chunk: BufMut) -> Result<(), Box<dyn Error>>,
    flush_fn: fn() -> Result<(), Box<dyn Error>>,
}

pub const CRLF: &str = "\r\n";


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
enum RespValue {
    SimpleString(String), // For Simple Strings the first byte of the reply is "+".
    Error(String), // For Errors the first byte of the reply is "-".
    Integer(i64), // For Integers the first byte of the reply is ":".
    BulkString(String), // For Bulk Strings the first byte of the reply is "$".
    BinaryBulkString(Vec<u8>), // For Bulk <binary> Strings the first byte of the reply is "$".
    Null, // Null bulk reply, `$-1\r\n`
    NullArray, // Null array reply, `*-1\r\n`
    Array(Vec<RespValue>), // For Arrays the first byte of the reply is "*".
}

// refered source code: https://github.dev/iorust/resp/tree/master/src
impl RespValue {

    /// Returns `true` if the value is a `Null` or `NullArray`. Returns `false` otherwise.
    /// # Examples
    /// ```
    /// # use self::client::{RespValue};
    /// assert_eq!(RespValue::Null.is_null(), true);
    /// assert_eq!(RespValue::NullArray.is_null(), true);
    /// assert_eq!(RespValue::Integer(123).is_null(), false);
    /// ```
    pub fn is_null(&self) -> bool {
        // TODO: self 와 *self 의 차이
        match *self { 
            RespValue::Null | RespValue::NullArray => true,
            _ => false
        }
    }

    /// Returns `true` if the value is a `Error`. Returns `false` otherwise.
    /// # Examples
    /// ```
    /// # use self::resp::{Value};
    /// assert_eq!(Value::Null.is_error(), false);
    /// assert_eq!(Value::Error("".to_string()).is_error(), true);
    /// ```
    pub fn is_error(&self) -> bool {
        match *self {
            RespValue::Error(_) => true,
            _ => false
        }
    }

}



// TODO: make this more generic
impl RespCodec {
    fn encode(&self, value: Value) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

struct Encoder {
    encode_simple_string_fn: fn(value: &str) -> Result<(), Box<dyn Error>>,
    encode_error_fn: fn(value: &str) -> Result<(), Box<dyn Error>>,
    encode_integer_fn: fn(value: i64) -> Result<(), Box<dyn Error>>,
    encode_bulk_string_fn: fn(value: &str) -> Result<(), Box<dyn Error>>,
    encode_binary_bulk_string_fn: fn(value: &[u8]) -> Result<(), Box<dyn Error>>,
    encode_null_fn: fn() -> Result<(), Box<dyn Error>>,
    encode_null_array_fn: fn() -> Result<(), Box<dyn Error>>,
    encode_array_fn: fn(value: &[&str]) -> Result<(), Box<dyn Error>>,
}

impl encoder for Encoder {
    fn encode_simple_string_fn(&self, command: &str, buffer: &mut BufMut) -> Result<(), Box<dyn Error>> {
        let to_write = BufMut::new();
        to_write.put_u8(b'+');
        to_write.put_slice(command.as_bytes());
        to_write.put_slice(CRLF);
        buffer.put_slice(to_write.as_bytes());
    }
}


pub fn encode(&self, value: &Value, buffer: &mut BufMut) -> Result<(), Box<dyn Error>> {
    if (value.is_empty()) {
        return Err(Box::new(io::E));
    }


    match value {
        Value::SimpleString(s) => {
            buffer.put_u8(b'+');
            buffer.put_slice(s.as_bytes());
            buffer.put_slice(CRLF);
        }
        Value::Error(s) => {
            // buffer.write_all();
        }
    }
}


pub fn read_line(&self, buffer: &mut BufMut) -> Result<Value, Box<dyn Error>> {
    if let Some(line) = self.iter().position(|&x| x == b'\n') {
        let line = src.split_to(line + 1);
        line.truncate(line.len() - 2); // delete CRLF (\r\n)
        Ok(Some(line))
    } else {
        Ok(None)
    }
}

// --
 
// const toWrite: Array<RedisCommandArgument> = [];

// let strings = '*' + args.length + CRLF;

// for (let i = 0; i < args.length; i++) {
//     const arg = args[i];
//     if (typeof arg === 'string') {
//         strings += '$' + Buffer.byteLength(arg) + CRLF + arg + CRLF;
//     } else if (arg instanceof Buffer) {
//         toWrite.push(
//             strings + '$' + arg.length.toString() + CRLF,
//             arg
//         );
//         strings = CRLF;
//     } else {
//         throw new TypeError('Invalid argument type');
//     }
// }

// toWrite.push(strings);

// return toWrite;