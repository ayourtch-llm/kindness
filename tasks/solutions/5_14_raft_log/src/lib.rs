#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub term: u64,
    pub index: u64,
    pub data: Vec<u8>,
}

pub struct RaftNode {
    #[allow(dead_code)]
    id: u64,
    current_term: u64,
    log: Vec<LogEntry>,
    commit_index: u64,
}

impl RaftNode {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            current_term: 0,
            log: Vec::new(),
            commit_index: 0,
        }
    }

    pub fn current_term(&self) -> u64 {
        self.current_term
    }

    pub fn set_term(&mut self, term: u64) {
        self.current_term = term;
    }

    pub fn append_entry(&mut self, data: Vec<u8>) -> u64 {
        let index = self.log.len() as u64 + 1;
        self.log.push(LogEntry {
            term: self.current_term,
            index,
            data,
        });
        index
    }

    pub fn receive_append_entries(
        &mut self,
        leader_term: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
    ) -> bool {
        if leader_term < self.current_term {
            return false;
        }

        if prev_log_index > 0 {
            if prev_log_index as usize > self.log.len() {
                return false;
            }
            let entry = &self.log[(prev_log_index - 1) as usize];
            if entry.term != prev_log_term {
                return false;
            }
        }

        if leader_term > self.current_term {
            self.current_term = leader_term;
        }

        for entry in entries {
            let idx = entry.index as usize;
            if idx <= self.log.len() {
                // Overwrite if conflicting
                if self.log[idx - 1].term != entry.term {
                    self.log.truncate(idx - 1);
                    self.log.push(entry);
                }
                // If same term, keep existing (or overwrite - doesn't matter, same data expected)
            } else {
                self.log.push(entry);
            }
        }

        true
    }

    pub fn commit_up_to(&mut self, index: u64) {
        let last = self.last_log_index();
        self.commit_index = index.min(last);
    }

    pub fn get_committed(&self) -> Vec<&LogEntry> {
        self.log.iter().take(self.commit_index as usize).collect()
    }

    pub fn last_log_index(&self) -> u64 {
        self.log.len() as u64
    }

    pub fn last_log_term(&self) -> u64 {
        self.log.last().map_or(0, |e| e.term)
    }
}
