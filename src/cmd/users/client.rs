pub struct UsersClient {
    server_address: String
}

impl UsersClient {
    pub fn default(address: String) -> Self {
      UsersClient{
        server_address: address,
      } 
    }
}
