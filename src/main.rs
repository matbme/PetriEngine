use petri_engine::{net, petri_net};

fn main() {
    let pn = petri_net! {
        places => [L1<2>, L2, L3<2>, L4, L5<5>, L6, L7, L8],
        transitions => [T1 -> |t, _, _| {
            println!("T1 is active and has ID {}", t.id());
        }, T2, T3, T4],
        connections => [
            (2) L1 -> T1,
            (2) T1 -> L2,
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

    let simul = net::Simulation::new_interactive(pn);
    simul.run();
}
