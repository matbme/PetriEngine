use petri_engine::{net, petri_net};

fn main() {
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

    let simul = net::Simulation::new_interactive(pn);
    simul.run();
}
