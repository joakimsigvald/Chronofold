use crate::models::monad::Monad;
use crate::models::handshake::Handshake;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Vacuum {
    pub tick: u64,
    pub monads: Vec<Monad>,
    pub handshakes: Vec<Handshake>, // Entanglements / Triad Closures
}
