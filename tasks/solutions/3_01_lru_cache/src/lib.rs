use std::collections::HashMap;

pub struct LruCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    entries: Vec<(i32, i32)>,
    order: Vec<i32>,
}

impl LruCache {
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: HashMap::new(),
            entries: Vec::new(),
            order: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &i32) -> Option<&i32> {
        if self.map.contains_key(key) {
            self.touch(*key);
            let idx = self.map[key];
            Some(&self.entries[idx].1)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            let idx = self.map[&key];
            self.entries[idx].1 = value;
            self.touch(key);
        } else {
            if self.map.len() >= self.capacity {
                let lru_key = self.order.remove(0);
                let idx = self.map.remove(&lru_key).unwrap();
                self.entries[idx] = (key, value);
                self.map.insert(key, idx);
                self.order.push(key);
            } else {
                let idx = self.entries.len();
                self.entries.push((key, value));
                self.map.insert(key, idx);
                self.order.push(key);
            }
        }
    }

    fn touch(&mut self, key: i32) {
        if let Some(pos) = self.order.iter().position(|&k| k == key) {
            self.order.remove(pos);
            self.order.push(key);
        }
    }
}
