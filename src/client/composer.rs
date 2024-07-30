/// up to 512 MB in length
const RESP_MAX_SIZE: i64 = 512 * 1024 * 1024;
const CRLF_BYTES: &'static [u8] = b"\r\n";
const NULL_BYTES: &'static [u8] = b"$-1\r\n";
const NULL_ARRAY_BYTES: &'static [u8] = b"*-1\r\n";

pub struct Composer {
    write_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>,
    end_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>,
    reset_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>,
}

impl CommandComposer for Composer {

}