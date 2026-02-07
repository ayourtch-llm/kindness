use solution::*;

#[test]
fn single_task_completes() {
    let mut sched = Scheduler::new();
    sched.spawn(1, Box::new(|ctx: &mut TaskContext| {
        ctx.emit(42);
        TaskState::Complete(100)
    }));
    let results = sched.run();
    assert_eq!(results, vec![42, 100]);
}

#[test]
fn priority_ordering() {
    let mut sched = Scheduler::new();
    sched.spawn(1, Box::new(|ctx: &mut TaskContext| {
        ctx.emit(1);
        TaskState::Complete(10)
    }));
    sched.spawn(3, Box::new(|ctx: &mut TaskContext| {
        ctx.emit(3);
        TaskState::Complete(30)
    }));
    sched.spawn(2, Box::new(|ctx: &mut TaskContext| {
        ctx.emit(2);
        TaskState::Complete(20)
    }));
    let results = sched.run();
    // Highest priority runs first: 3, then 2, then 1
    assert_eq!(results, vec![3, 30, 2, 20, 1, 10]);
}

#[test]
fn yielding_tasks() {
    let mut sched = Scheduler::new();
    let mut count = 0;
    sched.spawn(1, Box::new(move |ctx: &mut TaskContext| {
        count += 1;
        ctx.emit(count);
        if count >= 3 {
            TaskState::Complete(count * 10)
        } else {
            TaskState::Yield
        }
    }));
    let results = sched.run();
    assert_eq!(results, vec![1, 2, 3, 30]);
}

#[test]
fn interleaved_by_priority() {
    let mut sched = Scheduler::new();
    let mut hi_count = 0;
    let mut lo_count = 0;
    sched.spawn(1, Box::new(move |ctx: &mut TaskContext| {
        lo_count += 1;
        ctx.emit(lo_count * 100);
        if lo_count >= 2 { TaskState::Complete(0) } else { TaskState::Yield }
    }));
    sched.spawn(2, Box::new(move |ctx: &mut TaskContext| {
        hi_count += 1;
        ctx.emit(hi_count);
        if hi_count >= 2 { TaskState::Complete(0) } else { TaskState::Yield }
    }));
    let results = sched.run();
    // Round 1: hi(p=2) emits 1 yields, Round 2: hi(p=2) emits 2 completes(0),
    // Round 3: lo(p=1) emits 100 yields, Round 4: lo(p=1) emits 200 completes(0)
    assert_eq!(results, vec![1, 2, 0, 100, 200, 0]);
}

#[test]
fn same_priority_fifo() {
    let mut sched = Scheduler::new();
    sched.spawn(1, Box::new(|ctx: &mut TaskContext| {
        ctx.emit(1);
        TaskState::Complete(0)
    }));
    sched.spawn(1, Box::new(|ctx: &mut TaskContext| {
        ctx.emit(2);
        TaskState::Complete(0)
    }));
    sched.spawn(1, Box::new(|ctx: &mut TaskContext| {
        ctx.emit(3);
        TaskState::Complete(0)
    }));
    let results = sched.run();
    // Same priority -> FIFO order
    assert_eq!(results, vec![1, 0, 2, 0, 3, 0]);
}

#[test]
fn empty_scheduler() {
    let mut sched = Scheduler::new();
    let results = sched.run();
    assert!(results.is_empty());
}
