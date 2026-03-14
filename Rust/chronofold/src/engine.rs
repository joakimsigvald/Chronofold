use std::collections::HashSet;

use crate::models::{Handshake, Monad, VacuumState};

pub struct ChronofoldEngine {
    state: VacuumState,
    pub lambda: f64, //fugacity change rate. TODO: move into the monad as an evolutionary property
}

impl ChronofoldEngine {
    pub fn ignite() -> Self {
        Self {
            lambda: 0.1,
            state: VacuumState {
                tick: 0,
                monads: vec![
                    Monad {
                        id: 0,
                        horizon: vec![1],
                        affinity: 0.0,
                        fugacity: 0.0,
                    },
                    Monad {
                        id: 1,
                        horizon: vec![0],
                        affinity: 0.0,
                        fugacity: 0.0,
                    },
                ],
                handshakes: vec![Handshake {
                    source_id: 0,
                    target_id: 1,
                }],
            },
        }
    }

    pub fn advance(&mut self) {
        self.state.tick += 1;
        self.prune_monads();
        self.prune_handshakes();
        for monad in &mut self.state.monads {
            let n = monad.horizon.len() as f64;

            // We no longer need to check if n == 0, because the cascade loop
            // guarantees all surviving Monads have N >= 1.
            monad.fugacity += self.lambda * (1.0 - monad.fugacity) / n;
        }
    }

    pub fn state(&self) -> &VacuumState {
        &self.state
    }

    fn prune_monads(&mut self) {
        loop {
            // Find all Monads that currently have an empty horizon
            let dead_ids: HashSet<u32> = self
                .state
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
            self.state.monads.retain(|m| !dead_ids.contains(&m.id));

            // Scrub the dead IDs from the horizons of all surviving Monads
            for monad in &mut self.state.monads {
                monad.horizon.retain(|id| !dead_ids.contains(id));
            }
        }
    }

    fn prune_handshakes(&mut self) {
        let monad_ids: std::collections::HashSet<u32> =
            self.state.monads.iter().map(|m| m.id).collect();
        self.state
            .handshakes
            .retain(|h| monad_ids.contains(&h.source_id) && monad_ids.contains(&h.target_id));
    }
}
