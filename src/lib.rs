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
            places => [P1, P2],
            transitions => [T1],
            connections => [P1 >> T1, (2) T1 -> P2]
        };

        pn.places().print_table();
        pn.transitions().print_table();
        pn.connections().print_table();

        Ok(())
    }

    #[test]
    fn run_simulation() -> Result<(), String> {
        let start = std::time::Instant::now();

        let pn = petri_net! {
            places => [L1<2>, L2, L3<2>, L4, L5<5>, L6, L7, L8],
            transitions => [T1 -> |t, _, _| {
                println!("T1 is active and has ID {}", t.id());
            }, T2, T3, T4],
            connections => [
                L1 -> T1,
                T1 -> L2,
                L2 -> T2,
                T2 -> L4,
                L4 -> T3,
                T3 -> L7,
                T3 -> L6,
                L7 -> T4,
                L6 -> T4,
                T4 -> L8,
                (2) T3 -> L3,
                (2) L3 -> T2,
                (3) T4 -> L5,
                (3) L5 -> T2
            ]
        };

        let simul = net::Simulation::new(pn);
        simul.run();

        let end = std::time::Instant::now();

        simul.print_table();

        println!("Elapsed time: {:?}", end - start);

        Ok(())
    }

    #[test]
    fn concurrency_check() -> Result<(), String> {
        let pn = petri_net! {
            places => [L1<1>],
            transitions => [T1, T2],
            connections => [
                L1 -> T1,
                L1 -> T2
            ]
        };

        let simul = net::Simulation::new(pn);
        simul.run();

        simul.print_table();

        Ok(())
    }
}
