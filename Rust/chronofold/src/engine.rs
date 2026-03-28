use crate::models::{
    handshake::Handshake, invite::Invite, monad::Monad, topology_update::TopologyUpdate,
    vacuum::Vacuum,
};
use std::cmp::Reverse;

pub struct ChronofoldEngine {
    vacuum_state: Vacuum,
}

enum ResolveResult {
    Success(Handshake, Option<Monad>),
    Bounce(Invite),
    Defer(Invite),
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
        let mut deferred = Vec::new();
        for invite in invites {
            if update.is_resolved(invite.source_id) {
                continue;
            }
            let result = match invite.target_id {
                Some(target_id) => self.try_resolve_known(invite, target_id, update, pass),
                None => {
                    if pass < 3 {
                        ResolveResult::Defer(invite)
                    } else {
                        self.try_resolve_unknown(invite, update)
                    }
                }
            };
            match result {
                ResolveResult::Success(hs, newborn) => update.record_success(hs, newborn),
                ResolveResult::Bounce(bounce_invite) => update.record_bounce(&bounce_invite),
                ResolveResult::Defer(deferred_invite) => deferred.push(deferred_invite),
            }
        }
        deferred
    }

    fn try_resolve_known(
        &mut self,
        invite: Invite,
        target_id: u32,
        update: &mut TopologyUpdate,
        pass: u8,
    ) -> ResolveResult {
        if update.is_resolved(target_id) {
            if pass < 3 {
                return ResolveResult::Defer(invite);
            }
            if let Some(ricochet_id) =
                self.find_ricochet_target(target_id, invite.source_id, update)
            {
                if let Some(hs) =
                    self.perform_ricochet_handshake(invite.source_idx, target_id, ricochet_id)
                {
                    return ResolveResult::Success(hs, None);
                }
            }
            return ResolveResult::Bounce(invite);
        }
        let source_mass = self.vacuum_state.get_at(invite.source_idx).unwrap().mass();
        let target_info = self
            .vacuum_state
            .get_monad(target_id)
            .map(|monad| (monad.mass(), monad.knows(invite.source_id)));
        match target_info {
            None => ResolveResult::Bounce(invite),
            Some((target_mass, known)) => {
                if Self::should_defer_peer(pass, source_mass, target_mass, known) {
                    ResolveResult::Defer(invite)
                } else {
                    let hs = self
                        .vacuum_state
                        .perform_peer_handshake(invite.source_idx, target_id)
                        .unwrap();
                    ResolveResult::Success(hs, None)
                }
            }
        }
    }

    fn try_resolve_unknown(
        &mut self,
        invite: Invite,
        update: &mut TopologyUpdate,
    ) -> ResolveResult {
        if let Some(hs) = self.try_resolve_weave(&invite, update) {
            return ResolveResult::Success(hs, None);
        }
        let (hs, newborn) = self
            .vacuum_state
            .perform_genesis_handshake(invite.source_idx);
        ResolveResult::Success(hs, Some(newborn))
    }

    fn try_resolve_weave(
        &mut self,
        invite: &Invite,
        update: &mut TopologyUpdate,
    ) -> Option<Handshake> {
        let source_monad = self.vacuum_state.get_at(invite.source_idx).unwrap();
        let &proxy_id = source_monad.horizon.first()?;
        let ricochet_id = self
            .find_ricochet_target(proxy_id, invite.source_id, update)
            .filter(|&id| !source_monad.knows(id))?;
        return self.perform_ricochet_handshake(invite.source_idx, proxy_id, ricochet_id);
    }

    fn perform_ricochet_handshake(
        &mut self,
        source_idx: usize,
        proxy_id: u32,
        ricochet_id: u32,
    ) -> Option<Handshake> {
        let c_monad = self.vacuum_state.get_monad(ricochet_id)?;
        if !c_monad.knows(proxy_id) {
            return None;
        }
        self.vacuum_state
            .perform_peer_handshake(source_idx, ricochet_id)
    }

    fn find_ricochet_target(
        &self,
        proxy_id: u32,
        source_id: u32,
        update: &TopologyUpdate,
    ) -> Option<u32> {
        let proxy_monad = self.vacuum_state.get_monad(proxy_id)?;
        for &potential_target in &proxy_monad.horizon {
            if potential_target == source_id || update.is_resolved(potential_target) {
                continue;
            }
            return Some(potential_target);
        }
        None
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
