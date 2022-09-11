use uuid::Uuid;

#[derive(Clone, Debug)]
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

    pub fn name(&self) -> &str {
        &self.name
    }
}
