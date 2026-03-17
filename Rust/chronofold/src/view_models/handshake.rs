use serde::Serialize;

#[derive(Serialize)]
pub struct HandshakeView {
    pub source_id: u32,
    pub target_id: u32,
    pub strength: f32,
}
