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

    pub fn get(&self, index: usize) -> &Password {
        &self.passwords[index]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Password {
    pub id: String,
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Password {
    pub fn new(name: String, url: String, username: String, password: String) -> Self {
        Password {
            id: uuid::Uuid::new_v4().to_string(),
            name: name,
            url: url,
            username: username,
            password: password,
        }
    }
}
