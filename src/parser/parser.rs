// pub const CRLF: &[u8] = b"\r\n";


pub enum Value {
    SimpleString(String ),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<Value>),
}


pub struct RespCodec {
    decode_command: fn(&mut BufMut) -> Result<Value, Box<dyn Error>>,
    encode_command: fn(&mut BufMut, Value) -> Result<(), Box<dyn Error>>,
}



pub impl RespCodec {
    pub fn decode(&self, buffer: &mut BufMut) -> Result<Value, Box<dyn Error>> {
        let mut value = Value::SimpleString(String::new());

        if (buffer.is_empty()) {
            return Err(Box::new(io::E));
        }

        match src[0] {
            b'+' => {
                value = Value::decode_SimpleString(buffer)
            }
            b'-' => {
                value = Value::decode_Error(buffer)
            }
            b':' => {
                value = Value::decode_Integer(buffer)
            }
            b'$' => {
                value = Value::decode_BulkString(buffer)
            }
            b'*' => {
                value = Value::decode_Array(buffer)
            }
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

    fn decode_simple_string(&mut buffer: &mut BufMut) -> Result<Value, Box<dyn Error>> {
        if let Some(command) = self.read_line(buffer)? {
            Ok(Value::SimpleString(command.freeze()))
        } else {
            Ok(None)
        }

        if (buffer.is_empty()) {
            return Err(Box::new(io::E));
        }
    }


    fn read_line(&self, buffer: &mut BufMut) -> Result<Value, Box<dyn Error>> {
        if let Some(line) = self.iter().position(|&x| x == b'\n') {
            let line = src.split_to(line + 1);
            line.truncate(line.len() - 2); // delete CRLF (\r\n)
            Ok(Some(line))
        } else {
            Ok(None)
        }
    }


}