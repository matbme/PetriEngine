use uuid::Uuid;

pub struct Transition {
    id: Uuid,
    name: String
}

impl Transition {
    /// Creates a new Transition with given name
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into()
        }
    }
}
