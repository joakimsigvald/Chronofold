use crate::models::Monad;
use serde::Serialize;

#[derive(Serialize)]
pub struct LinkView {
    pub left_id: u32,
    pub right_id: u32,
    pub strength: f32,
}

impl LinkView {
    pub fn from_peers(left: &Monad, right: &Monad) -> Option<Self> {
        let left_dist = left.distance(right.id)?;
        let right_dist = right.distance(left.id)?;

        Some(Self {
            left_id: left.id,
            right_id: right.id,
            strength: Self::compute_strength(left_dist, right_dist),
        })
    }

    fn compute_strength(left_idx: usize, right_idx: usize) -> f32 {
        1.0 / (((left_idx + 1) * (right_idx + 1)) as f32).sqrt()
    }
}
