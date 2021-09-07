use irc_rust::tls_server;

fn main() {
    tls_server::start("192.168.0.110:8084")
}
