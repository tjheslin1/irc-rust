use crate::user::User;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct IRCServer {
    name: String,
    users: Vec<User>,
    nodes: Vec<String>,
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
                nodes: vec![],
            }),
        }
    }

    pub fn users(&self) -> Vec<&User> {
        self.users.iter().collect::<Vec<&User>>()
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user)
    }

    pub fn add_node(&mut self, node: String) {
        self.nodes.push(node)
    }

    pub fn pretty_print(&self) -> String {
        format!(
            "IRC Server: {}\r\n \
                Users: {:#?}",
            self.name, self.users
        )
    }

    pub fn handle_input(rc_server: Arc<Mutex<IRCServer>>, input: String) -> Result<(), String> {
        match input.split("\r\n").collect::<Vec<&str>>().first() {
            Some(nickname) => match User::new(nickname) {
                Ok(created_user) => {
                    let mut server = rc_server.lock().expect("Error obtaining lock.");

                    Ok(server.add_user(created_user))
                }
                Err(e) => return Err(e),
            },
            None => return Err("Invalid message format!".to_string()),
        }
    }
}
