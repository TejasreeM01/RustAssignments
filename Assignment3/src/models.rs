use crate::errors::WalletError;


pub struct User {
    pub username: String,
    password: String,
    pub balance: f64,
    pub history: Vec<String>,   
}

impl User {
    //create a new user 

    pub fn new(username: &str, password: &str) ->Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            balance: 0.0,
            history: vec![],
        }
    }

    pub fn verify_password(&self, input: &str) -> Result<(), WalletError> {
    if self.password == input {
        Ok(())

    }else {
        Err(WalletError::IncorrectPassword)
    }
}
}

