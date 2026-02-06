use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct PasswordEntry {
    pub service: String,
    pub username: String,
    pub password: String,
}





/*
    data structures
*/
