use uuid::Uuid;
use crate::models::message::Message;
#[derive(Debug)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub messages: Vec<Message>,
}

impl Room {
    fn new(id: Uuid, name: &str) -> Room {
        Room { id, name: name.to_string(), messages: Vec::new() }
    }

    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.sort_messages();
    }

    fn sort_messages(&mut self) {
        self.messages.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    }
}