use std::os::unix::net::UnixStream;
use std::io::{BufWriter, Write};

fn main() {
    let mut stream = UnixStream::connect("/home/majortom/tmp/can.sock").unwrap();
    let mut bf = BufWriter::new(&stream);
    let mut n  = 0;
    while n < 200 {
        n+=1;
        let mut s = String::new();
        s.push_str(format!("number: {}\n", n).as_str());
        bf.write_all(s.as_bytes());
        bf.flush();
    }
    drop(bf);
}