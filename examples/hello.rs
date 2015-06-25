extern crate fastcgi;

use std::io::Write;

fn main() {
    fastcgi::run(|mut req| {
        write!(&mut req.stdout(), "Content-Type: text/plain\n\nHello, world!")
        .unwrap();
    });
}
