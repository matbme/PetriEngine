//! # Overview
//!
//! This is a simple declarative implementation of a Petri Net engine.
//!
//! TODO: Documentation

pub mod net;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_pn() -> Result<(), String>  {
        let pn = petri_net! {
            places => P1, P2;
            transitions => T1;
            connections => (P1 -> T1 reset), (T1 2-> P2);
        };

        println!("{:?}", pn.places());
        println!("{:?}", pn.transitions());
        println!("{:?}", pn.connections());

        Ok(())
    }
}
