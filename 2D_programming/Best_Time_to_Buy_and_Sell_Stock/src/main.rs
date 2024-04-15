pub struct Solution {}

#[derive(Default)]
struct State {
    buying: i32,
    selling: i32,
    cooldown: i32
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.into_iter().rev().fold(State::default(), |curr, price| State {
            buying: curr.buying.max(curr.cooldown + price),
            selling: curr.selling.max(curr.buying - price),
            cooldown: curr.selling
        }).selling
    }
}

fn main() {
    println!("Hello, world!");
}
