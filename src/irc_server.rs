use crate::user::User;

#[derive(Debug)]
pub struct IRCServer {
    name: String,
    users: Vec<User>,
    // clients: Map<ClientIdentifier, Server>,
}

impl IRCServer {
    pub fn new(name: &str) -> Result<IRCServer, String> {
        match name {
            server_name if server_name.len() > 63 => Err(format!(
                "Error creating Server. Name needs to be 63 characters or fewer, \
                        got: \"{}\".",
                server_name
            )),
            server_name => Ok(IRCServer {
                name: server_name.to_string(),
                users: vec![],
            }),
        }
    }

    pub fn users(&self) -> Vec<&User> {
        self.users.iter().collect::<Vec<&User>>()
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user)
    }

    pub fn pretty_print(&self) -> String {
        format!(
            "IRC Server: {}\r\n \
                Users: {:#?}",
            self.name, self.users
        )
    }
}
