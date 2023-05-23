use std::{collections::HashSet};

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut my_hash = HashSet::with_capacity(nums.len());
        for num in &nums {
            match my_hash.contains(num) {
                true => return true,
                false => my_hash.insert(num),
            };
        }
        false
    }
}