use crate::net::*;

pub enum ConnectionType {
    NORMAL, INHIBITOR, RESET
}

pub struct Connection<'a> {
    place: &'a Place,
    transition: &'a Transition,
    weight: i32,
    is_input: bool,
    con_type: ConnectionType
}

impl<'a> Connection<'a> {
    /// Creates a new connection
    pub fn new(place: &'a Place, transition: &'a Transition, weight: i32, is_input: bool, con_type: ConnectionType) -> Self {
        Self {
            place, transition, weight, is_input, con_type
        }
    }
}
