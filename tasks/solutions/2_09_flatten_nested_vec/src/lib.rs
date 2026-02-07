pub fn flatten(nested: Vec<Vec<i32>>) -> Vec<i32> {
    nested.into_iter().flatten().collect()
}
