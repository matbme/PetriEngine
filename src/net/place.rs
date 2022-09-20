use uuid::Uuid;

use std::cell::RefCell;
use std::rc::Rc;

use super::Connectable;
use crate::ui::UITable;

#[derive(Clone, Debug)]
pub struct TokensInner(i32);

#[derive(Clone, Debug)]
pub struct Place {
    id: Uuid,
    name: String,
    tokens: RefCell<TokensInner>,
}

impl UITable for Vec<Rc<Place>> {
    fn header(&self) -> Vec<&str> {
        vec!["Name", "Tokens"]
    }

    fn rows(&self) -> Vec<Vec<String>> {
        let mut rows = vec![];

        for elem in self {
            rows.push(vec![elem.name().to_string(), elem.tokens.borrow().0.clone().to_string()])
        }

        rows
    }
}

impl Connectable for Place {
    fn connection_title(&self) -> &str {
        self.name()
    }
}

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
