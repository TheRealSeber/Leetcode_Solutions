pub struct Solution {}
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut my_hash = std::collections::HashSet::with_capacity(nums.len());
        for v in nums.into_iter() {
            if !my_hash.insert(v) {
                return v;
            }
        };
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
