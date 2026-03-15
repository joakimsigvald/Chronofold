use std::collections::HashSet;

use crate::models::handshake::Handshake;
use crate::models::vacuum::Vacuum;

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
        self.prune_monads();
        self.prune_handshakes();
        self.update_fugacity();

        let excited_indices = self.get_excited_indices();
        let connected_indices = self.resolve_handshakes(&excited_indices);
        self.vacuum_state
            .update_affinity(&excited_indices, connected_indices);
    }

    pub fn vacuum(&self) -> &Vacuum {
        &self.vacuum_state
    }

    fn update_fugacity(&mut self) {
        for monad in &mut self.vacuum_state.monads {
            monad.update_fugacity();
        }
    }

    fn prune_monads(&mut self) {
        loop {
            let dead_ids: HashSet<u32> = self
                .vacuum_state
                .monads
                .iter()
                .filter(|m| m.horizon.is_empty())
                .map(|m| m.id)
                .collect();

            if dead_ids.is_empty() {
                break;
            }

            self.vacuum_state
                .monads
                .retain(|m| !dead_ids.contains(&m.id));

            for monad in &mut self.vacuum_state.monads {
                monad.horizon.retain(|id| !dead_ids.contains(id));
            }
        }
    }

    fn prune_handshakes(&mut self) {
        let monad_ids: std::collections::HashSet<u32> =
            self.vacuum_state.monads.iter().map(|m| m.id).collect();
        self.vacuum_state
            .handshakes
            .retain(|h| monad_ids.contains(&h.source_id) && monad_ids.contains(&h.target_id));
    }

    fn get_excited_indices(&self) -> Vec<usize> {
        self.vacuum_state
            .monads
            .iter()
            .enumerate()
            .filter(|(_, m)| m.stress() > m.tau_s)
            .map(|(i, _)| i)
            .collect()
    }

    fn resolve_handshakes(&mut self, excited_indices: &[usize]) -> HashSet<u32> {
        let mut connected_ids = std::collections::HashSet::new();
        let mut newborns = Vec::new();
        let mut current_handshakes = Vec::new();

        for &idx in excited_indices {
            let source_id = self.vacuum_state.monads[idx].id;
            if connected_ids.contains(&source_id) {
                continue;
            }

            let (handshake, newborn_opt) = match self.vacuum_state.monads[idx].get_targeted_id() {
                None => {
                    let (source, mut newborn) = self.vacuum_state.spawn_newborn(idx);
                    (Handshake::perform(source, &mut newborn), Some(newborn))
                }

                Some(target_id) => {
                    if connected_ids.contains(&target_id) {
                        continue;
                    }

                    let Some(target_vec_idx) = self.vacuum_state.find_monad(target_id) else {
                        continue;
                    };

                    let Some([source, target]) =
                        self.vacuum_state.get_disjoint_monads(idx, target_vec_idx)
                    else {
                        continue;
                    };
                    (Handshake::perform(source, target), None)
                }
            };

            connected_ids.extend([handshake.source_id, handshake.target_id]);
            current_handshakes.push(handshake);

            if let Some(newborn) = newborn_opt {
                newborns.push(newborn);
            }
        }

        self.vacuum_state.monads.extend(newborns);
        self.vacuum_state.handshakes = current_handshakes;
        connected_ids
    }
}
