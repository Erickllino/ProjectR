use serde::{Serialize,Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthResponse {
    Success,
    InvalidCredentials,
    UsernameTaken,
}
