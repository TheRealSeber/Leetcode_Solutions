pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        for i in 0..cost.len()-2 {
            cost[i+2] += cost[i].min(cost[i+1]);
        }
        cost[cost.len()-2].min(cost[cost.len()-1])
    }
}

fn main() {
    println!("Hello, world!");
}
