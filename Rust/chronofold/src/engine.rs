use crate::models::{
    handshake::Handshake, invite::Invite, monad::Monad, topology_update::TopologyUpdate,
    vacuum::Vacuum,
};

pub struct ChronofoldEngine {
    vacuum_state: Vacuum,
}

impl ChronofoldEngine {
    pub fn ignite() -> Self {
        Self {
            vacuum_state: Vacuum::create(),
        }
    }

    pub fn vacuum(&self) -> &Vacuum {
        &self.vacuum_state
    }

    pub fn advance(&mut self) {
        self.vacuum_state.advance_time();
        let excited_indices = self.vacuum_state.get_excited_indices();
        let topology_update = self.resolve_handshakes(&excited_indices);
        self.vacuum_state.update_topology(topology_update);
    }

    fn resolve_handshakes(&mut self, excited_indices: &[usize]) -> TopologyUpdate {
        let mut invites: Vec<_> = excited_indices
            .iter()
            .filter_map(|idx| self.get_handshake_invite(*idx))
            .collect();
        invites.sort_unstable_by(|a, b| {
            a.is_genesis()
                .cmp(&b.is_genesis())
                .then_with(|| b.distance.cmp(&a.distance))
                .then_with(|| a.valence.cmp(&b.valence))
                .then_with(|| b.stress.total_cmp(&a.stress))
        });
        invites
            .into_iter()
            .fold(TopologyUpdate::default(), |mut update, invite| {
                if update.is_resolved(invite.source_id) {
                    return update;
                }

                if let Some((hs, newborn)) = self.perform_handshake(&invite, &update) {
                    update.record_success(hs, newborn);
                } else {
                    update.record_bounce(&invite);
                }
                update
            })
    }

    fn get_handshake_invite(&self, idx: usize) -> Option<Invite> {
        self.vacuum_state
            .get_at(idx)
            .map(|source| source.get_handshake_invite(idx))
    }

    fn perform_handshake(
        &mut self,
        invite: &Invite,
        update: &TopologyUpdate,
    ) -> Option<(Handshake, Option<Monad>)> {
        match invite.target_id {
            None => {
                let (hs, newborn) = self
                    .vacuum_state
                    .perform_genesis_handshake(invite.source_idx);
                Some((hs, Some(newborn)))
            }
            Some(target_id) => (!self.is_bounce(invite.source_id, target_id, &update))
                .then(|| {
                    self.vacuum_state
                        .perform_peer_handshake(invite.source_idx, target_id)
                })
                .flatten()
                .map(|hs| (hs, None)),
        }
    }

    fn is_bounce(&self, source_id: u32, target_id: u32, update: &TopologyUpdate) -> bool {
        update.is_resolved(target_id) || self.vacuum_state.is_bounce(source_id, target_id)
    }
}
