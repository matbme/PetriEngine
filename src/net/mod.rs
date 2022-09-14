pub mod connection;
pub mod net;
pub mod place;
pub mod transition;
pub mod simulation;

pub use net::PetriNet;
pub use connection::{Connection, ConnectionType, InputFrom};
pub use place::Place;
pub use transition::Transition;
pub use simulation::Simulation;

pub trait Connectable { }
