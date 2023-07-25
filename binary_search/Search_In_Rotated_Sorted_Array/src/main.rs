pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lp = 0 as i32;
        let mut rp = (nums.len() - 1) as i32;
        while lp <= rp {
            let mp = (lp + rp) / 2;
            if nums[mp as usize] == target {
                return mp;
            }
            if nums[mp as usize] > nums[lp as usize] {
                if nums[lp as usize] > target {
                    lp = mp + 1;
                } else {
                    rp = mp - 1;
                }
            } else {
                if nums[rp as usize] < target {
                    rp = mp - 1;
                } else {
                    lp = mp + 1;
                }
            }
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}
