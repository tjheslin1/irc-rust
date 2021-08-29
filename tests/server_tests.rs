use irc_rust::server::Server;

#[test]
fn test_server_creation_name_too_long() {
    let server_creation_error =
        Server::new("Areallylongnameforaserverthatjustoverexceedsthesixtyhreecharacterlimit")
            .expect_err("Expected Err because Server name is too long.");

    let expected_message = "Error creating Server. Name needs to be 63 characters or fewer, \
        got: \"Areallylongnameforaserverthatjustoverexceedsthesixtyhreecharacterlimit\".";

    assert_eq!(server_creation_error.message, expected_message);
}
