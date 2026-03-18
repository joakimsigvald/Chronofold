use std::collections::HashSet;

const INITIAL_AFFINITY: f32 = 0.0;
const INITIAL_FUGACITY: f32 = 0.0;
const INITIAL_CAPACITY: u8 = 1;

#[derive(Clone)]
pub struct Monad {
    pub id: u32,
    pub horizon: Vec<u32>,
    pub fugacity: f32,
    pub affinity: f32,
    pub capacity: u8,
    pub alpha: f32,
    pub beta: f32,
    pub lambda: f32,
    pub tau_s: f32,
    pub tau_f: f32,
    pub tau_a: f32,
}

impl Monad {
    pub fn stress(&self) -> f32 {
        1.0 - ((1.0 - self.affinity) * (1.0 - self.fugacity)).sqrt()
    }

    pub fn aperture(&self) -> f32 {
        ((1.0 - self.affinity) * self.fugacity).sqrt()
    }

    pub fn get_targeted_id(&self) -> Option<u32> {
        let idx = self.target_index();
        if idx >= self.valence() {
            None // Genesis
        } else {
            Some(self.horizon[idx]) // Internal
        }
    }

    pub fn update_fugacity(&mut self) {
        let n = self.valence();
        if n == 0 {
            panic!(
                "Topological Entropy Violation: Monad {} attempted to update with an empty horizon. \
                This indicates a failure in the pruning cascade.",
                self.id
            );
        }
        self.fugacity += self.lambda * (1.0 - self.fugacity) / (n as f32);
    }

    pub fn update_capacity(&mut self) {
        if self.fugacity > self.tau_f {
            self.capacity = self.capacity.saturating_add(1);
        }
        if self.affinity > self.tau_a {
            self.capacity = self.capacity.saturating_sub(1);
        }
        if self.is_over_capacity() {
            self.horizon.pop();
        }
    }

    pub fn entangle(&mut self, target: &mut Monad) {
        self.fugacity =
            (1.0 - self.beta) * self.fugacity + (self.beta * self.get_recency(target.id));
        self.elevate(target.id);
        self.sample_from(target);
        target.sample_from(self);
    }

    pub fn spawn_newborn(&mut self, newborn_id: u32) -> Monad {
        Monad::create(
            newborn_id,
            vec![self.id],
            self.alpha,
            self.beta,
            self.lambda,
            self.tau_s,
            self.tau_f,
            self.tau_a,
        )
    }

    pub fn update_affinity(&mut self, connected: bool, excited: bool) {
        if connected {
            self.affinity *= 1.0 - self.alpha;
        } else if excited {
            self.affinity += self.alpha * (1.0 - self.affinity);
        }
    }

    pub fn prune_horizon(&mut self, dead_ids: &HashSet<u32>) {
        self.horizon.retain(|id| !dead_ids.contains(id));
    }

    pub fn capacity(&self) -> usize {
        self.capacity as usize
    }

    pub fn valence(&self) -> usize {
        self.horizon.len()
    }

    pub fn is_over_capacity(&self) -> bool {
        self.valence() > self.capacity()
    }

    pub fn sample_from(&mut self, peer: &Monad) {
        self.has_free_capacity()
            .then(|| self.find_novel_link_from(peer))
            .flatten()
            .map(|id| self.horizon.push(id));
    }

    pub fn index_of(&self, peer_id: u32) -> usize {
        self.horizon.iter().position(|&id| id == peer_id).unwrap()
    }

    pub fn get_higher_peers(&self) -> impl Iterator<Item = u32> + '_ {
        self.horizon.iter().copied().filter(move |&id| id > self.id)
    }

    fn find_novel_link_from(&self, peer: &Monad) -> Option<u32> {
        peer.horizon
            .iter()
            .find(|&id| *id != self.id && !self.horizon.contains(&id))
            .copied()
    }

    fn has_free_capacity(&self) -> bool {
        self.valence() < self.capacity()
    }

    fn target_index(&self) -> usize {
        let p = self.aperture();
        if p < 1.0 {
            (p * (self.capacity as f32)).floor() as usize
        } else {
            self.capacity() - 1
        }
    }

    fn elevate(&mut self, target_id: u32) {
        self.horizon.retain(|&id| id != target_id);
        self.horizon.insert(0, target_id);
    }

    fn get_recency(&self, target_id: u32) -> f32 {
        self.horizon
            .iter()
            .position(|&id| id == target_id)
            .map(|n| 1.0 / 2.0_f32.powf(n as f32))
            .unwrap_or(0.0)
    }

    pub fn create(
        id: u32,
        horizon: Vec<u32>,
        alpha: f32,
        beta: f32,
        lambda: f32,
        tau_s: f32,
        tau_f: f32,
        tau_a: f32,
    ) -> Monad {
        Self {
            id,
            horizon,
            affinity: INITIAL_AFFINITY,
            fugacity: INITIAL_FUGACITY,
            capacity: INITIAL_CAPACITY,
            alpha,
            beta,
            lambda,
            tau_s,
            tau_f,
            tau_a,
        }
    }
}
