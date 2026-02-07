use std::collections::BTreeMap;

pub struct Allocator {
    free_blocks: Vec<(usize, usize)>, // (offset, size), sorted by offset
    allocated: BTreeMap<usize, usize>, // offset -> size
    total_size: usize,
}

impl Allocator {
    pub fn new(size: usize) -> Self {
        Allocator {
            free_blocks: vec![(0, size)],
            allocated: BTreeMap::new(),
            total_size: size,
        }
    }

    pub fn alloc(&mut self, size: usize) -> Option<usize> {
        if size == 0 {
            return None;
        }

        // First-fit: find first free block large enough
        for i in 0..self.free_blocks.len() {
            let (offset, block_size) = self.free_blocks[i];
            if block_size >= size {
                let alloc_offset = offset;
                if block_size == size {
                    self.free_blocks.remove(i);
                } else {
                    self.free_blocks[i] = (offset + size, block_size - size);
                }
                self.allocated.insert(alloc_offset, size);
                return Some(alloc_offset);
            }
        }

        None
    }

    pub fn free(&mut self, offset: usize) {
        let size = match self.allocated.remove(&offset) {
            Some(s) => s,
            None => return, // invalid offset or double free
        };

        // Insert the freed block back into the free list (sorted by offset)
        let new_block = (offset, size);
        let pos = self
            .free_blocks
            .binary_search_by_key(&offset, |&(o, _)| o)
            .unwrap_or_else(|i| i);
        self.free_blocks.insert(pos, new_block);

        // Coalesce with next block
        if pos + 1 < self.free_blocks.len() {
            let (cur_off, cur_sz) = self.free_blocks[pos];
            let (next_off, next_sz) = self.free_blocks[pos + 1];
            if cur_off + cur_sz == next_off {
                self.free_blocks[pos] = (cur_off, cur_sz + next_sz);
                self.free_blocks.remove(pos + 1);
            }
        }

        // Coalesce with previous block
        if pos > 0 {
            let (prev_off, prev_sz) = self.free_blocks[pos - 1];
            let (cur_off, cur_sz) = self.free_blocks[pos];
            if prev_off + prev_sz == cur_off {
                self.free_blocks[pos - 1] = (prev_off, prev_sz + cur_sz);
                self.free_blocks.remove(pos);
            }
        }
    }

    pub fn available(&self) -> usize {
        self.free_blocks.iter().map(|(_, s)| s).sum()
    }

    pub fn largest_free_block(&self) -> usize {
        self.free_blocks.iter().map(|(_, s)| *s).max().unwrap_or(0)
    }
}
