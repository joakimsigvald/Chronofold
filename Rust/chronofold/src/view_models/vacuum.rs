use crate::view_models::link::LinkView;
use crate::view_models::monad::MonadView;

use serde::Serialize;

#[derive(Serialize)]
pub struct VacuumView {
    pub tick: u32,
    pub monads: Vec<MonadView>,
    pub links: Vec<LinkView>,
}
