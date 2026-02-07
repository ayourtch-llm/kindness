pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub fn serialize(root: &Option<Box<TreeNode>>) -> String {
    let mut result = String::new();
    serialize_helper(root, &mut result);
    result
}

fn serialize_helper(node: &Option<Box<TreeNode>>, result: &mut String) {
    match node {
        None => {
            if !result.is_empty() {
                result.push(',');
            }
            result.push_str("null");
        }
        Some(n) => {
            if !result.is_empty() {
                result.push(',');
            }
            result.push_str(&n.val.to_string());
            serialize_helper(&n.left, result);
            serialize_helper(&n.right, result);
        }
    }
}

pub fn deserialize(data: &str) -> Option<Box<TreeNode>> {
    let tokens: Vec<&str> = data.split(',').collect();
    let mut pos = 0;
    deserialize_helper(&tokens, &mut pos)
}

fn deserialize_helper(tokens: &[&str], pos: &mut usize) -> Option<Box<TreeNode>> {
    if *pos >= tokens.len() {
        return None;
    }
    let token = tokens[*pos].trim();
    *pos += 1;
    if token == "null" {
        return None;
    }
    let val: i32 = token.parse().unwrap();
    let left = deserialize_helper(tokens, pos);
    let right = deserialize_helper(tokens, pos);
    Some(Box::new(TreeNode { val, left, right }))
}
