use crate::models::{handshake::Handshake, invite::Invite, monad::Monad};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct TopologyUpdate {
    connected_ids: HashSet<u32>,
    bounces: HashMap<u32, u32>, // Maps source_id -> target_id
    newborns: Vec<Monad>,
}

impl TopologyUpdate {
    pub fn is_resolved(&self, id: u32) -> bool {
        self.connected_ids.contains(&id)
    }

    pub fn record_success(&mut self, hs: Handshake, newborn_opt: Option<Monad>) {
        self.connected_ids.extend([hs.source_id, hs.target_id]);
        if let Some(newborn) = newborn_opt {
            self.newborns.push(newborn);
        }
    }

    pub fn record_bounce(&mut self, invite: &Invite) {
        self.bounces
            .insert(invite.source_id, invite.target_id.unwrap());
    }

    pub fn bounces(&self) -> impl Iterator<Item = (&u32, &u32)> {
        self.bounces
            .iter()
            .filter(|(source_id, _)| !self.connected_ids.contains(source_id))
    }

    pub fn newborns(self) -> Vec<Monad> {
        self.newborns
    }
}
