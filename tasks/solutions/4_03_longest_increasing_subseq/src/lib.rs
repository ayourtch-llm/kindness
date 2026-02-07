pub fn lis(nums: &[i32]) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut tails: Vec<i32> = Vec::new();
    for &num in nums {
        match tails.binary_search(&num) {
            Ok(_) => {} // duplicate, do nothing (strictly increasing)
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(num);
                } else {
                    tails[pos] = num;
                }
            }
        }
    }
    tails.len()
}
