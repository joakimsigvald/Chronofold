#[derive(Clone)]
pub struct Invite {
    pub source_idx: usize,
    pub distance: usize,
    pub valence: usize,
    pub stress: f32,
    pub source_id: u32,
    pub target_id: Option<u32>,
}

impl Invite {
    pub fn is_genesis(&self) -> bool {
        self.distance == self.valence
    }
}
