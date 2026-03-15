use std::collections::HashSet;

const INITIAL_AFFINITY: f64 = 0.0;
const INITIAL_FUGACITY: f64 = 0.0;
const INITIAL_CAPACITY: usize = 1;

#[derive(Clone)]
pub struct Monad {
    pub id: u32,
    pub horizon: Vec<u32>,
    pub fugacity: f64,
    pub affinity: f64,
    pub capacity: usize,
    pub alpha: f64,
    pub beta: f64,
    pub lambda: f64,
    pub tau_s: f64,
    pub tau_f: f64,
    pub tau_a: f64,
}

impl Monad {
    pub fn stress(&self) -> f64 {
        1.0 - ((1.0 - self.affinity) * (1.0 - self.fugacity)).sqrt()
    }

    pub fn aperture(&self) -> f64 {
        ((1.0 - self.affinity) * self.fugacity).sqrt()
    }

    pub fn get_targeted_id(&self) -> Option<u32> {
        let idx = self.target_index();
        if idx >= self.horizon.len() {
            None // Genesis
        } else {
            Some(self.horizon[idx]) // Internal
        }
    }

    pub fn update_fugacity(&mut self) {
        let n = self.horizon.len() as f64;
        if n < 1.0 {
            panic!(
                "Topological Entropy Violation: Monad {} attempted to update with an empty horizon. \
                This indicates a failure in the pruning cascade.",
                self.id
            );
        }
        self.fugacity += self.lambda * (1.0 - self.fugacity) / n;
    }

    pub fn update_capacity(&mut self) {
        if self.fugacity > self.tau_f {
            self.capacity += 1;
        }
        if self.affinity > self.tau_a {
            self.capacity -= 1;
        }
        while self.horizon.len() > self.capacity as usize {
            self.horizon.pop();
        }
    }

    pub fn entangle(&mut self, target_id: u32) {
        self.fugacity =
            (1.0 - self.beta) * self.fugacity + (self.beta * self.get_recency(target_id));
        self.elevate(target_id);
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

    fn target_index(&self) -> usize {
        let p = self.aperture();
        if p < 1.0 {
            (p * (self.capacity as f64)).floor() as usize
        } else {
            self.capacity.saturating_sub(1)
        }
    }

    fn elevate(&mut self, target_id: u32) {
        self.horizon.retain(|&id| id != target_id);
        self.horizon.insert(0, target_id);
    }

    fn get_recency(&self, target_id: u32) -> f64 {
        self.horizon
            .iter()
            .position(|&id| id == target_id)
            .map(|n| 2.0_f64.powf(-(n as f64)))
            .unwrap_or(0.0)
    }

    pub fn create(
        id: u32,
        horizon: Vec<u32>,
        alpha: f64,
        beta: f64,
        lambda: f64,
        tau_s: f64,
        tau_f: f64,
        tau_a: f64,
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
