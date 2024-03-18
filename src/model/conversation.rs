use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Messages>
}
impl Conversation {
    pub fn new() -> Conversation{
        Conversation {
            messages: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Messages {
    pub user: bool,
    pub text: String,
}