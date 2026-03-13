use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Monad {
    pub id: u32,
}

#[derive(Serialize, Clone)]
pub struct Handshake {
    pub source_id: u32,
    pub target_id: u32, // The Active Horizon (Relational Distance)
}

#[derive(Serialize, Clone)]
pub struct VacuumState {
    pub tick: u64,
    pub monads: Vec<Monad>,
    pub handshakes: Vec<Handshake>, // Entanglements / Triad Closures
}
