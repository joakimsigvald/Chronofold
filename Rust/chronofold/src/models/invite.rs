#[derive(Clone)]
pub struct Invite {
    pub source_idx: usize,
    pub source_id: u32,
    pub source_mass: f32,
    pub target_id: Option<u32>,
}
