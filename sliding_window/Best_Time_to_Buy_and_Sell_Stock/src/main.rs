pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut min_val, mut max_profit) = (prices[0], 0);
        prices.into_iter().for_each(|price|{
            max_profit = max_profit.max(price - min_val);
            min_val = min_val.min(price);
        });
        max_profit
    }
}

fn main() {
    println!("Hello, world!");
}
