use std::io;
use std::io::{Read, Write};
use std::net;
use std::sync::Arc;
use std::sync::Mutex;

use irc_rust::threadpool::ThreadPool;

// https://github.com/nwtgck/hyper-rustls-example/blob/master/src/main.rs
// https://stackoverflow.com/questions/60357636/how-do-i-make-a-tls-connection-using-the-rustls-library

// certs:
//https://stackoverflow.com/questions/61169422/implementing-https-server-using-rustls-with-hyper-in-rust

fn main() {
    let rc_config = Arc::new(rustls::ServerConfig::new(rustls::NoClientAuth::new()));
    // rc_config
    //     // .root_store
    //     .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let server_session = Arc::new(Mutex::new(rustls::ServerSession::new(&rc_config)));

    let tcp_listener = net::TcpListener::bind("127.0.0.1:8084").unwrap(); // 6667
    let thread_pool = ThreadPool::new(20).unwrap();

    tcp_listener.incoming().for_each(|stream| match stream {
        Ok(tcp_stream) => {
            let arc_clone = Arc::clone(&server_session);

            thread_pool.execute(|| handle_client(arc_clone, tcp_stream).unwrap())
        }
        Err(e) => panic!("Connection failed: {}", e),
    })
}

fn handle_client(
    rc_session: Arc<Mutex<rustls::ServerSession>>,
    mut tcp_stream: net::TcpStream,
) -> io::Result<()> {
    let mut locked_session = rc_session.lock().expect("Error obtaining lock.");

    let mut tls_stream = rustls::Stream::new(&mut *locked_session, &mut tcp_stream);

    let mut buffer = String::new();
    if let Err(x) = tls_stream.read_to_string(&mut buffer) {
        println!("Err on read_to_string: {}\n{}", x, buffer);

        return Err(x);
    }

    tls_stream.write_all("Bonjourno!".as_bytes()).unwrap();
    // tls_stream.write(b"GET / HTTP/1.1\r\nConnection: close\r\n\r\n");

    Ok(())
}
