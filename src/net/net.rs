use crate::net::*;

pub struct PetriNet<'a> {
    places: Vec<Place>,
    transitions: Vec<Transition>,
    connections: Vec<Connection<'a>>,
}

impl<'a> PetriNet<'a> {
    /// Creates an empty Petri Network
    pub fn new() -> Self {
        Self {
            places: vec![],
            transitions: vec![],
            connections: vec![]
        }
    }
}
