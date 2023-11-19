pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let amount = amount as usize;
        let mut dp = vec![None; amount + 1];
        for i in 1..=amount {
            dp[i] = coins
                .iter()
                .filter_map(|&v| {
                    let v = v as usize;
                    if v - i <= 0 {
                        dp[i - v].map(|n| n + 1)
                    } else {
                        None
                    }
                })
                .min()
        }
        dp[amount].unwrap_or(-1)
    }
}
fn main() {
    println!("Hello, world!");
}
