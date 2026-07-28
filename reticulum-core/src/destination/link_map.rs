use alloc::collections::BTreeMap;

use crate::hash::AddressHash;

use super::link::LinkId;

#[derive(Default)]
pub struct LinkMap {
    map: BTreeMap<AddressHash, LinkId>,
}

impl LinkMap {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn resolve(&self, address: &AddressHash) -> Option<LinkId> {
        self.map.get(address).copied()
    }

    pub fn insert(&mut self, address: &AddressHash, id: &LinkId) {
        self.map.insert(*address, *id);
    }

    pub fn remove(&mut self, address: &AddressHash) {
        self.map.remove(address);
    }
}
