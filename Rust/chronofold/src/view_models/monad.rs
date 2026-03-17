use serde::Serialize;

#[derive(Serialize)]
pub struct MonadView {
    pub id: u32,
    pub fugacity: f32,
    pub affinity: f32,
}
