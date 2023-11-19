pub struct Solution {}

impl Solution {
    pub fn max_product( nums: Vec<i32>) -> i32 {
        nums.into_iter().fold((1, 1, i32::MIN), |(c_min, c_max, max), x| {
            if x == 0 {
                    (1, 1, max.max(0))
            } else {
                let (temp1, temp2) = (c_min * x, c_max * x);
                let new_max = temp1.max(temp2).max(x);
                (temp1.min(temp2).min(x), new_max, max.max(new_max))
            }
        }).2
    }
}

fn main() {
    println!("Hello, world!");
}
