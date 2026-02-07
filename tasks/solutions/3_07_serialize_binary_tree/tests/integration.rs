use solution::*;

#[test]
fn test_single_node() {
    let root = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
    let serialized = serialize(&root);
    let deserialized = deserialize(&serialized);
    assert_eq!(deserialized.as_ref().unwrap().val, 1);
    assert!(deserialized.as_ref().unwrap().left.is_none());
    assert!(deserialized.as_ref().unwrap().right.is_none());
}

#[test]
fn test_complete_tree() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
        right: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
    }));
    let serialized = serialize(&root);
    let deserialized = deserialize(&serialized);
    let r = deserialized.as_ref().unwrap();
    assert_eq!(r.val, 1);
    assert_eq!(r.left.as_ref().unwrap().val, 2);
    assert_eq!(r.right.as_ref().unwrap().val, 3);
}

#[test]
fn test_empty_tree() {
    let root: Option<Box<TreeNode>> = None;
    let serialized = serialize(&root);
    let deserialized = deserialize(&serialized);
    assert!(deserialized.is_none());
}

#[test]
fn test_left_skewed_tree() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
            right: None,
        })),
        right: None,
    }));
    let serialized = serialize(&root);
    let deserialized = deserialize(&serialized);
    let r = deserialized.as_ref().unwrap();
    assert_eq!(r.val, 1);
    assert_eq!(r.left.as_ref().unwrap().val, 2);
    assert_eq!(r.left.as_ref().unwrap().left.as_ref().unwrap().val, 3);
    assert!(r.right.is_none());
}

#[test]
fn test_negative_values() {
    let root = Some(Box::new(TreeNode {
        val: -1,
        left: Some(Box::new(TreeNode { val: -2, left: None, right: None })),
        right: Some(Box::new(TreeNode { val: -3, left: None, right: None })),
    }));
    let serialized = serialize(&root);
    let deserialized = deserialize(&serialized);
    let r = deserialized.as_ref().unwrap();
    assert_eq!(r.val, -1);
    assert_eq!(r.left.as_ref().unwrap().val, -2);
    assert_eq!(r.right.as_ref().unwrap().val, -3);
}
