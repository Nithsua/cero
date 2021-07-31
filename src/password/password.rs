use serde::{Deserialize, Serialize};

pub struct Passwords {
    passwords: Vec<Password>,
}

impl Passwords {
    pub fn new(passwords: Vec<Password>) -> Self {
        Passwords {
            passwords: passwords,
        }
    }

    pub fn get_passwords(&self) -> &Vec<Password> {
        &self.passwords
    }

    pub fn length_of_passwords(&self) -> usize {
        self.passwords.len()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    pub id: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Password {
    pub fn new(url: String, username: String, password: String) -> Self {
        Password {
            id: "  ".to_string(),
            url: url,
            username: username,
            password: password,
        }
    }
}
