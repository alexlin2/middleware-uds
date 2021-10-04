mod send_can;

use std::io::Result;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::io::{BufReader, BufRead};

fn server_driver(path: &str) -> Result<UnixListener> {
    std::fs::remove_file(path)?;
    let socket_file = path;
    let socket = Path::new(socket_file);
    UnixListener::bind(&socket)
}

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line.unwrap())
    }
}



fn main () {
    
    // to be replaced 
    let path = "/home/majortom/tmp/can.sock";

    let listener = match server_driver(path) {
        Ok(sock) => sock,
        Err(e) => {
            println!("Could not bind socket! {:?}", e);
            return
        }
    };

    println!("Server started, waiting for clients");

    for client in listener.incoming() {
            let stream = client.unwrap();
            handle_client(stream);

    }



}