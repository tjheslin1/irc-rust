use irc_rust::user::User;

#[test]
fn test_user_empty_name() {
    let result = User::new("").expect_err("Expected User creation to fail!");

    assert_eq!("User's nickname cannot be empty", result);
}
