use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let map: HashSet<_>= nums.into_iter().collect();
        map.iter()
            .filter(|&&num| !map.contains(&(num-1)))
            .map(|&num| (num..).take_while(|num| map.contains(num)).count())
            .max()
            .unwrap_or(0) as i32
    }
}
fn main() {
    println!("Hello, world!");
}
