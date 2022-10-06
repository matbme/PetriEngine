use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdout;
use std::rc::Rc;

use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::prelude::*;

use super::{Connection, PetriNet, Place, Transition};
use crate::ui::UITable;

// A `Transition` (key) connects to a `Place` via a `Connection` (value tuple)
pub type ConnectionMap = HashMap<Rc<Transition>, Vec<(Rc<Place>, Rc<Connection>)>>;

// A `Plaace` (key) can have at least two concurrent `Connection`s (value)
pub type ConcurrencyMap = HashMap<Rc<Place>, Vec<Rc<Connection>>>;

pub struct Simulation {
    net: PetriNet,
    incoming_connections: ConnectionMap,
    outgoing_connections: ConnectionMap,
    concurrent_connections: ConcurrencyMap,
    ui_rows: RefCell<Vec<Vec<String>>>, // Store a log of each step for printing
    interactive: bool,                  // Whether user confirmation is required to advance
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
    /// Create a new `Simulation` with the provided `net`.
    pub fn new(net: PetriNet) -> Self {
        let mut simul = Self {
            net,
            incoming_connections: HashMap::new(),
            outgoing_connections: HashMap::new(),
            concurrent_connections: HashMap::new(),
            ui_rows: RefCell::new(vec![]),
            interactive: false,
        };

        simul.scan_connections();
        simul.check_concurrency();

        simul
    }

    /// Create a new interactive `Simulation` with the provided `net`. An interactive simulation
    /// requires the user to press *ENTER* to advance cycles.
    pub fn new_interactive(net: PetriNet) -> Self {
        let mut simul = Self {
            net,
            incoming_connections: HashMap::new(),
            outgoing_connections: HashMap::new(),
            concurrent_connections: HashMap::new(),
            ui_rows: RefCell::new(vec![]),
            interactive: true,
        };

        simul.scan_connections();
        simul.check_concurrency();

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
                super::InputFrom::PLACE => self
                    .incoming_connections
                    .get_mut(&connection.transition())
                    .expect("Incoming connections map was not properly initialized")
                    .push((connection.place().clone(), connection.clone())),
                super::InputFrom::TRANSITION => self
                    .outgoing_connections
                    .get_mut(&connection.transition())
                    .expect("Outgoing connections map was not properly initialized")
                    .push((connection.place().clone(), connection.clone())),
            }
        }
    }

    fn check_concurrency(&mut self) {
        // Initialize keys
        for place in self.net.places().iter() {
            self.concurrent_connections.insert(place.clone(), vec![]);
        }

        // For each connection, if input is from place, add to corresponding vector
        for connection in self.net.connections().iter() {
            if connection.input_from() == super::InputFrom::PLACE {
                self.concurrent_connections
                    .get_mut(&connection.place())
                    .expect("Concurrent transactions map was not properly initialized")
                    .push(connection.clone());
            }
        }

        // Iterate through all map keys. If vector contains only one entry,
        // there is no concurrency and key is dropped
        let mut drop_keys = vec![];
        for (key, value) in self.concurrent_connections.iter() {
            if value.len() <= 1 {
                drop_keys.push(key.clone());
            }
        }

        drop_keys.into_iter().for_each(|key| {
            self.concurrent_connections.remove(&key);
        });
    }

    fn place_matches_requirement(&self, place: &Place, connection: &Connection) -> bool {
        let tokens = place.tokens();

        match connection.connection_type() {
            super::ConnectionType::NORMAL => connection.weight() <= &tokens,
            super::ConnectionType::INHIBITOR => connection.weight() > &tokens,
            super::ConnectionType::RESET => true,
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
    fn consume_tokens(&self, transition: &Transition, incoming_connections: &ConnectionMap) {
        if let Some(vals) = incoming_connections.get(transition) {
            for (place, connection) in vals.iter() {
                match connection.connection_type() {
                    super::ConnectionType::NORMAL => {
                        place.remove_tokens(connection.weight().clone())
                    }
                    super::ConnectionType::INHIBITOR => {}
                    super::ConnectionType::RESET => place.clear_tokens(),
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

    /// Execute the simulation
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
            let mut enabled_transitions = vec![];
            for transition in self.net.transitions() {
                if self.transition_enabled(transition.as_ref()) {
                    enabled_transitions.push(transition.clone());
                }
            }

            // Choose path for concurrent connections
            let mut rng = rand::thread_rng();
            let mut unconcurrent_incoming_connections = self.incoming_connections.clone();
            for (_, connections) in self.concurrent_connections.iter() {
                let disable_cons = connections
                    .into_iter()
                    .filter(|con| enabled_transitions.contains(&con.transition()))
                    .choose_multiple(&mut rng, connections.len() - 1);

                for connection in disable_cons.iter() {
                    unconcurrent_incoming_connections
                        .get_mut(&connection.transition())
                        .expect("Incoming connections map was not properly initialized")
                        .retain(|(_, map_connection)| {
                            map_connection.as_ref() != connection.as_ref()
                        });

                    enabled_transitions
                        .retain(|tr| tr.id() != connection.transition().as_ref().id());
                }
            }

            // Run cycle
            for transition in self.net.transitions().iter() {
                if enabled_transitions.contains(transition) {
                    some_transition_enabled = true;
                    if let Some(callback) = transition.callback() {
                        callback(
                            &transition,
                            &unconcurrent_incoming_connections,
                            &self.outgoing_connections,
                        );
                    }

                    self.consume_tokens(transition.as_ref(), &unconcurrent_incoming_connections);
                    self.propagate_tokens(transition.as_ref());
                }
            }

            // Insert cycle status into table
            let mut table_row = vec![cycle_count.to_string()];
            table_row.append(&mut places_marks);
            table_row.append(&mut Vec::from_iter(self.net.transitions().into_iter().map(
                |tr| {
                    if enabled_transitions.contains(tr) {
                        "X".to_string()
                    } else {
                        " ".to_string()
                    }
                },
            )));
            self.ui_rows.borrow_mut().push(table_row.clone());

            // Interactive session
            if self.interactive {
                self.print_table();

                execute!(
                    stdout(),
                    Print("Press "),
                    SetForegroundColor(Color::Magenta),
                    Print("ENTER"),
                    ResetColor,
                    Print(" to execute the next step, or "),
                    SetForegroundColor(Color::Red),
                    Print("c"),
                    ResetColor,
                    Print(" to cancel the simulation.\n"),
                )
                .expect("Failed to setup interactive session");

                enable_raw_mode().expect("Failed to setup interactive session");

                loop {
                    match read().expect("Failed to setup interactive session") {
                        Event::Key(event) => match event.code {
                            KeyCode::Enter => break,
                            KeyCode::Char('c') => {
                                disable_raw_mode().expect("Failed to setup interactive session");
                                return;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                disable_raw_mode().expect("Failed to setup interactive session");
            }
        }
    }
}
