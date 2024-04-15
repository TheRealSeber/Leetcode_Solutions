pub struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        fn dfs(candidates: &[i32], curr_sum: i32, res: &mut i32, target: i32) {
            if candidates.is_empty() {
                if curr_sum == target {
                    *res += 1;
                }
            } else {
                dfs(&candidates[1..], curr_sum + *candidates.first().expect("Should exist"), res, target);
                dfs(&candidates[1..], curr_sum - *candidates.first().expect("Should exist"), res, target);
            }
        }
        dfs(&nums, 0, &mut res, target);
        res
    }
}

fn main() {
    println!("Hello, world!");
}
