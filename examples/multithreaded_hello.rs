extern crate fastcgi;

use std::net::TcpListener;

use std::io::Write;
use std::thread;

fn main() {
    fastcgi::run_tcp(|mut req| {
        thread::spawn(move || write!(&mut req.stdout(), "Content-Type: text/plain\n\nHello, world!"));
    }, &TcpListener::bind("127.0.0.1:8000").unwrap());
}
