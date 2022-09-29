use uuid::Uuid;

use std::rc::Rc;

use super::Connectable;
use crate::ui::UITable;

#[derive(Clone, Debug, Eq, Hash)]
pub struct Transition {
    id: Uuid,
    name: String,
}

impl Connectable for Transition {
    fn connection_title(&self) -> &str {
        self.name()
    }
}

impl PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl UITable for Vec<Rc<Transition>> {
    fn header(&self) -> Vec<&str> {
        vec!["Name"]
    }

    fn rows(&self) -> Vec<Vec<String>> {
        let mut rows = vec![];

        for elem in self {
            rows.push(vec![elem.name().to_string()])
        }

        rows
    }
}

impl Transition {
    /// Creates a new Transition with given name
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
