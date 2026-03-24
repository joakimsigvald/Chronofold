use crate::models::monad::Monad;

#[derive(Clone)]
pub struct Handshake {
    pub source_id: u32,
    pub target_id: u32,
}

impl Handshake {
    pub fn perform(source: &mut Monad, target: &mut Monad) -> Handshake {
        let source_peer = *source.horizon.first().unwrap();
        let target_peer = *target.horizon.first().unwrap_or(&target.id);
        source.entangle(target.id, target_peer);
        target.entangle(source.id, source_peer);
        Self {
            source_id: source.id,
            target_id: target.id,
        }
    }
}
