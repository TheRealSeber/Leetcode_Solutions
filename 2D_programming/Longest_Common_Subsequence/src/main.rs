pub struct Solution {}

impl Solution {

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for x in 1..text1.len() {
            for y in 1..text2.len() {
                if text1[x] == text2[y] {
                    dp[x][y] = dp[x-1][y-1] + 1
                } else {
                    dp[x][y] = dp[x-1][y].max(dp[x][y-1])
                }
            }
        }
        dp[text1.len()][text2.len()]
    }
}

fn main() {
    println!("Hello, world!");
}
