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
    fn declare_simple_pn() -> Result<(), String> {
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
            places => L1, L2, L3, L4, L5, L6, L7, L8;
            transitions => T1, T2, T3, T4;
            connections => (L1 -> T1),
                           (T1 -> L2),
                           (L2 -> T2),
                           (T2 -> L4),
                           (L4 -> T3),
                           (T3 -> L7),
                           (T3 -> L6),
                           (L7 -> T4),
                           (L6 -> T4),
                           (T4 -> L8),
                           (T3 2-> L3),
                           (L3 2-> T2),
                           (T4 3-> L5),
                           (L5 3-> T2);
        };

        pn.places()[0].add_tokens(2); // TODO: Declarative way of adding tokens
        pn.places()[2].add_tokens(2); // TODO: Declarative way of adding tokens
        pn.places()[4].add_tokens(5); // TODO: Declarative way of adding tokens

        let simul = net::Simulation::new(pn);
        simul.run();
        simul.print_table();

        Ok(())
    }
}
