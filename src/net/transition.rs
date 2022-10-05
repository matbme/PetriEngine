use derivative::Derivative;
use uuid::Uuid;

use std::rc::Rc;

use super::{Connectable, ConnectionMap};
use crate::ui::UITable;

// Callbacks for Transition receive a reference for the `Transition`, as well as references to the
// `HashMap` expressing the net graph.
type TransitionCallback = Box<dyn Fn(&Transition, &ConnectionMap, &ConnectionMap)>;

#[derive(Derivative)]
#[derivative(Debug, Eq, Hash)]
pub struct Transition {
    id: Uuid,
    name: String,

    #[derivative(Hash = "ignore")]
    #[derivative(Debug = "ignore")]
    callback: Option<TransitionCallback>,
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
