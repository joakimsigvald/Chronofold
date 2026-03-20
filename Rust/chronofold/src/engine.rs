use crate::models::{
    handshake::Handshake,
    monad::Monad,
    vacuum::{TopologyUpdate, Vacuum},
};
use std::collections::HashSet;

pub struct ChronofoldEngine {
    vacuum_state: Vacuum,
}

impl ChronofoldEngine {
    pub fn ignite() -> Self {
        Self {
            vacuum_state: Vacuum::create(),
        }
    }

    pub fn vacuum(&self) -> &Vacuum {
        &self.vacuum_state
    }

    pub fn advance(&mut self) {
        self.vacuum_state.advance_time();
        let excited_indices = self.vacuum_state.get_excited_indices();
        let topology_update = self.resolve_handshakes(&excited_indices);
        self.vacuum_state.update_topology(topology_update);
    }

    // TODO: Prioritize handshake resolution by "adventurousness" (distant peers > close peers > genesis).
    // What: Evaluate all connection intents before resolving them, favoring the longest-distance connections.
    // Why: Prevents local, low-energy handshakes from cannibalizing available Monads before high-fugacity
    // signals can connect. This ensures the network favors complex topology over unnecessary newborn bloat.
    fn resolve_handshakes(&mut self, excited_indices: &[usize]) -> TopologyUpdate {
        excited_indices
            .iter()
            .fold(TopologyUpdate::default(), |mut update, &idx| {
                if let Some((src, target)) = self.get_handshake_invite(idx, &update) {
                    update.excited_ids.insert(src);
                    if let Some(res) =
                        self.perform_handshake(idx, src, target, &update.connected_ids)
                    {
                        update.record_success(res.0, res.1);
                    }
                }
                update
            })
    }

    fn get_handshake_invite(
        &self,
        idx: usize,
        update: &TopologyUpdate,
    ) -> Option<(u32, Option<u32>)> {
        self.vacuum_state
            .get_at(idx)
            .filter(|source| !update.connected_ids.contains(&source.id))
            .map(|source| (source.id, source.get_targeted_id()))
    }

    fn perform_handshake(
        &mut self,
        idx: usize,
        source_id: u32,
        target_id_opt: Option<u32>,
        connected: &HashSet<u32>,
    ) -> Option<(Handshake, Option<Monad>)> {
        match target_id_opt {
            None => {
                let (hs, newborn) = self.vacuum_state.perform_genesis_handshake(idx);
                Some((hs, Some(newborn)))
            }
            Some(target_id) => (!self.is_bounce(source_id, target_id, connected))
                .then(|| self.vacuum_state.perform_peer_handshake(idx, target_id))
                .flatten()
                .map(|hs| (hs, None)),
        }
    }

    fn is_bounce(&self, source_id: u32, target_id: u32, connected_ids: &HashSet<u32>) -> bool {
        connected_ids.contains(&target_id) || self.vacuum_state.is_bounce(source_id, target_id)
    }
}
