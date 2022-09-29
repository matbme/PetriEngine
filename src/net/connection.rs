use std::rc::Rc;

use super::Connectable;
use crate::net::*;
use crate::ui::UITable;

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionType {
    NORMAL,
    INHIBITOR,
    RESET,
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConnectionType::NORMAL => "Normal",
                ConnectionType::INHIBITOR => "Inhibitor",
                ConnectionType::RESET => "Reset",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub enum InputFrom {
    PLACE,
    TRANSITION,
}

#[derive(Debug)]
pub struct Connection {
    place: Rc<Place>,
    transition: Rc<Transition>,
    weight: i32,
    input_from: InputFrom,
    con_type: ConnectionType,
}

impl UITable for Vec<Rc<Connection>> {
    fn header(&self) -> Vec<&str> {
        vec!["Input", "Output", "Weight", "Type"]
    }

    fn rows(&self) -> Vec<Vec<String>> {
        let mut rows = vec![];

        for elem in self {
            rows.push(vec![
                elem.input().connection_title().to_string(),
                elem.output().connection_title().to_string(),
                elem.weight().to_string(),
                elem.connection_type().to_string(),
            ])
        }

        rows
    }
}

impl Connection {
    /// Creates a new connection
    pub fn new(
        place: Rc<Place>,
        transition: Rc<Transition>,
        weight: i32,
        input_from: InputFrom,
        con_type: ConnectionType,
    ) -> Self {
        Self {
            place,
            transition,
            weight,
            input_from,
            con_type,
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
