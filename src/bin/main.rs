use rustls::Connection;
use std::env;
use std::fs;
use std::io;
use std::io::{BufReader, Read, Write};
use std::net;
use std::sync::Arc;

use irc_rust::threadpool::ThreadPool;

// https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsserver.rs

// https://github.com/nwtgck/hyper-rustls-example/blob/master/src/main.rs
// https://stackoverflow.com/questions/60357636/how-do-i-make-a-tls-connection-using-the-rustls-library

// certs:
// https://stackoverflow.com/questions/61169422/implementing-https-server-using-rustls-with-hyper-in-rust
// https://github.com/gpg/gnupg/blob/master/doc/howto-create-a-server-cert.texi
// https://github.com/rustls/rustls/issues/397

const CLI_ERROR: &str = "Expected two (2) arguments for certs path and private key path";

fn main() {
    let (certs, private_key) = match env::args().collect::<Vec<String>>().as_slice() {
        [_, certs_path, private_key_path] => {
            println!("{}\n{}", certs_path, private_key_path);

            (load_certs(certs_path), load_private_key(private_key_path))
        }
        args => panic!("{}, got {}", CLI_ERROR, args.len() - 1),
    };

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

    let tcp_listener = net::TcpListener::bind("192.168.0.110:8084").unwrap(); // 6667
    let thread_pool = ThreadPool::new(20).unwrap();

    tcp_listener.incoming().for_each(|stream| match stream {
        Ok(tcp_stream) => {
            let tls_config = Arc::clone(&rc_tls_config); // TODO: inline

            thread_pool.execute(|| handle_connection(tls_config, tcp_stream).unwrap())
        }
        Err(e) => println!("Connection failed: {}", e),
    })
}

fn handle_connection(
    rc_tls_config: Arc<rustls::ServerConfig>,
    mut tcp_stream: net::TcpStream,
) -> io::Result<()> {
    println!("Connection started!");

    let mut server_connection = rustls::ServerConnection::new(rc_tls_config).unwrap();

    loop {
        if server_connection.wants_read() {
            match server_connection.read_tls(&mut tcp_stream) {
                Err(e) => {
                    println!("Err occurred: {:?}", e);

                    if let io::ErrorKind::WouldBlock = e.kind() {
                        println!("WouldBlock found!!");

                        ()
                    }

                    println!("Error on read! {:?}", e);
                    return Err(e);
                }
                Ok(0) => {
                    println!("EOF ?");
                    return Ok(());
                }
                Ok(_) => {
                    println!("All is Ok");

                    ()
                }
            }

            match server_connection.process_new_packets() {
                Err(e) => {
                    println!("Cannot process packet!: {:?}", e);

                    if let Err(write_err) = server_connection.write_tls(&mut tcp_stream) {
                        println!("write failed {:?}", write_err);
                        return Err(write_err);
                    }
                }
                Ok(io) => {
                    println!("{:?}", io);

                    if io.peer_has_closed() {
                        println!("Peer closed!");

                        return Ok(());
                    } else if io.plaintext_bytes_to_read() > 0 {
                        let mut buf = Vec::new();
                        buf.resize(io.plaintext_bytes_to_read(), 0u8);

                        server_connection.reader().read(&mut buf).unwrap();

                        let request = String::from_utf8_lossy(&buf);
                        let http_response =
                            format!("HTTP/1.0 200 OK\r\nConnection: close\r\n\r\n{}", request);

                        println!("{}", http_response);

                        // echo
                        server_connection
                            .writer()
                            .write_all(http_response.as_bytes())
                            .unwrap();
                        server_connection.send_close_notify()
                    }
                }
            }
        }

        if server_connection.wants_write() {
            if let Err(write_err) = server_connection.write_tls(&mut tcp_stream) {
                println!("write failed {:?}", write_err);
                return Err(write_err);
            }
        }
    }
}

// from: https://github.com/rustls/rustls/blob/main/rustls-mio/examples/tlsserver.rs

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
