use std::collections::HashSet;

use crate::models::handshake::Handshake;
use crate::models::monad::Monad;

/// Friction (λ): Passive Fugacity growth per tick.
/// High values make Monads crave novelty faster.
const LAMBDA: f32 = 0.2;

/// Relaxation (β): The satisfying "cool-down" of Fugacity after a handshake.
/// Higher values make successful connections more "fulfilling."
const BETA: f32 = 0.1;

/// Penalty (α): How quickly Affinity (clinging) spikes after a failed signal.
/// Lower values (like 0.05) make the network more resilient to rejection.
const ALPHA: f32 = 0.1;

/// Critical Stress (τ_S): The threshold for action.
/// When internal stress > τ_S, the Monad is forced to signal.
const TAU_S: f32 = 0.5;

/// Critical Expansion (τ_F): The threshold for growth.
/// When Fugacity > τ_F, the Monad gains +1 Dimensional Capacity.
const TAU_F: f32 = 0.7;

/// Critical Contraction (τ_A): The threshold for retreat.
/// When Affinity > τ_A, the Monad loses -1 Dimensional Capacity.
const TAU_A: f32 = 0.7;

#[derive(Clone)]
pub struct Vacuum {
    pub tick: u32,
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

    pub fn perform_peer_handshake(
        &mut self,
        source_idx: usize,
        target_id: u32,
    ) -> Option<Handshake> {
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

    pub fn update_topology(&mut self) {
        for monad in &mut self.monads {
            monad.update_capacity();
        }
        let dead_ids = self.kill_monads();
        if dead_ids.is_empty() {
            return;
        }
        self.handshakes
            .retain(|h| !dead_ids.contains(&h.source_id) && !dead_ids.contains(&h.target_id));
    }

    fn kill_monads(&mut self) -> HashSet<u32> {
        let mut all_dead_ids = HashSet::new();

        loop {
            let new_dead_ids: HashSet<u32> = self
                .monads
                .iter()
                .filter(|m| m.horizon.is_empty())
                .map(|m| m.id)
                .collect();

            if new_dead_ids.is_empty() {
                break;
            }

            all_dead_ids.extend(&new_dead_ids);
            self.monads.retain(|m| !new_dead_ids.contains(&m.id));
            for monad in &mut self.monads {
                monad.prune_horizon(&new_dead_ids);
            }
        }

        all_dead_ids
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

    fn get_ids(&self, indices: &[usize]) -> HashSet<u32> {
        indices.iter().map(|&idx| self.monads[idx].id).collect()
    }

    fn create_primordial(source_id: u32, peer_id: u32) -> Monad {
        Monad::create(
            source_id,
            vec![peer_id],
            ALPHA,
            BETA,
            LAMBDA,
            TAU_S,
            TAU_F,
            TAU_A,
        )
    }
}
