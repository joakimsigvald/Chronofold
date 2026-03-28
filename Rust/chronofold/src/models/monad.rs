use std::collections::HashSet;

use crate::models::Invite;

const INITIAL_AFFINITY: f32 = 0.0;
const INITIAL_FUGACITY: f32 = 0.0;

#[derive(Clone)]
pub struct Monad {
    pub id: u32,
    pub horizon: Vec<u32>,
    pub fugacity: f32,
    pub affinity: f32,
    pub alpha: f32,
    pub beta: f32,
    pub kappa: f32,
    pub lambda: f32,
    pub tau_s: f32,
    pub tau_a: f32,
}

impl Monad {
    pub fn stress(&self) -> f32 {
        1.0 - ((1.0 - self.affinity) * (1.0 - self.fugacity)).sqrt()
    }

    pub fn aperture(&self) -> f32 {
        ((1.0 - self.affinity) * self.fugacity).sqrt()
    }

    pub fn accumulate_fugacity(&mut self) {
        self.fugacity += self.lambda * (1.0 - self.fugacity);
    }

    pub fn apply_bounce_penalty(&mut self, target_id: u32) {
        self.update_affinity(None);
        if let Some(pos) = self.distance(target_id) {
            let id = self.horizon.remove(pos);
            self.horizon.push(id);
        }
    }

    pub fn update_capacity(&mut self) {
        if self.affinity > self.tau_a {
            self.horizon.pop();
        }
    }

    pub fn entangle(&mut self, peer_id: u32) {
        self.update_fugacity(peer_id);
        self.update_affinity(Some(peer_id));
        self.elevate(peer_id);
    }

    pub fn spawn_newborn(&mut self, newborn_id: u32) -> Monad {
        Monad::create(
            newborn_id,
            vec![],
            self.alpha,
            self.beta,
            self.kappa,
            self.lambda,
            self.tau_s,
            self.tau_a,
        )
    }

    pub fn prune_horizon(&mut self, dead_ids: &HashSet<u32>) {
        self.horizon.retain(|id| !dead_ids.contains(id));
    }

    pub fn valence(&self) -> usize {
        self.horizon.len()
    }

    pub fn mass(&self) -> f32 {
        (self.valence() as f32) * self.stress()
    }

    pub fn distance(&self, peer_id: u32) -> Option<usize> {
        self.horizon.iter().position(|&id| id == peer_id)
    }

    pub fn get_higher_peers(&self) -> impl Iterator<Item = u32> + '_ {
        self.horizon.iter().copied().filter(move |&id| id > self.id)
    }

    pub fn knows(&self, peer_id: u32) -> bool {
        self.horizon.contains(&peer_id)
    }

    pub fn get_handshake_invite(&self, source_idx: usize) -> Invite {
        let target_idx = self.target_index();
        let target_id = self.horizon.get(target_idx);
        Invite {
            source_idx,
            source_id: self.id,
            source_mass: self.mass(),
            target_id: target_id.copied(),
        }
    }

    fn update_affinity(&mut self, peer_id: Option<u32>) {
        let penalty = peer_id
            .and_then(|id| self.distance(id))
            .map_or(1.0, |n| 1.0 - self.kappa.powi(n as i32));
        self.affinity += self.alpha * (penalty - self.affinity);
    }

    fn update_fugacity(&mut self, peer_id: u32) {
        self.fugacity = (1.0 - self.beta) * self.fugacity + (self.beta * self.get_recency(peer_id));
    }

    fn target_index(&self) -> usize {
        ((self.aperture() * (self.valence() as f32 + 1.0)) as usize).min(self.valence())
    }

    fn elevate(&mut self, peer_id: u32) {
        self.horizon.retain(|&id| id != peer_id);
        self.horizon.insert(0, peer_id);
    }

    fn get_recency(&self, target_id: u32) -> f32 {
        self.distance(target_id)
            .map(|n| 1.0 / 2.0_f32.powf(n as f32))
            .unwrap_or(0.0)
    }

    pub fn create(
        id: u32,
        horizon: Vec<u32>,
        alpha: f32,
        beta: f32,
        kappa: f32,
        lambda: f32,
        tau_s: f32,
        tau_a: f32,
    ) -> Monad {
        Self {
            id,
            horizon,
            affinity: INITIAL_AFFINITY,
            fugacity: INITIAL_FUGACITY,
            alpha,
            beta,
            kappa,
            lambda,
            tau_s,
            tau_a,
        }
    }
}
