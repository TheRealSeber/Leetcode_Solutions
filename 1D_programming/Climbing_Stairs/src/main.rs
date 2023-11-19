pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n).fold((1, 0), |(res, prev), _|(res + prev, res)).0
    }
}

fn main() {
    println!("Hello, world!");
}
