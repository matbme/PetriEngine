use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::{Connection, PetriNet, Place, Transition};
use crate::ui::UITable;

pub struct Simulation {
    net: PetriNet,
    incoming_connections: HashMap<Rc<Transition>, Vec<(Rc<Place>, Rc<Connection>)>>,
    outgoing_connections: HashMap<Rc<Transition>, Vec<(Rc<Place>, Rc<Connection>)>>,

    // Store a log of each step for printing
    ui_rows: RefCell<Vec<Vec<String>>>,
}

impl UITable for Simulation {
    fn header(&self) -> Vec<&str> {
        let mut cols = vec!["Cycle"];

        for place in self.net.places().iter() {
            cols.push(place.name());
        }

        for transition in self.net.transitions().iter() {
            cols.push(transition.name());
        }

        cols
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.ui_rows.borrow().clone()
    }
}

impl Simulation {
    pub fn new(net: PetriNet) -> Self {
        let mut simul = Self {
            net,
            incoming_connections: HashMap::new(),
            outgoing_connections: HashMap::new(),
            ui_rows: RefCell::new(vec![])
        };

        simul.scan_connections();

        simul
    }

    pub fn net(&self) -> &PetriNet {
        &self.net
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
                connection.weight() <= &tokens
            },
            super::ConnectionType::INHIBITOR => {
                connection.weight() > &tokens
            },
            super::ConnectionType::RESET => {
                true
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

    /// Consume tokens from incoming places
    fn consume_tokens(&self, transition: &Transition) {
        if let Some(vals) = self.incoming_connections.get(transition) {
            for (place, connection) in vals.iter() {
                if connection.connection_type() == super::ConnectionType::RESET {
                    place.clear_tokens();
                } else {
                    place.remove_tokens(connection.weight().clone());
                }
            }
        }
    }

    /// Propagate tokens to outgoing places
    fn propagate_tokens(&self, transition: &Transition) {
        if let Some(vals) = self.outgoing_connections.get(transition) {
            for (place, connection) in vals.iter() {
                place.add_tokens(connection.weight().clone());
            }
        }
    }

    pub fn run(&self) {
        let mut some_transition_enabled = true;
        let mut cycle_count = -1;

        while some_transition_enabled {
            cycle_count += 1;
            some_transition_enabled = false;

            // Add current places' marks to table
            let mut places_marks = vec![];
            for places in self.net.places() {
                places_marks.push(places.tokens().to_string());
            }

            // Scan transitions and check if enabled
            let mut transition_states = vec![];
            for transition in self.net.transitions() {
                transition_states.push(self.transition_enabled(transition.as_ref()));
            }

            for (i, transition) in self.net.transitions().iter().enumerate() {
                if transition_states[i] {
                    some_transition_enabled = true;

                    self.consume_tokens(transition.as_ref());
                    self.propagate_tokens(transition.as_ref());
                }
            }

            // Insert cycle status into table
            let mut table_row = vec![cycle_count.to_string()];
            table_row.append(&mut places_marks);
            table_row.append(&mut transition_states.into_iter().map(|tr| if tr { "X".to_string() } else { " ".to_string() }).collect());

            self.ui_rows.borrow_mut().push(table_row.clone());
        }
    }
}
