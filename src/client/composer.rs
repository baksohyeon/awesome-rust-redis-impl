// use bytes::BufMut;

// pub struct ComposerImpl {
    
// }

// pub trait Composer {
//     fn end(&self, buf: BufMut) -> Result<(), Box<dyn Error>>;
//     fn write_fn(&self, buf: BufMut) -> Result<(), Box<dyn Error>>;
//     fn end_fn(&self, buf: BufMut) -> Result<(), Box<dyn Error>>;
//     fn reset_fn(&self, buf: BufMut) -> Result<(), Box<dyn Error>>;
// }


// impl Composer for ComposerImpl {
//     fn new(write_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>,
//            end_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>,
//            reset_fn: fn(buf: BufMut) -> Result<(), Box<dyn Error>>) -> Self {
//         Self { write_fn, end_fn, reset_fn }
//     }

//     fn end(&self, buf: BufMut) -> Result<(), Box<dyn Error>> {
//         self.end_fn(buf)
//     }
// }