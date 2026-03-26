use std::collections::{HashMap, HashSet};

use crate::models::topology_update::TopologyUpdate;
use crate::models::handshake::Handshake;
use crate::models::monad::Monad;

/// Penalty (α): How quickly Affinity (clinging) spikes after a failed signal.
/// Lower values (like 0.05) make the network more resilient to rejection.
const ALPHA: f32 = 0.15;

/// Relaxation (β): The satisfying "cool-down" of Fugacity after a handshake.
/// Higher values make successful connections more "fulfilling."
const BETA: f32 = 0.2;

// Cohesion (κ): The distance tolerance dial for Affinity.
/// Pulls Affinity toward `1.0 - κ^n` upon a successful handshake at index `n`.
/// Higher values (e.g., 0.95) let the Monad tolerate deeper connections,
/// while lower values cause Affinity to spike aggressively even at short distances.
const KAPPA: f32 = 0.95;

/// Friction (λ): Passive Fugacity growth per tick.
/// High values make Monads crave novelty faster.
const LAMBDA: f32 = 0.1;

/// Critical Stress (τ_S): The threshold for action.
/// When internal stress > τ_S, the Monad is forced to signal.
const TAU_S: f32 = 0.5;

/// Critical Contraction (τ_A): The threshold for retreat.
/// When Affinity > τ_A, the Monad loses -1 Dimensional Capacity.
const TAU_A: f32 = 0.7;

#[derive(Clone)]
pub struct Vacuum {
    pub tick: u32,
    pub max_id: u32,
    monads: Vec<Monad>,
    monad_indices: HashMap<u32, usize>,
}

impl Vacuum {
    pub fn create() -> Vacuum {
        let monads = vec![Self::create_primordial(0, 1), Self::create_primordial(1, 0)];
        let monad_indices: HashMap<u32, usize> = Self::build_index_map(&monads);
        let max_id = monads.iter().map(|m| m.id).max().unwrap();
        Self {
            tick: 0,
            max_id: max_id,
            monads: monads,
            monad_indices: monad_indices,
        }
    }

    pub fn monads(&self) -> impl Iterator<Item = &Monad> {
        self.monads.iter()
    }

    pub fn advance_time(&mut self) {
        self.tick += 1;
        for monad in &mut self.monads {
            monad.accumulate_fugacity();
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

    pub fn update_topology(&mut self, update: TopologyUpdate) {
        self.resolve_bounces(&update);
        self.extend(update.newborns());
        self.update_capacity();
        self.prune_monads();
    }

    pub fn get_at(&self, index: usize) -> Option<&Monad> {
        self.monads.get(index)
    }

    pub fn get_monad(&self, id: u32) -> Option<&Monad> {
        self.find_monad(id).and_then(|index| self.monads.get(index))
    }

    fn get_monad_mut(&mut self, id: u32) -> &mut Monad {
        &mut self.monads[self.monad_indices[&id]]
    }

    fn resolve_bounces(&mut self, update: &TopologyUpdate) {
        for (source_id, target_id) in update.bounces() {
            self.get_monad_mut(*source_id).apply_bounce_penalty(*target_id);
        }
    }

    fn extend(&mut self, newborns: Vec<Monad>) {
        let start_idx = self.monads.len();
        for (offset, monad) in newborns.iter().enumerate() {
            self.monad_indices.insert(monad.id, start_idx + offset);
        }
        self.monads.extend(newborns);
    }

    fn update_capacity(&mut self) {
        for monad in &mut self.monads {
            monad.update_capacity();
        }
    }

    fn find_monad(&self, id: u32) -> Option<usize> {
        self.monad_indices.get(&id).copied()
    }

    fn prune_monads(&mut self) {
        let mut changed = false;
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
            changed = true;
            self.monads.retain(|m| !new_dead_ids.contains(&m.id));
            for monad in &mut self.monads {
                monad.prune_horizon(&new_dead_ids);
            }
        }
        if changed {
            self.monad_indices = Self::build_index_map(&self.monads);
        }
    }

    fn build_index_map(monads: &Vec<Monad>) -> HashMap<u32, usize> {
        monads.iter().enumerate().map(|(i, m)| (m.id, i)).collect()
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

    fn create_primordial(source_id: u32, peer_id: u32) -> Monad {
        Monad::create(
            source_id,
            vec![peer_id],
            ALPHA,
            BETA,
            KAPPA,
            LAMBDA,
            TAU_S,
            TAU_A,
        )
    }
}
