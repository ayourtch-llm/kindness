use solution::*;

#[test]
fn test_initial_full() {
    let mut rl = RateLimiter::new(10.0, 5.0);
    // Bucket starts full with 5 tokens
    for _ in 0..5 {
        assert!(rl.try_acquire());
    }
    // 6th should fail
    assert!(!rl.try_acquire());
}

#[test]
fn test_refill() {
    use std::thread;
    use std::time::Duration;

    let mut rl = RateLimiter::new(10.0, 5.0);
    // Drain all tokens
    for _ in 0..5 {
        rl.try_acquire();
    }
    assert!(!rl.try_acquire());

    // Wait 200ms => should gain ~2 tokens (10 tokens/sec * 0.2s)
    thread::sleep(Duration::from_millis(200));
    assert!(rl.try_acquire());
}

#[test]
fn test_acquire_n() {
    let mut rl = RateLimiter::new(100.0, 10.0);
    // Start with 10 tokens, try to acquire 5
    assert!(rl.try_acquire_n(5.0));
    // 5 remaining, try to acquire 6 => fail (no partial)
    assert!(!rl.try_acquire_n(6.0));
    // 5 still remaining (atomic), acquire 5 => success
    assert!(rl.try_acquire_n(5.0));
}

#[test]
fn test_capacity_cap() {
    use std::thread;
    use std::time::Duration;

    let mut rl = RateLimiter::new(1000.0, 3.0);
    // Starts full at 3
    // Even after waiting, should not exceed capacity
    thread::sleep(Duration::from_millis(100));
    // Should be capped at 3
    assert!(rl.try_acquire_n(3.0));
    assert!(!rl.try_acquire());
}

#[test]
fn test_zero_tokens_requested() {
    let mut rl = RateLimiter::new(1.0, 1.0);
    // Acquiring 0 tokens should always succeed
    assert!(rl.try_acquire_n(0.0));
}

#[test]
fn test_high_rate() {
    use std::thread;
    use std::time::Duration;

    let mut rl = RateLimiter::new(1000.0, 100.0);
    // Drain
    for _ in 0..100 {
        rl.try_acquire();
    }
    assert!(!rl.try_acquire());
    // Wait 50ms => ~50 tokens
    thread::sleep(Duration::from_millis(50));
    assert!(rl.try_acquire_n(30.0));
}
