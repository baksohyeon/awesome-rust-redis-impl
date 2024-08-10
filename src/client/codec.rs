use std::io::{self, BufRead, BufReader};
use super::model::RespValue;
/// up to 512 MB in length
const RESP_MAX_SIZE: i64 = 512 * 1024 * 1024;
const CRLF_BYTES: &'static [u8] = b"\r\n";
const CR_BYTES: &'static [u8] = b"\r";
const LF_BYTES: &'static [u8] = b"\n";
const NULL_BYTES: &'static [u8] = b"$-1\r\n";
const NULL_ARRAY_BYTES: &'static [u8] = b"*-1\r\n";


// #[derive(Debug)]
// enum BufferByteTypes  {
//     SimpleString = 43, // +
//     Error = 45, // -
//     Integer = 58, // :
//     BulkString = 36, // $
//     Array = 42, // *
// }

// impl BufferByteTypes {
//     fn from_byte(self) -> u8 {
//         match self {
//             BufferByteTypes::SimpleString => 43,
//             BufferByteTypes::Error => 45,
//             BufferByteTypes::Integer => 58,
//             BufferByteTypes::BulkString => 36,
//             BufferByteTypes::Array => 42,
//         }
//     }
// }


#[derive(Debug)]
pub struct Decoder<R> {
    pub is_binary: bool,
    pub reader: BufReader<R>,
}

impl<R: BufRead> Decoder<R> {
    /// Creates a Decoder instance with given BufReader for decoding the RESP buffers.
    pub fn new(reader: BufReader<R>) -> Self {
        Decoder {
            is_binary: false,
            reader: reader,
        }
    }

    // https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_until
    pub fn decode(&mut self) -> Result<RespValue, io::Error> {
        let mut buffer_byte: Vec<u8> = Vec::new();
        self.reader.read_until(b'\n', &mut buffer_byte)?;
        let len = buffer_byte.len();
        // EOF error
        if len == 0 || (len == 1 && buffer_byte[0] == CR_BYTES[0]) {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected EOF"));
        }


        if len < 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input length too short"));
        }

        if buffer_byte[len - 2] != CR_BYTES[0] && buffer_byte[len - 1] != LF_BYTES[0] {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input length too short"));
        }


        let byte_slice = &buffer_byte[1..len - 2];



        if byte_slice.len() > RESP_MAX_SIZE as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input is too large"));
        }

        match buffer_byte[0] {
            b'+' => {
                let value = String::from_utf8(byte_slice.to_vec()).unwrap();
                Ok(RespValue::SimpleString(value))
            }
            b'-' => {
                let value = String::from_utf8(byte_slice.to_vec()).unwrap();
                Ok(RespValue::Error(value))
            }
            b'$' => {
                let value = String::from_utf8(byte_slice.to_vec()).unwrap();
                Ok(RespValue::BulkString(value))
            }
            _ => {
                Ok(RespValue::SimpleString("PONG".to_string()))
            }
        }
    }
}



// pub trait Codec {
//     fn encode(&self, value: &str) -> String;
//     fn decode(&self, value: &str) -> Result<RespValue>;
// }

// pub struct RedisCodec {
//     pub host: String,
//     pub decoder: Decoder<R>,
// }

// impl Codec for RedisCodec {
//     fn encode(&self, value: &str) -> String {
//         format!("*{}", value.len())
//     }

    
// }


