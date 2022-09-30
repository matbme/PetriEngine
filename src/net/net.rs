use std::rc::Rc;

use crate::net::*;

#[derive(Default)]
pub struct PetriNet {
    places: Vec<Rc<Place>>,
    transitions: Vec<Rc<Transition>>,
    connections: Vec<Rc<Connection>>,
}

impl PetriNet {
    /// Creates an empty Petri Net
    pub fn new() -> Self {
        Self {
            places: vec![],
            transitions: vec![],
            connections: vec![],
        }
    }

    /// Adds `place` to the net
    pub fn add_place(&mut self, place: Place) {
        self.places.push(Rc::new(place));
    }

    /// Adds `transition` to the net
    pub fn add_transition(&mut self, transition: Transition) {
        self.transitions.push(Rc::new(transition));
    }

    /// Adds `connection` to the net
    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(Rc::new(connection));
    }

    /// Return a reference to `places`
    pub fn places(&self) -> &Vec<Rc<Place>> {
        &self.places
    }

    /// Return a reference to `transitions`
    pub fn transitions(&self) -> &Vec<Rc<Transition>> {
        &self.transitions
    }

    /// Return a reference to `connections`
    pub fn connections(&self) -> &Vec<Rc<Connection>> {
        &self.connections
    }

    /// Return place named `name`, if it doesn't exist, return None
    pub fn place_with_name(&self, name: &str) -> Option<Rc<Place>> {
        for place in self.places.iter() {
            if place.name() == name {
                return Some(place.clone());
            }
        }

        None
    }

    /// Return transition named `name`, if it doesn't exist, return None
    pub fn transition_with_name(&self, name: &str) -> Option<Rc<Transition>> {
        for transition in self.transitions.iter() {
            if transition.name() == name {
                return Some(transition.clone());
            }
        }

        None
    }
}

#[macro_export]
macro_rules! petri_net {
    (places $var:ident $name:ident) => {
        $var.add_place(crate::net::Place::new(stringify!($name)));
    };

    (transitions $var:ident $name:ident) => {
        $var.add_transition(crate::net::Transition::new(stringify!($name)));
    };

    (connections $var:ident ( $from:ident $($weight:literal)? -> $to:ident $($ctype:ident)? )) => {
        #[allow(unused_mut, unused_assignments)]
        let mut con_type = crate::net::ConnectionType::NORMAL;

        #[allow(unused_variables)]
        let weight: i32 = 1;

        $(
            match stringify!($ctype) {
                "inhibitor" => con_type = crate::net::ConnectionType::INHIBITOR,
                "reset" => con_type = crate::net::ConnectionType::RESET,
                _ => unimplemented!()
            }
        )?

        $(
            let weight = $weight;
        )?

        if let Some(place) = $var.place_with_name(stringify!($from)) {
            let input_from = crate::net::InputFrom::PLACE;
            let transition = $var.transition_with_name(stringify!($to)).expect("Transition does not exist");

            $var.add_connection(crate::net::Connection::new(place, transition, weight, input_from, con_type));
        } else {
            let input_from = crate::net::InputFrom::TRANSITION;
            let transition = $var.transition_with_name(stringify!($from)).expect("Transition does not exist");
            let place = $var.place_with_name(stringify!($to)).expect("Place does not exist");

            $var.add_connection(crate::net::Connection::new(place, transition, weight, input_from, con_type));
        }
    };

    ($( $decl:tt => [ $( $val:tt ),* ] ),+) => {
        {
            let mut new_net = crate::net::PetriNet::new();

            $(
                $( petri_net!($decl new_net $val); )+
            )+

            new_net
        }
    };
}
