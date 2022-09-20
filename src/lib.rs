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
        pn.transitions().print_table();
        pn.connections().print_table();

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

        pn.places().print_table();
        pn.transitions().print_table();
        pn.connections().print_table();

        let simul = net::Simulation::new(pn);
        simul.run();

        simul.net().places().print_table();
        simul.net().transitions().print_table();
        simul.net().connections().print_table();

        Ok(())
    }
}
