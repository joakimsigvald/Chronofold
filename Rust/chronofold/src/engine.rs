use std::collections::HashSet;

use crate::models::handshake::Handshake;
use crate::models::monad::Monad;
use crate::models::vacuum::Vacuum;

const INITIAL_ALPHA: f64 = 0.1;
const INITIAL_BETA: f64 = 0.1;
const INITIAL_LAMBDA: f64 = 0.1;
const INITIAL_TAU_S: f64 = 0.5;

pub struct ChronofoldEngine {
    vacuum_state: Vacuum,
}

impl ChronofoldEngine {
    pub fn ignite() -> Self {
        Self {
            vacuum_state: Vacuum {
                tick: 0,
                monads: vec![
                    Monad::create(
                        0,
                        1,
                        INITIAL_ALPHA,
                        INITIAL_BETA,
                        INITIAL_LAMBDA,
                        INITIAL_TAU_S,
                    ),
                    Monad::create(
                        1,
                        0,
                        INITIAL_ALPHA,
                        INITIAL_BETA,
                        INITIAL_LAMBDA,
                        INITIAL_TAU_S,
                    ),
                ],
                handshakes: vec![Handshake {
                    source_id: 0,
                    target_id: 1,
                }],
            },
        }
    }

    pub fn advance(&mut self) {
        self.vacuum_state.tick += 1;
        self.prune_monads();
        self.prune_handshakes();
        self.update_fugacity();
        self.process_handshakes();
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
            // Find all Monads that currently have an empty horizon
            let dead_ids: HashSet<u32> = self
                .vacuum_state
                .monads
                .iter()
                .filter(|m| m.horizon.is_empty())
                .map(|m| m.id)
                .collect();

            // If no Monads are scheduled for death, the topology is stable. Break the loop.
            if dead_ids.is_empty() {
                break;
            }

            // Remove the dead Monads from reality
            self.vacuum_state
                .monads
                .retain(|m| !dead_ids.contains(&m.id));

            // Scrub the dead IDs from the horizons of all surviving Monads
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

    fn process_handshakes(&mut self) {
        let excited_indices = self.get_excited_indices();
        self.resolve_waveforms(&excited_indices);
        self.apply_thermodynamic_updates(&excited_indices);
    }

    // We can compress this entire phase into a clean, functional iterator.
    fn get_excited_indices(&self) -> Vec<usize> {
        self.vacuum_state
            .monads
            .iter()
            .enumerate()
            .filter(|(_, m)| m.stress() > m.tau_s)
            .map(|(i, _)| i)
            .collect()
    }

    fn resolve_waveforms(&mut self, excited_indices: &[usize]) {
        let mut resolved_ids = std::collections::HashSet::new();
        let mut newborns = Vec::new();
        let mut current_handshakes = Vec::new();

        let mut next_id = self
            .vacuum_state
            .monads
            .iter()
            .map(|m| m.id)
            .max()
            .unwrap_or(0)
            + 1;

        for &idx in excited_indices {
            let source_id = self.vacuum_state.monads[idx].id;
            if resolved_ids.contains(&source_id) {
                continue;
            }

            match self.vacuum_state.monads[idx].get_targeted_id() {
                None => {
                    // --- GENESIS HANDSHAKE ---
                    let source = &mut self.vacuum_state.monads[idx];
                    let (handshake, newborn) =
                        Self::execute_genesis_handshake(source, &mut next_id);

                    resolved_ids.extend([source_id, newborn.id]);
                    newborns.push(newborn);
                    current_handshakes.push(handshake);
                }

                Some(target_id) => {
                    // --- INTERNAL HANDSHAKE ---
                    if resolved_ids.contains(&target_id) {
                        continue;
                    }

                    if let Some(target_vec_idx) = self
                        .vacuum_state
                        .monads
                        .iter()
                        .position(|m| m.id == target_id)
                    {
                        if let Ok([source, target]) = self
                            .vacuum_state
                            .monads
                            .get_disjoint_mut([idx, target_vec_idx])
                        {
                            let handshake = Self::execute_internal_handshake(source, target);

                            resolved_ids.extend([source_id, target_id]);
                            current_handshakes.push(handshake);
                        }
                    }
                }
            }
        }

        self.vacuum_state.monads.extend(newborns);
        self.vacuum_state.handshakes = current_handshakes;
    }

    fn apply_thermodynamic_updates(&mut self, excited_indices: &[usize]) {
        // 1. Map successes
        let mut successful_ids = std::collections::HashSet::new();
        for hs in &self.vacuum_state.handshakes {
            successful_ids.insert(hs.source_id);
            successful_ids.insert(hs.target_id);
        }

        // 2. Map attempted signals
        let mut signaled_ids = std::collections::HashSet::new();
        for &idx in excited_indices {
            signaled_ids.insert(self.vacuum_state.monads[idx].id);
        }

        // 3. Apply state changes based on network elasticity rules
        for monad in &mut self.vacuum_state.monads {
            let success = successful_ids.contains(&monad.id);
            let signaled = signaled_ids.contains(&monad.id);
            if success {
                monad.affinity *= 1.0 - monad.alpha;
            } else if signaled {
                monad.affinity += monad.alpha * (1.0 - monad.affinity);
            }
        }
    }

    fn execute_genesis_handshake(source: &mut Monad, next_id: &mut u32) -> (Handshake, Monad) {
        let newborn_id = *next_id;
        *next_id += 1;

        let mut newborn = Monad::create(
            newborn_id,
            source.id,
            source.alpha,
            source.beta,
            source.lambda,
            source.tau_s,
        );

        // We can reuse our existing internal logic to perfectly entangle them!
        let handshake = Self::execute_internal_handshake(source, &mut newborn);

        (handshake, newborn)
    }

    fn execute_internal_handshake(source: &mut Monad, target: &mut Monad) -> Handshake {
        source.entangle(target.id);
        target.entangle(source.id);
        Handshake::create(source.id, target.id)
    }
}
