pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[s.len()] = true;
        (0..s.len()).into_iter().rev().for_each(|i| {
            for word in word_dict.iter() {
                if i + word.len() <= s.len() && s[i..i + word.len()].contains(word) {
                    dp[i] = dp[i + word.len()];
                }
                if dp[i] {
                    break;
                }
            }
        });
        dp[0]
    }
}

fn main() {
    println!("Hello, world!");
}
