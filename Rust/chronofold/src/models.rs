use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Monad {
    pub id: u32,
    pub horizon: Vec<u32>,
    pub fugacity: f64,
    pub affinity: f64,
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
