use std::fmt;

#[derive(Debug, PartialEq)]
pub struct User {
    nickname: String,
}

impl User {
    pub fn new(nickname: &str) -> Result<User, String> {
        Ok(User {
            nickname: nickname.to_string(),
        })
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "User: {}", self.nickname)
    }
}
