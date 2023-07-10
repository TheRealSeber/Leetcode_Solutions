pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lp = 0 as i32;
        let mut rp = (nums.len() - 1) as i32;
        while lp <= rp {
            let mp = lp + ((rp - lp)/2);
            if nums[mp as usize] > target {
                rp = mp - 1
            } else if nums[mp as usize] < target {
                lp = mp + 1;
            } else {
                return mp as i32;
            }
        }
        -1
    }
}

fn main() {
    let res = Solution::search(vec![-1,0,3,3,6,5,7,12], 9);
    println!("Hello, world! The result is: {}", res);
}
