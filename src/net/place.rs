use uuid::Uuid;

pub struct Place {
    id: Uuid,
    name: String
}

impl Place {
    /// Creates a new place with given name
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into()
        }
    }
}
