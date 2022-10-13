use uuid::Uuid;

use std::rc::Rc;

use super::{Connectable, ConnectionMap};
use crate::ui::UITable;

// Callbacks for Transition receive a reference for the `Transition`, as well as references to the
// `HashMap` expressing the net graph.
type TransitionCallback = Box<dyn Fn(&Transition, &ConnectionMap, &ConnectionMap)>;

pub struct Transition {
    id: Uuid,
    name: String,

    callback: Option<TransitionCallback>,
}

impl std::fmt::Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transition")
            .field("id", &self.id)
            .field("name", &self.name)
            .field(
                "callback",
                if self.callback.is_some() {
                    &"Some Fn()"
                } else {
                    &"None"
                },
            )
            .finish()
    }
}

impl std::hash::Hash for Transition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}

impl PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Transition {}

impl Connectable for Transition {
    fn connection_title(&self) -> &str {
        self.name()
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
            callback: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn callback(&self) -> Option<&TransitionCallback> {
        self.callback.as_ref()
    }

    pub fn add_callback(&mut self, callback: TransitionCallback) -> Option<TransitionCallback> {
        self.callback.replace(callback)
    }
}
