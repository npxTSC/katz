use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatChannel {
    pub UUID: u32,
    pub messagez: Vec<String>,
}

impl ChatChannel {
    pub fn new() -> Self {
        ChatChannel {
            UUID: 0,
            messagez: Vec::new(),
        }
    }
}
