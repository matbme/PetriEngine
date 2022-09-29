pub mod connection;
pub mod net;
pub mod place;
pub mod simulation;
pub mod transition;

pub use connection::{Connection, ConnectionType, InputFrom};
pub use net::PetriNet;
pub use place::Place;
pub use simulation::Simulation;
pub use transition::Transition;

pub trait Connectable {
    fn connection_title(&self) -> &str;
}
