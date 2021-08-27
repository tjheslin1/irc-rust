use std::io;
use std::io::{Read, Write};
use std::net;
use std::sync::Arc;

// https://github.com/nwtgck/hyper-rustls-example/blob/master/src/main.rs
fn main() {
    match start() {
        Ok(_) => println!("Running"),
        Err(err_msg) => panic!("{}", err_msg),
    }
}

fn start() -> io::Result<()> {
    let rc_config = Arc::new(rustls::ServerConfig::new(rustls::NoClientAuth::new()));
    // rc_config
    // .root_store
    // .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let mut server_session = rustls::ServerSession::new(&rc_config);

    let tcp_listener = net::TcpListener::bind("127.0.0.1:8084")?;

    for tcp_stream in tcp_listener.incoming() {
        handle_client(&mut server_session, &mut tcp_stream?)?
    }

    Ok(())
}

fn handle_client(
    session: &mut rustls::ServerSession,
    tcp_stream: &mut net::TcpStream,
) -> io::Result<()> {
    let mut tls_stream = rustls::Stream::new(session, tcp_stream);

    let mut buffer = String::new();
    tls_stream.read_to_string(&mut buffer)?;

    println!("Received message: {}", buffer);

    tls_stream.write_all(buffer.as_bytes())
}
