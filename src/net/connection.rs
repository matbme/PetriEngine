use std::rc::Rc;

use crate::net::*;
use super::Connectable;

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionType {
    NORMAL, INHIBITOR, RESET
}

#[derive(Clone, Debug)]
pub enum InputFrom {
    PLACE, TRANSITION
}

#[derive(Debug)]
pub struct Connection {
    place: Rc<Place>,
    transition: Rc<Transition>,
    weight: i32,
    input_from: InputFrom,
    con_type: ConnectionType,
}

impl Connection {
    /// Creates a new connection
    pub fn new(place: Rc<Place>, transition: Rc<Transition>, weight: i32, input_from: InputFrom, con_type: ConnectionType) -> Self {
        Self {
            place, transition, weight, input_from, con_type
        }
    }

    pub fn weight(&self) -> &i32 {
        &self.weight
    }

    pub fn connection_type(&self) -> ConnectionType {
        self.con_type.clone()
    }

    pub fn place(&self) -> Rc<Place> {
        self.place.clone()
    }

    pub fn transition(&self) -> Rc<Transition> {
        self.transition.clone()
    }

    pub fn input_from(&self) -> InputFrom {
        self.input_from.clone()
    }

    /// Returns the connection's input. Can be either a Place or a Transition
    pub fn input(&self) -> Rc<dyn Connectable> {
        match self.input_from {
            InputFrom::PLACE => self.place.clone(),
            InputFrom::TRANSITION => self.transition.clone(),
        }
    }

    /// Returns the connection's output. Can be either a Place or a Transition
    pub fn output(&self) -> Rc<dyn Connectable> {
        match self.input_from {
            InputFrom::PLACE => self.transition.clone(),
            InputFrom::TRANSITION => self.place.clone(),
        }
    }
}
