use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bits: Vec<bool>,
    size: usize,
    num_hashes: usize,
}

impl BloomFilter {
    pub fn new(size: usize, num_hashes: usize) -> Self {
        Self {
            bits: vec![false; size],
            size,
            num_hashes,
        }
    }

    fn hashes(&self, item: &str) -> Vec<usize> {
        let mut h1 = DefaultHasher::new();
        item.hash(&mut h1);
        let hash1 = h1.finish();

        let mut h2 = DefaultHasher::new();
        item.len().hash(&mut h2);
        item.hash(&mut h2);
        42u64.hash(&mut h2);
        let hash2 = h2.finish();

        (0..self.num_hashes)
            .map(|i| ((hash1.wrapping_add((i as u64).wrapping_mul(hash2))) % self.size as u64) as usize)
            .collect()
    }

    pub fn insert(&mut self, item: &str) {
        for idx in self.hashes(item) {
            self.bits[idx] = true;
        }
    }

    pub fn might_contain(&self, item: &str) -> bool {
        self.hashes(item).iter().all(|&idx| self.bits[idx])
    }
}
