pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        let min = match self.stack.last() {
            Some(&(_, current_min)) => val.min(current_min),
            None => val,
        };
        self.stack.push((val, min));
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop().map(|(val, _)| val)
    }

    pub fn min(&self) -> Option<i32> {
        self.stack.last().map(|&(_, min)| min)
    }
}
