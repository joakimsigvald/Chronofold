use serde::Serialize;

const INITIAL_AFFINITY: f64 = 0.0;
const INITIAL_FUGACITY: f64 = 0.0;
const INITIAL_CAPACITY: usize = 1;

#[derive(Serialize, Clone)]
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

    pub fn entangle(&mut self, target_id: u32) {
        self.fugacity =
            (1.0 - self.beta) * self.fugacity + (self.beta * self.get_recency(target_id));
        self.elevate(target_id);
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
        parent_id: u32,
        alpha: f64,
        beta: f64,
        lambda: f64,
        tau_s: f64,
    ) -> Monad {
        Self {
            id,
            horizon: vec![parent_id],
            affinity: INITIAL_AFFINITY,
            fugacity: INITIAL_FUGACITY,
            capacity: INITIAL_CAPACITY,
            alpha,
            beta,
            lambda,
            tau_s,
        }
    }
}
