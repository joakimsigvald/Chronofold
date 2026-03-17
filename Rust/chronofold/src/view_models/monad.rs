use serde::Serialize;
use crate::models::Monad;

#[derive(Serialize)]
pub struct MonadView {
    pub id: u32,
    pub fugacity: f32,
    pub affinity: f32,
}

impl MonadView {
    pub fn from_model(monad: &Monad) -> Self {
    Self {
        id: monad.id,
        fugacity: monad.fugacity,
        affinity: monad.affinity,
    }
}
}
