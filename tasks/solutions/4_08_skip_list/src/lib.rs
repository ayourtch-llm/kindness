use std::cell::RefCell;

const MAX_LEVEL: usize = 16;

thread_local! {
    static RNG_STATE: RefCell<u64> = RefCell::new(12345);
}

fn random_level() -> usize {
    RNG_STATE.with(|state| {
        let mut level = 1;
        let mut s = *state.borrow();
        while level < MAX_LEVEL {
            // xorshift64
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            if s % 2 == 0 {
                level += 1;
            } else {
                break;
            }
        }
        *state.borrow_mut() = s;
        level
    })
}

struct Node {
    val: i32,
    forward: Vec<Option<usize>>,
}

pub struct SkipList {
    nodes: Vec<Node>,
    head: usize,
    level: usize,
    length: usize,
}

impl SkipList {
    pub fn new() -> Self {
        let head_node = Node {
            val: i32::MIN,
            forward: vec![None; MAX_LEVEL],
        };
        Self {
            nodes: vec![head_node],
            head: 0,
            level: 1,
            length: 0,
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        let mut update = vec![self.head; MAX_LEVEL];
        let mut current = self.head;

        for i in (0..self.level).rev() {
            while let Some(next) = self.nodes[current].forward[i] {
                if self.nodes[next].val < val {
                    current = next;
                } else {
                    break;
                }
            }
            update[i] = current;
        }

        // Check if already exists
        if let Some(next) = self.nodes[current].forward[0] {
            if self.nodes[next].val == val {
                return false;
            }
        }

        let new_level = random_level();
        if new_level > self.level {
            for i in self.level..new_level {
                update[i] = self.head;
            }
            self.level = new_level;
        }

        let new_node = Node {
            val,
            forward: vec![None; new_level],
        };
        let new_idx = self.nodes.len();
        self.nodes.push(new_node);

        for i in 0..new_level {
            self.nodes[new_idx].forward[i] = self.nodes[update[i]].forward[i];
            self.nodes[update[i]].forward[i] = Some(new_idx);
        }

        self.length += 1;
        true
    }

    pub fn contains(&self, val: i32) -> bool {
        let mut current = self.head;
        for i in (0..self.level).rev() {
            while let Some(next) = self.nodes[current].forward[i] {
                if self.nodes[next].val < val {
                    current = next;
                } else if self.nodes[next].val == val {
                    return true;
                } else {
                    break;
                }
            }
        }
        false
    }

    pub fn remove(&mut self, val: i32) -> bool {
        let mut update = vec![self.head; MAX_LEVEL];
        let mut current = self.head;

        for i in (0..self.level).rev() {
            while let Some(next) = self.nodes[current].forward[i] {
                if self.nodes[next].val < val {
                    current = next;
                } else {
                    break;
                }
            }
            update[i] = current;
        }

        let target = self.nodes[current].forward[0];
        if let Some(target_idx) = target {
            if self.nodes[target_idx].val != val {
                return false;
            }
            let target_level = self.nodes[target_idx].forward.len();
            for i in 0..target_level {
                if self.nodes[update[i]].forward[i] == Some(target_idx) {
                    self.nodes[update[i]].forward[i] = self.nodes[target_idx].forward[i];
                }
            }
            while self.level > 1 && self.nodes[self.head].forward[self.level - 1].is_none() {
                self.level -= 1;
            }
            self.length -= 1;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}
