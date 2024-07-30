pub struct Composer {
    write_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>,
    end_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>,
    reset_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>,
}

impl CommandComposer for Composer {
    
}