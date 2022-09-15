use uuid::Uuid;

use std::cell::RefCell;

use super::Connectable;

#[derive(Clone, Debug)]
pub struct TokensInner(i32);

#[derive(Clone, Debug)]
pub struct Place {
    id: Uuid,
    name: String,
    tokens: RefCell<TokensInner>,
}

impl Connectable for Place { }

impl Place {
    /// Creates a new place with given name
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            tokens: RefCell::new(TokensInner(0))
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tokens(&self) -> i32 {
        self.tokens.borrow().0.clone()
    }

    /// Adds `amount` tokens to place
    pub fn add_tokens(&self, amount: i32) {
        self.tokens.borrow_mut().0 += amount;
    }

    /// Removes `amount` tokens from place
    pub fn remove_tokens(&self, amount: i32) {
        self.tokens.borrow_mut().0 -= amount;
    }

    /// Removes all tokens from place, usually called by a Reset connection
    pub fn clear_tokens(&self) {
        self.tokens.borrow_mut().0 = 0;
    }
}
