use solution::*;

#[test]
fn test_empty_filter() {
    let bf = BloomFilter::new(1000, 3);
    assert_eq!(bf.might_contain("hello"), false);
}

#[test]
fn test_insert_and_check() {
    let mut bf = BloomFilter::new(1000, 3);
    bf.insert("apple");
    bf.insert("banana");
    assert!(bf.might_contain("apple"));
    assert!(bf.might_contain("banana"));
}

#[test]
fn test_no_false_negatives() {
    let mut bf = BloomFilter::new(10000, 5);
    let items: Vec<String> = (0..100).map(|i| format!("item_{}", i)).collect();
    for item in &items {
        bf.insert(item);
    }
    for item in &items {
        assert!(bf.might_contain(item), "False negative for {}", item);
    }
}

#[test]
fn test_false_positive_rate() {
    let mut bf = BloomFilter::new(10000, 7);
    for i in 0..500 {
        bf.insert(&format!("inserted_{}", i));
    }
    let mut false_positives = 0;
    for i in 0..1000 {
        if bf.might_contain(&format!("not_inserted_{}", i)) {
            false_positives += 1;
        }
    }
    // With 10000 bits, 7 hashes, 500 items, FP rate should be well under 10%
    assert!(false_positives < 100, "Too many false positives: {}", false_positives);
}

#[test]
fn test_different_hash_counts() {
    let mut bf1 = BloomFilter::new(1000, 1);
    let mut bf2 = BloomFilter::new(1000, 10);
    bf1.insert("test");
    bf2.insert("test");
    assert!(bf1.might_contain("test"));
    assert!(bf2.might_contain("test"));
}

#[test]
fn test_empty_string() {
    let mut bf = BloomFilter::new(100, 3);
    bf.insert("");
    assert!(bf.might_contain(""));
}
