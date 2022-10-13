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
            places => [L1, L2<2>, L3<1>, L4, L5, L6, L7<10>, L8, L9, L10, L11, L12, L13],
            transitions => [Ta, Tb, Tc, Td, Te, Tf, Tg],
            connections => [
                (2) L1 -> Ta,
                Ta -> L4,
                L4 -> Td,
                L7 >> Td,
                Td -> L11,
                L11 -> Tg,
                Tg -> L12,
                Tb -> L1,
                Tb -> L5,
                L5 -> Te,
                Te -> L8,
                L8 -> Td,
                L8 -> Tg,
                Te -> L9,
                L9 -> Tg,
                Te -> L10,
                Tg -> L13,
                L13 -> Tf,
                L10 -> Tf,
                L2 -> Tb,
                Tf -> L2,
                Tf -> L6,
                (2) L6 @ Te,
                L3 -> Tc,
                Tc -> L6
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
