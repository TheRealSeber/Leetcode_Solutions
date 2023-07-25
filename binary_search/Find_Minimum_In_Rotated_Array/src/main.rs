pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut lp = 0 as i32;
        let mut rp = (nums.len() - 1) as i32;
        while lp <= rp {
            let mp = (lp + rp) / 2;
            if nums[mp as usize] > nums[rp as usize] {
                lp = mp + 1;
            } else {
                rp = mp - 1;
            }
            result = result.min(nums[mp as usize]);
        }
        result
    }
}

fn main() {
    let res = Solution::find_min(vec![11,13,15,17]);
    println!("Hello, world! The result is: {}", res);
}
