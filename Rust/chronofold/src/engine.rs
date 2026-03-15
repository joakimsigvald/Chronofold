use crate::models::vacuum::Vacuum;
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

    pub fn advance(&mut self) {
        self.vacuum_state.tick += 1;
        self.vacuum_state.update_fugacity();
        let excited_indices = self.vacuum_state.get_excited_indices();
        let connected_ids = self.resolve_handshakes(&excited_indices);
        self.vacuum_state
            .update_affinity(&excited_indices, connected_ids);
        self.vacuum_state.update_topology();
    }

    pub fn vacuum(&self) -> &Vacuum {
        &self.vacuum_state
    }

    fn update_fugacity(&mut self) {
        for monad in &mut self.vacuum_state.monads {
            monad.update_fugacity();
        }
    }

    fn resolve_handshakes(&mut self, excited_indices: &[usize]) -> HashSet<u32> {
        let mut connected_ids = std::collections::HashSet::new();
        let mut newborns = Vec::new();
        let mut handshakes = Vec::new();

        for &idx in excited_indices {
            let source_id = self.vacuum_state.monads[idx].id;
            if connected_ids.contains(&source_id) {
                continue;
            }

            let result = match self.vacuum_state.monads[idx].get_targeted_id() {
                None => {
                    let (hs, newborn) = self.vacuum_state.perform_genesis_handshake(idx);
                    Some((hs, Some(newborn)))
                }
                Some(target_id) => {
                    if connected_ids.contains(&target_id) {
                        continue;
                    }
                    self.vacuum_state
                        .perform_peer_handshake(idx, target_id)
                        .map(|hs| (hs, None))
                }
            };

            // Standardized bookkeeping for both Genesis and Internal
            if let Some((hs, newborn_opt)) = result {
                connected_ids.extend([hs.source_id, hs.target_id]);
                handshakes.push(hs);
                if let Some(newborn) = newborn_opt {
                    newborns.push(newborn);
                }
            }
        }

        self.vacuum_state.monads.extend(newborns);
        self.vacuum_state.handshakes = handshakes;

        connected_ids
    }
}
