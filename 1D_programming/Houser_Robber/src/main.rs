pub struct Solution {}

impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => nums[0].max(nums[1]),
            3 => (nums[0] + nums[2]).max(nums[1]),
            _ => {
                nums[2] += nums[0];
                for i in 3..nums.len() {
                    nums[i] += nums[i-3].max(nums[i-2]);
                    nums[i-2] = nums[i-2].max(nums[i-3]);
                }
                nums[nums.len() - 1].max(nums[nums.len() - 2])
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
