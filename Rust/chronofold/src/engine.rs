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
        self.resolve_handshakes(&excited_indices);
        self.vacuum_state.update_topology();
    }

    pub fn vacuum(&self) -> &Vacuum {
        &self.vacuum_state
    }

    fn resolve_handshakes(&mut self, excited_indices: &[usize]) {
        let mut connected_ids = std::collections::HashSet::new();
        let mut newborns = Vec::new();

        let is_bounce = |target_id: u32, source_id: u32| -> bool {
            if connected_ids.contains(&target_id) {
                return true;
            }
            match self.vacuum_state.get_monad(target_id) {
                Some(target) => !target.knows(source_id),
                None => true,
            }
        };

        for &idx in excited_indices {
            let source_id = self.vacuum_state.get_at(idx).unwrap().id;
            if connected_ids.contains(&source_id) {
                continue;
            }

            let result = match self.vacuum_state.get_at(idx).unwrap().get_targeted_id() {
                None => {
                    let (hs, newborn) = self.vacuum_state.perform_genesis_handshake(idx);
                    Some((hs, Some(newborn)))
                }
                Some(target_id) => {
                    if connected_ids.contains(&target_id) {
                        //bounce
                        self.vacuum_state.get_at(idx).unwrap().escalate_affinity();
                        continue;
                    }
                    self.vacuum_state
                        .perform_peer_handshake(idx, target_id)
                        .map(|hs| (hs, None))
                }
            };

            if let Some((hs, newborn_opt)) = result {
                connected_ids.extend([hs.source_id, hs.target_id]);
                if let Some(newborn) = newborn_opt {
                    newborns.push(newborn);
                }
            }
        }

        self.vacuum_state.extend(newborns);
    }
}
