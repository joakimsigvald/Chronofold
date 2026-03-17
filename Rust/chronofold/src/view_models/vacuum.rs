use crate::view_models::handshake::HandshakeView;
use crate::view_models::monad::MonadView;

use serde::Serialize;

#[derive(Serialize)]
pub struct VacuumView {
    pub tick: u32,
    pub monads: Vec<MonadView>,
    pub handshakes: Vec<HandshakeView>,
}
