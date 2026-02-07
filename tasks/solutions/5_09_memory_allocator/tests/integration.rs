use solution::*;

#[test]
fn basic_alloc_and_free() {
    let mut alloc = Allocator::new(1024);
    assert_eq!(alloc.available(), 1024);

    let a = alloc.alloc(100).unwrap();
    assert_eq!(a, 0);
    assert_eq!(alloc.available(), 924);

    alloc.free(a);
    assert_eq!(alloc.available(), 1024);
}

#[test]
fn multiple_allocations() {
    let mut alloc = Allocator::new(1000);
    let a = alloc.alloc(200).unwrap();
    let b = alloc.alloc(300).unwrap();
    let c = alloc.alloc(400).unwrap();

    assert_eq!(a, 0);
    assert_eq!(b, 200);
    assert_eq!(c, 500);
    assert_eq!(alloc.available(), 100);
}

#[test]
fn coalescing() {
    let mut alloc = Allocator::new(1000);
    let a = alloc.alloc(300).unwrap();
    let b = alloc.alloc(300).unwrap();
    let c = alloc.alloc(300).unwrap();
    assert_eq!(alloc.available(), 100);

    // Free middle block, then neighbors
    alloc.free(b);
    assert_eq!(alloc.available(), 400);
    assert_eq!(alloc.largest_free_block(), 300);

    alloc.free(a);
    // a and b should coalesce: 600 contiguous from offset 0
    assert_eq!(alloc.available(), 700);
    assert_eq!(alloc.largest_free_block(), 600);

    alloc.free(c);
    // Everything coalesced
    assert_eq!(alloc.available(), 1000);
    assert_eq!(alloc.largest_free_block(), 1000);
}

#[test]
fn allocation_failure() {
    let mut alloc = Allocator::new(100);
    let _ = alloc.alloc(60).unwrap();
    let _ = alloc.alloc(30).unwrap();
    assert_eq!(alloc.available(), 10);
    assert!(alloc.alloc(20).is_none());
    assert_eq!(alloc.alloc(0), None);
}

#[test]
fn first_fit_strategy() {
    let mut alloc = Allocator::new(1000);
    let a = alloc.alloc(200).unwrap(); // 0..200
    let b = alloc.alloc(200).unwrap(); // 200..400
    let c = alloc.alloc(200).unwrap(); // 400..600

    alloc.free(a); // free 0..200
    alloc.free(c); // free 400..600

    // First fit should use offset 0 (first free block)
    let d = alloc.alloc(100).unwrap();
    assert_eq!(d, 0);

    // Next alloc of 100 fits in remaining of first hole (offset 100)
    let e = alloc.alloc(100).unwrap();
    assert_eq!(e, 100);
}

#[test]
fn free_invalid_offset() {
    let mut alloc = Allocator::new(100);
    let a = alloc.alloc(50).unwrap();
    // Freeing an invalid offset should be a no-op
    alloc.free(999);
    assert_eq!(alloc.available(), 50);
    // Double free should be a no-op
    alloc.free(a);
    alloc.free(a);
    assert_eq!(alloc.available(), 100);
}
