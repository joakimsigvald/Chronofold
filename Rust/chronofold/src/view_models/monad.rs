use serde::Serialize;

#[derive(Serialize)]
pub struct MonadView {
    pub id: u32,
    pub fugacity: f64,
    pub affinity: f64,
}
