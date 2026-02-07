pub fn rotate_left(arr: &mut [i32], k: usize) {
    let len = arr.len();
    if len == 0 {
        return;
    }
    let k = k % len;
    arr.rotate_left(k);
}
