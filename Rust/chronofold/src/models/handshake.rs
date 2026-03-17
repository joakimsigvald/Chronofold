use crate::models::monad::Monad;

#[derive(Clone)]
pub struct Handshake {
    pub source_id: u32,
    pub target_id: u32,
}

impl Handshake {
    pub fn perform(source: &mut Monad, target: &mut Monad) -> Handshake {
        source.entangle(target);
        target.entangle(source);
        Self {
            source_id: source.id,
            target_id: target.id,
        }
    }
}
