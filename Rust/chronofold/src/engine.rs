use crate::models::{invite::Invite, topology_update::TopologyUpdate, vacuum::Vacuum};
use std::cmp::Reverse;

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
        let mut update = TopologyUpdate::default();
        let phase_1: Vec<_> = self.collect_invites(excited_indices);
        let phase_2 = self.process_invites(phase_1, &mut update, 1);
        let phase_3 = self.process_invites(phase_2, &mut update, 2);
        self.process_invites(phase_3, &mut update, 3);
        update
    }

    fn collect_invites(&self, excited_indices: &[usize]) -> Vec<Invite> {
        let mut invites: Vec<_> = excited_indices
            .iter()
            .filter_map(|&idx| self.get_handshake_invite(idx))
            .collect();
        invites.sort_unstable_by_key(|inv| Reverse(inv.source_mass.to_bits()));
        invites
    }

    fn process_invites(
        &mut self,
        invites: Vec<Invite>,
        update: &mut TopologyUpdate,
        pass: u8,
    ) -> Vec<Invite> {
        invites
            .into_iter()
            .filter_map(|invite| {
                if update.is_resolved(invite.source_id) {
                    return None;
                }
                match invite.target_id {
                    Some(target_id) => self.try_resolve_peer(invite, target_id, update, pass),
                    None => self.try_resolve_genesis(invite, update, pass),
                }
            })
            .collect()
    }

    fn try_resolve_peer(
        &mut self,
        invite: Invite,
        target_id: u32,
        update: &mut TopologyUpdate,
        pass: u8,
    ) -> Option<Invite> {
        if update.is_resolved(target_id) {
            update.record_bounce(&invite);
            return None;
        }
        let source_mass = self.vacuum_state.get_at(invite.source_idx).unwrap().mass();
        let target_info = self
            .vacuum_state
            .get_monad(target_id)
            .map(|monad| (monad.mass(), monad.knows(invite.source_id)));
        match target_info {
            None => {
                update.record_bounce(&invite);
                None
            }
            Some((target_mass, known)) => {
                if Self::should_defer_peer(pass, source_mass, target_mass, known) {
                    Some(invite)
                } else {
                    let hs = self
                        .vacuum_state
                        .perform_peer_handshake(invite.source_idx, target_id)
                        .unwrap();
                    update.record_success(hs, None);
                    None
                }
            }
        }
    }

    fn try_resolve_genesis(
        &mut self,
        invite: Invite,
        update: &mut TopologyUpdate,
        pass: u8,
    ) -> Option<Invite> {
        if pass < 3 {
            Some(invite)
        } else {
            let (hs, newborn) = self
                .vacuum_state
                .perform_genesis_handshake(invite.source_idx);
            update.record_success(hs, Some(newborn));
            None
        }
    }

    fn should_defer_peer(pass: u8, source_mass: f32, target_mass: f32, known: bool) -> bool {
        if known {
            pass == 1 && source_mass < target_mass
        } else {
            pass < 3
        }
    }

    fn get_handshake_invite(&self, idx: usize) -> Option<Invite> {
        self.vacuum_state
            .get_at(idx)
            .map(|source| source.get_handshake_invite(idx))
    }
}
