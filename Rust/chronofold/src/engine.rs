use crate::models::{
    invite::Invite, topology_update::TopologyUpdate,
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
        let mut update = TopologyUpdate::default();
        let invites: Vec<_> = excited_indices
            .iter()
            .filter_map(|&idx| self.get_handshake_invite(idx))
            .collect();

        let phase_2 = self.process_invites(invites, &mut update, 1);
        let phase_3 = self.process_invites(phase_2, &mut update, 2);
        self.process_invites(phase_3, &mut update, 3);

        update
    }

    fn process_invites(
        &mut self,
        invites: Vec<Invite>,
        update: &mut TopologyUpdate,
        pass: u8,
    ) -> Vec<Invite> {
        let mut deferred_invites = Vec::new();

        for invite in invites {
            if update.is_resolved(invite.source_id) {
                continue;
            }

            match invite.target_id {
                Some(target_id) => {
                    if update.is_resolved(target_id) {
                        update.record_bounce(&invite);
                        continue;
                    }

                    let (source_mass, target_info) = {
                        let source_mass =
                            self.vacuum_state.get_at(invite.source_idx).unwrap().mass();
                        let info = self
                            .vacuum_state
                            .get_monad(target_id)
                            .map(|t| (t.mass(), t.knows(invite.source_id)));
                        (source_mass, info)
                    };

                    match target_info {
                        Some((target_mass, known)) => {
                            // Phase sorting criteria
                            if (known && pass == 1 && source_mass < target_mass)
                                || (!known && pass < 3)
                            {
                                deferred_invites.push(invite);
                                continue;
                            }

                            // Criteria passed! Execute the handshake.
                            let hs = self
                                .vacuum_state
                                .perform_peer_handshake(invite.source_idx, target_id)
                                .unwrap();
                            update.record_success(hs, None);
                        }
                        None => {
                            // Target is dead. Bounce!
                            update.record_bounce(&invite);
                        }
                    }
                }
                None => {
                    // Genesis Invite (Void Target)
                    if pass < 3 {
                        deferred_invites.push(invite);
                    } else {
                        let (hs, newborn) = self
                            .vacuum_state
                            .perform_genesis_handshake(invite.source_idx);
                        update.record_success(hs, Some(newborn));
                    }
                }
            }
        }

        deferred_invites
    }

    fn get_handshake_invite(&self, idx: usize) -> Option<Invite> {
        self.vacuum_state
            .get_at(idx)
            .map(|source| source.get_handshake_invite(idx))
    }
}
