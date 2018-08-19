extern crate fastcgi;

use std::collections::HashMap;
use std::env;
use std::fs::remove_file;
use std::io::Write;
use std::os::unix::io::IntoRawFd;
use std::os::unix::net::UnixListener;
use std::panic;
use std::process::Command;
use std::str;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn it_works() -> thread::Result<()> {
    let mut path = env::current_dir().unwrap();
    path.push("test.sock");

    let result = panic::catch_unwind(|| {
        let stream = UnixListener::bind(&path).unwrap();
        let fd = stream.into_raw_fd();

        let (tx, rx) = channel();
        let tx_mutex = Arc::new(Mutex::new(tx));

        thread::spawn(move || {
            fastcgi::run_raw(
                move |mut req| {
                    write!(
                        &mut req.stdout(),
                        "Content-Type: text/plain\n\nHello, world!"
                    ).expect("write");

                    let tx = tx_mutex.lock().unwrap().clone();
                    let params: HashMap<String, String> = req.params().collect();
                    tx.send(params).expect("error sending req to test thread");
                },
                fd,
            );
        });

        let env_path = env::var("PATH").expect("PATH env var");

        let env: HashMap<String, String> = [
            ("PATH", env_path.as_str()),
            ("REQUEST_METHOD", "GET"),
            ("REMOTE_ADDR", "127.0.0.1"),
            ("SERVER_SOFTWARE", "Apache"),
            ("SCRIPT_FILENAME", "/var/www/index.php"),
            ("SCRIPT_NAME", "/index.php"),
            ("QUERY_STRING", ""),
        ].iter()
            .cloned()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        let output = Command::new("cgi-fcgi")
            .arg("-bind")
            .arg("-connect")
            .arg(&path)
            .env_clear()
            .envs(&env)
            .output()
            .expect("spawn cgi-fcgi");

        assert_eq!(Some(0), output.status.code());

        let expected = "Content-Type: text/plain\n\nHello, world!";
        assert_eq!(Ok(expected), str::from_utf8(&output.stdout));

        let one_second = Duration::new(1, 0);
        assert_eq!(Ok(env), rx.recv_timeout(one_second));
    });

    remove_file(path).unwrap_or(());

    result
}
