use solution::*;

#[test]
fn test_insert_and_search() {
    let mut trie = Trie::new();
    trie.insert("apple");
    assert!(trie.search("apple"));
    assert!(!trie.search("app"));
}

#[test]
fn test_starts_with() {
    let mut trie = Trie::new();
    trie.insert("apple");
    assert!(trie.starts_with("app"));
    assert!(trie.starts_with("apple"));
    assert!(!trie.starts_with("b"));
}

#[test]
fn test_multiple_words() {
    let mut trie = Trie::new();
    trie.insert("apple");
    trie.insert("app");
    assert!(trie.search("app"));
    assert!(trie.search("apple"));
    assert!(!trie.search("ap"));
}

#[test]
fn test_empty_trie() {
    let trie = Trie::new();
    assert!(!trie.search("anything"));
    assert!(!trie.starts_with("a"));
}

#[test]
fn test_overlapping_prefixes() {
    let mut trie = Trie::new();
    trie.insert("car");
    trie.insert("card");
    trie.insert("care");
    trie.insert("cared");
    assert!(trie.search("car"));
    assert!(trie.search("card"));
    assert!(trie.search("care"));
    assert!(trie.search("cared"));
    assert!(!trie.search("cars"));
    assert!(trie.starts_with("car"));
    assert!(trie.starts_with("care"));
}
