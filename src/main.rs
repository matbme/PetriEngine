use petri_engine::{net, petri_net};

fn main() {
    let pn = petri_net! {
        places => [L1<2>, L2, L3<2>, L4, L5<5>, L6, L7, L8],
        transitions => [T1 -> |t, incoming, outgoing| {
            println!("{:?}", t);
            println!("{:?}", incoming);
            println!("{:?}", outgoing);
        }, T2, T3, T4],
        connections => [
            (L1 -> T1),
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
            (L5 3-> T2)
        ]
    };

    let simul = net::Simulation::new_interactive(pn);
    simul.run();
}
