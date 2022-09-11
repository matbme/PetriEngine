use std::rc::Rc;

use crate::net::*;

#[derive(Debug)]
pub enum ConnectionType {
    NORMAL, INHIBITOR, RESET
}

#[derive(Debug)]
pub enum InputFrom {
    PLACE, TRANSITION
}

#[derive(Debug)]
pub struct Connection {
    place: Rc<Place>,
    transition: Rc<Transition>,
    weight: i32,
    input_from: InputFrom,
    con_type: ConnectionType
}

impl Connection {
    /// Creates a new connection
    pub fn new(place: Rc<Place>, transition: Rc<Transition>, weight: i32, input_from: InputFrom, con_type: ConnectionType) -> Self {
        Self {
            place, transition, weight, input_from, con_type
        }
    }
}
