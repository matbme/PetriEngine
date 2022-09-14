use std::collections::HashMap;
use std::rc::Rc;

use super::{Connection, PetriNet, Place, Transition};

pub struct Simulation {
    net: PetriNet,
    incoming_connections: HashMap<Rc<Transition>, Vec<(Rc<Place>, Rc<Connection>)>>,
    outgoing_connections: HashMap<Rc<Transition>, Vec<(Rc<Place>, Rc<Connection>)>>
}

impl Simulation {
    pub fn new(net: PetriNet) -> Self {
        let mut simul = Self {
            net,
            incoming_connections: HashMap::new(),
            outgoing_connections: HashMap::new()
        };

        simul.scan_connections();

        simul
    }

    /// Maps incoming and outpoing places for all transitions
    fn scan_connections(&mut self) {
        // Initialize keys
        for transition in self.net.transitions().iter() {
            self.incoming_connections.insert(transition.clone(), vec![]);
            self.outgoing_connections.insert(transition.clone(), vec![]);
        }

        for connection in self.net.connections().iter() {
            match connection.input_from() {
                super::InputFrom::PLACE => {
                    if let Some(places) = self.incoming_connections.get_mut(&connection.transition()) {
                        places.push((connection.place().clone(), connection.clone()));
                    }
                },
                super::InputFrom::TRANSITION => {
                    if let Some(places) = self.outgoing_connections.get_mut(&connection.transition()) {
                        places.push((connection.place().clone(), connection.clone()));
                    }
                },
            }
        }
    }

    fn place_matches_requirement(&self, place: &Place, connection: &Connection) -> bool {
        let tokens = place.tokens();

        match connection.connection_type() {
            super::ConnectionType::NORMAL => {
                connection.weight() <= tokens
            },
            super::ConnectionType::INHIBITOR => {
                connection.weight() > tokens
            },
            super::ConnectionType::RESET => {
                connection.weight() <= tokens
            },
        }
    }

    fn transition_enabled(&self, transition: &Transition) -> bool {
        if let Some(vals) = self.incoming_connections.get(transition) {
            for (place, connection) in vals.iter() {
                if !self.place_matches_requirement(place.as_ref(), connection.as_ref()) {
                    return false;
                }
            }
        }

        true
    }

    pub fn start(&self) {
        let mut some_transition_enabled = true;

        while some_transition_enabled {
            // Scan transitions
            some_transition_enabled = false;
            for transition in self.net.transitions() {
                // Check if enabled
                let enabled = self.transition_enabled(transition.as_ref());
            }
        }
    }
}
