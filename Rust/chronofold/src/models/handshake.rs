use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Handshake {
    pub source_id: u32,
    pub target_id: u32,
}

impl Handshake {
    pub fn create(source_id: u32, target_id: u32) -> Handshake {
        Self {
            source_id,
            target_id,
        }
    }
}
