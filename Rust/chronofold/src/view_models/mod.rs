use std::collections::HashMap;

pub mod handshake;
pub mod monad;
pub mod vacuum;

pub use handshake::HandshakeView;
pub use monad::MonadView;
pub use vacuum::VacuumView;

use crate::models::{Monad, Vacuum};

pub fn map_to_view(vacuum: &Vacuum) -> VacuumView {
    VacuumView {
        tick: vacuum.tick,
        monads: map_monads(vacuum),
        handshakes: map_handshakes(vacuum),
    }
}

fn map_monads(vacuum: &Vacuum) -> Vec<MonadView> {
    vacuum.monads.iter().map(MonadView::from_model).collect()
}

fn map_handshakes(vacuum: &Vacuum) -> Vec<HandshakeView> {
    let monad_map: HashMap<u32, &Monad> = vacuum.monads.iter().map(|m| (m.id, m)).collect();
    vacuum
        .monads
        .iter()
        .flat_map(|m| m.get_higher_peers().map(move |id_b| (m.id, id_b)))
        .map(|(id_a, id_b)| (monad_map[&id_a], monad_map[&id_b]))
        .map(|(source, target)| HandshakeView::from_peers(source, target))
        .collect()
}
