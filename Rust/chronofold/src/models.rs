use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Monad {
    pub id: u32,
    pub horizon: Vec<u32>, // The Active Horizon (Relational Distance)
}

#[derive(Serialize, Clone)]
pub struct VacuumState {
    pub tick: u64,
    pub monads: Vec<Monad>,
    pub links: Vec<(u32, u32)>, // Entanglements / Triad Closures
}
