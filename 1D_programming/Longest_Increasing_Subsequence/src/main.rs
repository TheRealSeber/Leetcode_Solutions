pub struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for j in 1..nums.len() {
            let mut max = 0;
            for i in 0..j {
                if nums[i] > nums[j] && dp[i] > max {
                    max = dp[i];
                }
            }
            dp[j] = max + 1;
        }
        dp[nums.len() - 1]
    }
}

fn main() {
    println!("Hello, world!");
}
