use solution::*;

#[test]
fn test_basic_put_and_get() {
    let mut cache = LruCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.get(&1), Some(&10));
    assert_eq!(cache.get(&2), Some(&20));
}

#[test]
fn test_eviction_on_capacity() {
    let mut cache = LruCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&20));
    assert_eq!(cache.get(&3), Some(&30));
}

#[test]
fn test_access_refreshes_order() {
    let mut cache = LruCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    let _ = cache.get(&1);
    cache.put(3, 30);
    assert_eq!(cache.get(&1), Some(&10));
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&3), Some(&30));
}

#[test]
fn test_update_existing_key() {
    let mut cache = LruCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(1, 100);
    assert_eq!(cache.get(&1), Some(&100));
    cache.put(3, 30);
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&1), Some(&100));
    assert_eq!(cache.get(&3), Some(&30));
}

#[test]
fn test_capacity_one() {
    let mut cache = LruCache::new(1);
    cache.put(1, 10);
    assert_eq!(cache.get(&1), Some(&10));
    cache.put(2, 20);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&20));
}
