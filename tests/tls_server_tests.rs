use std::sync::{Arc, Mutex};

use irc_rust::irc_server::IRCServer;
use irc_rust::tls_server;
use irc_rust::user::User;

#[test]
fn test_server_handle_empty_request() {
    let rc_server = Arc::new(Mutex::new(
        IRCServer::new("test-server").expect("Expected IRCServer to be created successfully!"),
    ));

    let buf = b"";

    tls_server::handle_request(rc_server, buf)
        .expect_err("Expected request to Err on empty request!");
}

#[test]
fn test_server_handle_empty_get_request() {
    let buf = b"GET / HTTP/1.1
Host: ircrust.com:8084
User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
Accept-Language: en-GB,en;q=0.5
Accept-Encoding: gzip, deflate, br
DNT: 1
Connection: keep-alive
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: none
Sec-Fetch-User: ?1
Sec-GPC: 1
Cache-Control: max-age=0

";

    let rc_server = Arc::new(Mutex::new(
        IRCServer::new("test-server").expect("Expected IRCServer to be created successfully!"),
    ));

    tls_server::handle_request(Arc::clone(&rc_server), buf)
        .expect_err("Expected request to error due to no body!");

    let irc_server = rc_server.lock().expect("Error obtaining lock.");
    let users = irc_server.users();

    let expected_users: Vec<&User> = vec![];

    assert_eq!(expected_users, users);
}

#[test]
fn test_server_handle_post_request() {
    let buf = b"POST / HTTP/1.1
Host: ircrust.com:8084
User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
Accept-Language: en-GB,en;q=0.5
Accept-Encoding: gzip, deflate, br
Content-Type: application/x-www-form-urlencoded
Content-Length: 11
DNT: 1
Connection: keep-alive
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: cross-site
Sec-Fetch-User: ?1
Sec-GPC: 1

data=mydata
";

    let rc_server = Arc::new(Mutex::new(
        IRCServer::new("test-server").expect("Expected IRCServer to be created successfully!"),
    ));

    tls_server::handle_request(Arc::clone(&rc_server), buf)
        .expect("Expected request to be handled!");

    let irc_server = rc_server.lock().expect("Error obtaining lock.");
    let users = irc_server.users();

    let expected_user = User::new("mydata").expect("Expeced User creation to be successful!");

    assert_eq!(vec![&expected_user], users);
}
