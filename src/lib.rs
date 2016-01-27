use std::io::{self, Read, Write};

const BUF_SIZE: usize = 8192;

pub enum Mode {
    Encode,
    Decode,
}

pub enum Method {
    Base64
}

pub fn pipe<R: Read, W: Write, F>(from: &mut R, to: &mut W, transform: F) -> io::Result<()>
where F: Fn(&[u8], bool) -> (Vec<u8>, usize) {
    let mut buf_in = [0; BUF_SIZE];
    let mut extra_left = 0;
    
    loop {
        let read = try!(from.read(&mut buf_in[extra_left..]));
        let size = extra_left + read;

        let (buf_out, extra_right) = transform(&buf_in[..size], read == 0);
        try!(to.write_all(&buf_out));
        if read == 0 {
            assert_eq!(extra_right, 0);
            break;
        }

        for i in 0..extra_right {
            buf_in[i] = buf_in[size - extra_right + i];
        }
        extra_left = extra_right;
    }
    Ok(())
}

pub mod plugins;
