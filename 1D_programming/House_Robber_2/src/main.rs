pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => nums[0].max(nums[1]),
            any_size => {
                Self::helper(&nums[..any_size - 1]).max(Self::helper(&nums[1..]))
            }
        }
    }
    fn helper(robs: &[i32]) -> i32 {
        let (mut rob1, mut rob2) = (0, 0);
        for rob in robs {
            let new_rob = rob2.max(rob1 + *rob);
            rob1 = rob2;
            rob2 = new_rob;
        }
        rob2
    }
}

fn main() {
    println!("Hello, world!");
}
