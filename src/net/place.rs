use uuid::Uuid;

use super::Connectable;

#[derive(Clone, Debug)]
pub struct Place {
    id: Uuid,
    name: String,
    tokens: i32,
}

impl Connectable for Place { }

impl Place {
    /// Creates a new place with given name
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            tokens: 0
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tokens(&self) -> &i32 {
        &self.tokens
    }

    /// Adds `amount` tokens to place
    pub fn add_tokens(&mut self, amount: i32) {
        self.tokens += amount;
    }

    /// Removes `amount` tokens from place
    pub fn remove_tokens(&mut self, amount: i32) {
        self.tokens -= amount;
    }
}
