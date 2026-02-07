use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashRing {
    ring: BTreeMap<u64, String>,
    replicas: usize,
}

impl HashRing {
    pub fn new(replicas: usize) -> Self {
        Self {
            ring: BTreeMap::new(),
            replicas,
        }
    }

    fn hash_key(key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    pub fn add_node(&mut self, node: &str) {
        for i in 0..self.replicas {
            let virtual_key = format!("{}#{}", node, i);
            let hash = Self::hash_key(&virtual_key);
            self.ring.insert(hash, node.to_string());
        }
    }

    pub fn remove_node(&mut self, node: &str) {
        for i in 0..self.replicas {
            let virtual_key = format!("{}#{}", node, i);
            let hash = Self::hash_key(&virtual_key);
            self.ring.remove(&hash);
        }
    }

    pub fn get_node(&self, key: &str) -> Option<&str> {
        if self.ring.is_empty() {
            return None;
        }
        let hash = Self::hash_key(key);
        // Walk clockwise: find first entry >= hash
        let node = self
            .ring
            .range(hash..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, v)| v.as_str());
        node
    }
}
