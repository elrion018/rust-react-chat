use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    fn new(id: Uuid, name: &str) -> User {
        User { id, name: name.to_string() }
    }
}