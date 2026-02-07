pub enum TaskState {
    Yield,
    Complete(i64),
}

pub struct TaskContext {
    pub emitted: Vec<i64>,
}

impl TaskContext {
    pub fn emit(&mut self, val: i64) {
        self.emitted.push(val);
    }
}

struct Task {
    priority: u8,
    spawn_order: usize,
    func: Box<dyn FnMut(&mut TaskContext) -> TaskState>,
    context: TaskContext,
}

pub struct Scheduler {
    tasks: Vec<Task>,
    next_id: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, priority: u8, task: Box<dyn FnMut(&mut TaskContext) -> TaskState>) {
        self.tasks.push(Task {
            priority,
            spawn_order: self.next_id,
            func: task,
            context: TaskContext {
                emitted: Vec::new(),
            },
        });
        self.next_id += 1;
    }

    pub fn run(&mut self) -> Vec<i64> {
        let mut results = Vec::new();

        loop {
            if self.tasks.is_empty() {
                break;
            }

            // Find the highest-priority task (ties broken by spawn_order, FIFO)
            let mut best_idx = 0;
            for i in 1..self.tasks.len() {
                if self.tasks[i].priority > self.tasks[best_idx].priority
                    || (self.tasks[i].priority == self.tasks[best_idx].priority
                        && self.tasks[i].spawn_order < self.tasks[best_idx].spawn_order)
                {
                    best_idx = i;
                }
            }

            let task = &mut self.tasks[best_idx];
            let state = (task.func)(&mut task.context);

            // Drain emitted values
            results.append(&mut task.context.emitted);

            match state {
                TaskState::Yield => {
                    // Task stays in the queue
                }
                TaskState::Complete(val) => {
                    results.push(val);
                    self.tasks.remove(best_idx);
                }
            }
        }

        results
    }
}
