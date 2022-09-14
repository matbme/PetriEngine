use super::Connectable;

use uuid::Uuid;

#[derive(Clone, Debug, Eq, Hash)]
pub struct Transition {
    id: Uuid,
    name: String
}

impl Connectable for Transition { }

impl PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Transition {
    /// Creates a new Transition with given name
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
