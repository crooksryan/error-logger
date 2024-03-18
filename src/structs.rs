use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
    pub code: u8,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    pub code: u8,
}

