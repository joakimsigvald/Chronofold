use std::collections::HashMap;

pub mod handshake;
pub mod monad;
pub mod vacuum;

pub use handshake::HandshakeView;
pub use monad::MonadView;
pub use vacuum::VacuumView;

use crate::models::{Monad, Vacuum};

pub fn map_to_view(vacuum: &Vacuum) -> VacuumView {
    let monad_views: Vec<MonadView> = vacuum.monads.iter().map(map_monad).collect();
    let monad_map: HashMap<u32, &Monad> = vacuum.monads.iter().map(|m| (m.id, m)).collect();
    let mut handshake_views = Vec::new();
    for monad_a in &vacuum.monads {
        for (idx_a, &target_id) in monad_a.horizon.iter().enumerate() {
            if monad_a.id > target_id {
                continue;
            }
            let Some(&monad_b) = monad_map.get(&target_id) else {
                continue;
            };
            let Some(idx_b) = monad_b.horizon.iter().position(|&id| id == monad_a.id) else {
                continue;
            };
            let strength = (((idx_a + 1) * (idx_b + 1)) as f32).sqrt();
            handshake_views.push(HandshakeView {
                source_id: monad_a.id,
                target_id,
                strength,
            });
        }
    }

    VacuumView {
        tick: vacuum.tick,
        monads: monad_views,
        handshakes: handshake_views,
    }
}

/// Helper function to map individual Monads from a reference
fn map_monad(monad: &Monad) -> MonadView {
    MonadView {
        id: monad.id,
        fugacity: monad.fugacity,
        affinity: monad.affinity,
    }
}
