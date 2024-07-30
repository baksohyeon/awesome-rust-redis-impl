
// RESP2 specification
// https://redis.io/topics/protocol



pub const CRLF: &str = "\r\n";

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