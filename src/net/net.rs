use std::rc::Rc;

use crate::net::*;

#[derive(Debug, Default)]
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
macro_rules! connection_type {
    (->) => { crate::net::ConnectionType::NORMAL };
    (@) => { crate::net::ConnectionType::INHIBITOR };
    (>>) => { crate::net::ConnectionType::RESET };
}

#[macro_export]
macro_rules! petri_net {
    (places $var:ident [ $($name:ident $(< $tokens:literal >)?),+ ]) => {
        $(
            let place = crate::net::Place::new(stringify!($name));
            $( place.add_tokens($tokens); )?
            $var.add_place(place);
        )+
    };

    (transitions $var:ident [ $($name:ident $(-> |$tr:pat_param, $inc:pat_param, $out:pat_param| $callback:block)?),+ ]) => {
        $(
            #[allow(unused_mut)]
            let mut transition = crate::net::Transition::new(stringify!($name));

            $( transition.add_callback(Box::new(|$tr, $inc, $out| $callback)); )?

            $var.add_transition(transition);
        )+
    };

    (connections $var:ident [ $( $(( $weight:literal ))? $from:ident $ctype:tt $to:ident ),+ ]) => {
        $(
            let con_type = $crate::connection_type!($ctype);

            #[allow(unused_variables)]
            let weight: i32 = 1;
            $( let weight = $weight; )?

            if let Some(place) = $var.place_with_name(stringify!($from)) {
                let input_from = crate::net::InputFrom::PLACE;
                let transition = $var.transition_with_name(stringify!($to))
                    .expect("Transition does not exist");

                $var.add_connection(
                    crate::net::Connection::new(place, transition, weight, input_from, con_type)
                );
            } else {
                let input_from = crate::net::InputFrom::TRANSITION;
                let transition = $var.transition_with_name(stringify!($from))
                    .expect("Transition does not exist");
                let place = $var.place_with_name(stringify!($to))
                    .expect("Place does not exist");

                $var.add_connection(
                    crate::net::Connection::new(place, transition, weight, input_from, con_type)
                );
            }
        )+
    };

    ($( $decl:tt => $vals:tt ),+) => {
        {
            let mut new_net = crate::net::PetriNet::new();

            $( petri_net!($decl new_net $vals); )+

            new_net
        }
    };
}
