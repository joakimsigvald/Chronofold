use crate::models::Monad;
use serde::Serialize;

#[derive(Serialize)]
pub struct HandshakeView {
    pub source_id: u32,
    pub target_id: u32,
    pub strength: f32,
}

impl HandshakeView {
    pub fn from_peers(source: &Monad, target: &Monad) -> Self {
        Self {
            source_id: source.id,
            target_id: target.id,
            strength: Self::compute_strength(
                source.index_of(target.id),
                target.index_of(source.id),
            ),
        }
    }

    fn compute_strength(idx_a: usize, idx_b: usize) -> f32 {
        1.0 / (((idx_a + 1) * (idx_b + 1)) as f32).sqrt()
    }
}
