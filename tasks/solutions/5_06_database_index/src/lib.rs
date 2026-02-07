#[derive(Debug, Clone)]
struct Node<K: Ord + Clone, V: Clone> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Node<K, V>>,
}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    fn new_leaf() -> Self {
        Node {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn get(&self, key: &K) -> Option<&V> {
        match self.keys.binary_search(key) {
            Ok(idx) => Some(&self.values[idx]),
            Err(idx) => {
                if self.is_leaf() {
                    None
                } else {
                    self.children[idx].get(key)
                }
            }
        }
    }

    fn insert(&mut self, key: K, value: V, order: usize) -> (Option<(K, V, Node<K, V>)>, bool) {
        let pos = match self.keys.binary_search(&key) {
            Ok(idx) => {
                self.values[idx] = value;
                return (None, false); // updated existing, not new
            }
            Err(idx) => idx,
        };

        if self.is_leaf() {
            self.keys.insert(pos, key);
            self.values.insert(pos, value);
            let split = if self.keys.len() >= order {
                Some(self.split())
            } else {
                None
            };
            (split, true)
        } else {
            let (result, is_new) = self.children[pos].insert(key, value, order);
            if let Some((median_key, median_val, right_child)) = result {
                let ins_pos = match self.keys.binary_search(&median_key) {
                    Ok(i) | Err(i) => i,
                };
                self.keys.insert(ins_pos, median_key);
                self.values.insert(ins_pos, median_val);
                self.children.insert(ins_pos + 1, right_child);

                if self.keys.len() >= order {
                    return (Some(self.split()), is_new);
                }
            }
            (None, is_new)
        }
    }

    fn split(&mut self) -> (K, V, Node<K, V>) {
        let mid = self.keys.len() / 2;
        let median_key = self.keys.remove(mid);
        let median_val = self.values.remove(mid);
        let right_keys = self.keys.split_off(mid);
        let right_values = self.values.split_off(mid);
        let right_children = if !self.children.is_empty() {
            self.children.split_off(mid + 1)
        } else {
            Vec::new()
        };
        (
            median_key,
            median_val,
            Node {
                keys: right_keys,
                values: right_values,
                children: right_children,
            },
        )
    }

    fn remove(&mut self, key: &K, order: usize) -> Option<V> {
        match self.keys.binary_search(key) {
            Ok(idx) => {
                if self.is_leaf() {
                    self.keys.remove(idx);
                    Some(self.values.remove(idx))
                } else {
                    let pred = self.children[idx].remove_largest(order);
                    let old_val = std::mem::replace(&mut self.values[idx], pred.1);
                    self.keys[idx] = pred.0;
                    self.fix_child(idx, order);
                    Some(old_val)
                }
            }
            Err(idx) => {
                if self.is_leaf() {
                    return None;
                }
                let result = self.children[idx].remove(key, order);
                if result.is_some() {
                    self.fix_child(idx, order);
                }
                result
            }
        }
    }

    fn remove_largest(&mut self, order: usize) -> (K, V) {
        if self.is_leaf() {
            let k = self.keys.pop().unwrap();
            let v = self.values.pop().unwrap();
            (k, v)
        } else {
            let last = self.children.len() - 1;
            let result = self.children[last].remove_largest(order);
            self.fix_child(last, order);
            result
        }
    }

    fn fix_child(&mut self, child_idx: usize, order: usize) {
        let min_keys = (order - 1) / 2;
        if self.children[child_idx].keys.len() >= min_keys {
            return;
        }

        // Try borrow from left sibling
        if child_idx > 0 && self.children[child_idx - 1].keys.len() > min_keys {
            let sep_idx = child_idx - 1;
            let sep_key = self.keys[sep_idx].clone();
            let sep_val = self.values[sep_idx].clone();

            let borrowed_key = self.children[sep_idx].keys.pop().unwrap();
            let borrowed_val = self.children[sep_idx].values.pop().unwrap();
            let borrowed_child = if !self.children[sep_idx].children.is_empty() {
                Some(self.children[sep_idx].children.pop().unwrap())
            } else {
                None
            };

            self.keys[sep_idx] = borrowed_key;
            self.values[sep_idx] = borrowed_val;

            self.children[child_idx].keys.insert(0, sep_key);
            self.children[child_idx].values.insert(0, sep_val);
            if let Some(c) = borrowed_child {
                self.children[child_idx].children.insert(0, c);
            }
            return;
        }

        // Try borrow from right sibling
        if child_idx < self.children.len() - 1
            && self.children[child_idx + 1].keys.len() > min_keys
        {
            let sep_idx = child_idx;
            let sep_key = self.keys[sep_idx].clone();
            let sep_val = self.values[sep_idx].clone();

            let borrowed_key = self.children[child_idx + 1].keys.remove(0);
            let borrowed_val = self.children[child_idx + 1].values.remove(0);
            let borrowed_child = if !self.children[child_idx + 1].children.is_empty() {
                Some(self.children[child_idx + 1].children.remove(0))
            } else {
                None
            };

            self.keys[sep_idx] = borrowed_key;
            self.values[sep_idx] = borrowed_val;

            self.children[child_idx].keys.push(sep_key);
            self.children[child_idx].values.push(sep_val);
            if let Some(c) = borrowed_child {
                self.children[child_idx].children.push(c);
            }
            return;
        }

        // Merge
        if child_idx > 0 {
            let sep_idx = child_idx - 1;
            let sep_key = self.keys.remove(sep_idx);
            let sep_val = self.values.remove(sep_idx);
            let right = self.children.remove(child_idx);
            let left = &mut self.children[sep_idx];
            left.keys.push(sep_key);
            left.values.push(sep_val);
            left.keys.extend(right.keys);
            left.values.extend(right.values);
            left.children.extend(right.children);
        } else {
            let sep_key = self.keys.remove(0);
            let sep_val = self.values.remove(0);
            let right = self.children.remove(1);
            let left = &mut self.children[0];
            left.keys.push(sep_key);
            left.values.push(sep_val);
            left.keys.extend(right.keys);
            left.values.extend(right.values);
            left.children.extend(right.children);
        }
    }

    fn range_collect<'a>(
        &'a self,
        start: &K,
        end: &K,
        result: &mut Vec<(&'a K, &'a V)>,
    ) {
        for i in 0..self.keys.len() {
            if !self.is_leaf() && self.keys[i] >= *start {
                self.children[i].range_collect(start, end, result);
            }
            if self.keys[i] >= *start && self.keys[i] <= *end {
                result.push((&self.keys[i], &self.values[i]));
            }
            if self.keys[i] > *end {
                return;
            }
        }
        if !self.is_leaf() {
            let last = self.children.len() - 1;
            if self.keys.last().map_or(true, |k| k <= end) {
                self.children[last].range_collect(start, end, result);
            }
        }
    }
}

pub struct BTreeIndex<K: Ord + Clone, V: Clone> {
    root: Node<K, V>,
    order: usize,
    len: usize,
}

impl<K: Ord + Clone, V: Clone> BTreeIndex<K, V> {
    pub fn new(order: usize) -> Self {
        let order = if order < 3 { 3 } else { order };
        BTreeIndex {
            root: Node::new_leaf(),
            order,
            len: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let order = self.order;
        let (result, is_new) = self.root.insert(key, value, order);
        if is_new {
            self.len += 1;
        }
        if let Some((median_key, median_val, right_node)) = result {
            let old_root = std::mem::replace(&mut self.root, Node::new_leaf());
            self.root.keys.push(median_key);
            self.root.values.push(median_val);
            self.root.children.push(old_root);
            self.root.children.push(right_node);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let order = self.order;
        let result = self.root.remove(key, order);
        if result.is_some() {
            self.len -= 1;
            if self.root.keys.is_empty() && !self.root.children.is_empty() {
                self.root = self.root.children.remove(0);
            }
        }
        result
    }

    pub fn range(&self, start: &K, end: &K) -> Vec<(&K, &V)> {
        let mut result = Vec::new();
        self.root.range_collect(start, end, &mut result);
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
