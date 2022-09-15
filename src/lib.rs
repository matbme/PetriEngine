//! # Overview
//!
//! This is a simple declarative implementation of a Petri Net engine.
//!
//! TODO: Documentation

pub mod net;
pub mod ui;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::UITable;

    #[test]
    fn declare_simple_pn() -> Result<(), String>  {
        let pn = petri_net! {
            places => P1, P2;
            transitions => T1;
            connections => (P1 -> T1 reset), (T1 2-> P2);
        };

        pn.places().print_table();
        println!("{:?}", pn.transitions());
        println!("{:?}", pn.connections());

        Ok(())
    }

    #[test]
    fn run_simulation() -> Result<(), String> {
        let pn = petri_net! {
            places => P1, P2;
            transitions => T1;
            connections => (P1 -> T1 reset), (T1 2-> P2);
        };

        pn.places()[0].add_tokens(1); // TODO: Declarative way of adding tokens

        println!("{:?}", pn.places());
        println!("{:?}", pn.transitions());
        println!("{:?}", pn.connections());

        let simul = net::Simulation::new(pn);
        simul.run();

        println!("{:?}", simul.net().places());
        println!("{:?}", simul.net().transitions());
        println!("{:?}", simul.net().connections());

        Ok(())
    }
}
