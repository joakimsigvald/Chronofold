// models/mod.rs
pub mod monad;
pub mod handshake;
pub mod vacuum;

// Re-export them so you can use crate::models::Monad 
// instead of crate::models::monad::Monad
pub use monad::Monad;
pub use handshake::Handshake;
pub use vacuum::Vacuum;