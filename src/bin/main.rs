use std::fs;
use std::io;
use std::io::{BufReader, Read, Write};
use std::net;
use std::sync::Arc;
use std::sync::Mutex;

use irc_rust::threadpool::ThreadPool;
use std::env;

// https://github.com/nwtgck/hyper-rustls-example/blob/master/src/main.rs
// https://stackoverflow.com/questions/60357636/how-do-i-make-a-tls-connection-using-the-rustls-library

// certs:
// https://stackoverflow.com/questions/61169422/implementing-https-server-using-rustls-with-hyper-in-rust
// https://github.com/gpg/gnupg/blob/master/doc/howto-create-a-server-cert.texi

const CLI_ERROR: &str = "Expected two (2) arguments for certs path and private key path";

fn main() {
    let (certs, private_key) = match env::args().collect::<Vec<String>>().as_slice() {
        [_, certs_path, private_key_path] => {
            println!("{}\n{}", certs_path, private_key_path);

            (load_certs(certs_path), load_private_key(private_key_path))
        }
        args => panic!("{}, got {}", CLI_ERROR, args.len() - 1),
    };

    // let certs = vec![];
    // let private_key = rustls::PrivateKey(vec![]);

    let rc_tls_config = Arc::new(
        rustls::ServerConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .expect("bad certificate/key"),
    );

    let server_connection = Arc::new(Mutex::new(
        rustls::ServerConnection::new(rc_tls_config).unwrap(),
    ));

    let tcp_listener = net::TcpListener::bind("127.0.0.1:8084").unwrap(); // 6667
    let thread_pool = ThreadPool::new(20).unwrap();

    tcp_listener.incoming().for_each(|stream| match stream {
        Ok(tcp_stream) => {
            let arc_clone = Arc::clone(&server_connection);

            thread_pool.execute(|| handle_connection(arc_clone, tcp_stream).unwrap())
        }
        Err(e) => panic!("Connection failed: {}", e),
    })
}

fn handle_connection(
    rc_connection: Arc<Mutex<rustls::ServerConnection>>,
    mut tcp_stream: net::TcpStream,
) -> io::Result<()> {
    let mut connection = rc_connection.lock().expect("Error obtaining lock.");

    let mut tls_stream = rustls::Stream::new(&mut *connection, &mut tcp_stream);

    let mut buffer = String::new();
    if let Err(x) = tls_stream.read_to_string(&mut buffer) {
        println!("Err on read_to_string: {}\n{}", x, buffer);

        return Err(x);
    }

    tls_stream.write_all("Bonjourno!".as_bytes()).unwrap();
    // tls_stream.write(b"GET / HTTP/1.1\r\nConnection: close\r\n\r\n");

    Ok(())
}

// https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsserver.rs

fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}
