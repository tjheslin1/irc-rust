use irc_rust::server::Server;
use irc_rust::user::User;

#[test]
fn test_server_creation_name_too_long() {
    let server_creation_error =
        Server::new("Areallylongnameforaserverthatjustoverexceedsthesixtyhreecharacterlimit")
            .expect_err("Expected Err because Server name is too long.");

    let expected_message = "Error creating Server. Name needs to be 63 characters or fewer, \
        got: \"Areallylongnameforaserverthatjustoverexceedsthesixtyhreecharacterlimit\".";

    assert_eq!(server_creation_error, expected_message);
}

#[test]
fn test_server_add_user() {
    let mut server = Server::new("brand-new-server").expect("Exepcted newly created server!");
    let user_name = "tommy";

    server.add_user(User::new(user_name).expect("Exepcted newly created user!"));

    let expected_users = vec![User::new(user_name).expect("Exepcted newly created user!")];

    expected_users
        .iter()
        .zip(server.users())
        .for_each(|(e, a)| assert_eq!(e, a));
}
