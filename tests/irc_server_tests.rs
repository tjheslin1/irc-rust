use irc_rust::irc_server::IRCServer;
use irc_rust::user::User;
use std::sync::{Arc, Mutex};

#[test]
fn test_server_creation_name_too_long() {
    let server_creation_error =
        IRCServer::new("Areallylongnameforaserverthatjustoverexceedsthesixtyhreecharacterlimit")
            .expect_err("Expected Err because Server name is too long.");

    let expected_message = "Error creating Server. Name needs to be 63 characters or fewer, \
        got: \"Areallylongnameforaserverthatjustoverexceedsthesixtyhreecharacterlimit\".";

    assert_eq!(server_creation_error, expected_message);
}

#[test]
fn test_server_add_user() {
    let rc_server = Arc::new(Mutex::new(
        IRCServer::new("brand-new-server").expect("Exepcted newly created server!"),
    ));

    let new_user_input = "user: tommy";
    let expected_nickname = "tommy";

    IRCServer::handle_input(Arc::clone(&rc_server), new_user_input.to_string())
        .expect("Expected Ok when adding user.");

    let server = rc_server.lock().expect("Error occurred obtaining lock.");

    let expected_users = vec![User::new(expected_nickname).expect("Exepcted newly created user!")];

    expected_users
        .iter()
        .zip(server.users())
        .for_each(|(e, a)| assert_eq!(e, a));
}

#[test]
fn test_server_discovery() {
    let rc_server = Arc::new(Mutex::new(
        IRCServer::new("brand-new-server").expect("Exepcted newly created server!"),
    ));

    let discover_input = "discover";

    IRCServer::handle_input(Arc::clone(&rc_server), discover_input.to_string())
        .expect("Expected Ok when calling server discovery.");

    let server = rc_server.lock().expect("Error occurred obtaining lock.");
}
