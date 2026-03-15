use std::collections::HashSet;

use crate::models::handshake::Handshake;
use crate::models::monad::Monad;

use serde::Serialize;

const INITIAL_ALPHA: f64 = 0.1;
const INITIAL_BETA: f64 = 0.1;
const INITIAL_LAMBDA: f64 = 0.1;
const INITIAL_TAU_S: f64 = 0.5;

#[derive(Serialize, Clone)]
pub struct Vacuum {
    pub tick: u64,
    pub max_id: u32,
    pub monads: Vec<Monad>,
    pub handshakes: Vec<Handshake>, // Entanglements / Triad Closures
}

impl Vacuum {
    pub fn create() -> Vacuum {
        Self {
            tick: 0,
            max_id: 1,
            monads: vec![Self::create_primordial(0, 1), Self::create_primordial(1, 0)],
            handshakes: vec![Handshake {
                source_id: 0,
                target_id: 1,
            }],
        }
    }

    pub fn prune(&mut self) {
        self.prune_monads();
        self.prune_handshakes();
    }

    pub fn update_fugacity(&mut self) {
        for monad in &mut self.monads {
            monad.update_fugacity();
        }
    }

    pub fn get_excited_indices(&self) -> Vec<usize> {
        self.monads
            .iter()
            .enumerate()
            .filter(|(_, m)| m.stress() > m.tau_s)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn perform_genesis_handshake(&mut self, source_idx: usize) -> (Handshake, Monad) {
        let (source, mut newborn) = self.spawn_newborn(source_idx);
        let handshake = Handshake::perform(source, &mut newborn);
        (handshake, newborn)
    }

    pub fn perform_peer_handshake(&mut self, source_idx: usize, target_id: u32) -> Option<Handshake> {
        let target_idx = self.find_monad(target_id)?;
        let [source, target] = self.get_disjoint_monads(source_idx, target_idx)?;
        Some(Handshake::perform(source, target))
    }

    pub fn update_affinity(&mut self, excited_indices: &[usize], connected_ids: HashSet<u32>) {
        let excited_ids = self.get_ids(excited_indices);

        for monad in &mut self.monads {
            let connected = connected_ids.contains(&monad.id);
            let excited = excited_ids.contains(&monad.id);

            monad.update_affinity(connected, excited);
        }
    }

    fn find_monad(&self, id: u32) -> Option<usize> {
        self.monads.iter().position(|m| m.id == id)
    }

    fn get_disjoint_monads(&mut self, idx1: usize, idx2: usize) -> Option<[&mut Monad; 2]> {
        self.monads.get_disjoint_mut([idx1, idx2]).ok()
    }

    fn spawn_newborn(&mut self, source_idx: usize) -> (&mut Monad, Monad) {
        self.max_id += 1;
        let source = &mut self.monads[source_idx];
        let newborn = source.spawn_newborn(self.max_id);
        (source, newborn)
    }

    fn prune_monads(&mut self) {
        loop {
            let dead_ids: HashSet<u32> = self
                .monads
                .iter()
                .filter(|m| m.horizon.is_empty())
                .map(|m| m.id)
                .collect();

            if dead_ids.is_empty() {
                break;
            }

            self.monads.retain(|m| !dead_ids.contains(&m.id));

            for monad in &mut self.monads {
                monad.horizon.retain(|id| !dead_ids.contains(id));
            }
        }
    }

    fn prune_handshakes(&mut self) {
        let monad_ids: std::collections::HashSet<u32> = self.monads.iter().map(|m| m.id).collect();
        self.handshakes
            .retain(|h| monad_ids.contains(&h.source_id) && monad_ids.contains(&h.target_id));
    }

    fn get_ids(&self, indices: &[usize]) -> HashSet<u32> {
        indices.iter().map(|&idx| self.monads[idx].id).collect()
    }

    fn create_primordial(source_id: u32, peer_id: u32) -> Monad {
        Monad::create(
            source_id,
            vec![peer_id],
            INITIAL_ALPHA,
            INITIAL_BETA,
            INITIAL_LAMBDA,
            INITIAL_TAU_S,
        )
    }
}
