use std::collections::HashMap;

pub mod link;
pub mod monad;
pub mod vacuum;

pub use link::LinkView;
pub use monad::MonadView;
pub use vacuum::VacuumView;

use crate::models::{Monad, Vacuum};

pub fn map_to_view(vacuum: &Vacuum) -> VacuumView {
    VacuumView {
        tick: vacuum.tick,
        monads: map_monads(vacuum),
        links: map_links(vacuum),
    }
}

fn map_monads(vacuum: &Vacuum) -> Vec<MonadView> {
    vacuum.monads().map(MonadView::from_model).collect()
}

fn map_links(vacuum: &Vacuum) -> Vec<LinkView> {
    let monad_map: HashMap<u32, &Monad> = vacuum.monads().map(|m| (m.id, m)).collect();
    vacuum
        .monads()
        .flat_map(|m| m.get_higher_peers().map(move |right_id| (m.id, right_id)))
        .map(|(left_id, right_id)| (monad_map[&left_id], monad_map[&right_id]))
        .map(|(left, right)| LinkView::from_peers(left, right))
        .collect()
}
