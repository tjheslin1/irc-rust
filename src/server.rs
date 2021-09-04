#[derive(Debug)]
pub struct Server {
    name: String,
    // clients: Map<ClientIdentifier, Server>,
}
#[derive(Debug)]
pub struct ServerCreationError {
    pub message: String,
}

impl Server {
    pub fn new(name: &str) -> Result<Server, ServerCreationError> {
        match name {
            server_name if server_name.len() > 63 => Err(ServerCreationError {
                message: format!(
                    "Error creating Server. Name needs to be 63 characters or fewer, \
                        got: \"{}\".",
                    server_name
                ),
            }),
            server_name => Ok(Server {
                name: server_name.to_string(),
            }),
        }
    }

    pub fn pretty_print(&self) -> String {
        format!("IRC Server: {}", self.name)
    }
}
