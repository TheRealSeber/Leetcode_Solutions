pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // f(n) = f(n - 1) -- ( when s[i] is possible to construct a num ) 
        //      + f(n - 2) -- ( when s[i] and s[i+1] is possible to construct a num)
        if s.contains("00") | s.starts_with("0") {
            return 0;
        }
        for x in 3..10 {
            if s.contains(format!("{}0", x).as_str()) {
                return 0;
            }
        }
        let s = s.as_bytes();
        match s.len() {
            1 => return 1,
            n => {
                let mut dp = vec![0; n];
                for i in (0..n).rev() {
                    if s[i] == b'0' { continue; }
                    if i + 1 == n {
                        dp[i] = 1;
                        continue;
                    }
                    let mut v = dp[i + 1];
                    if s[i] == b'1' || (s[i] == b'2' && s[i + 1] <= b'6') { 
                        if i + 2 < n { v += dp[i + 2]; }
                        else { v += 1; }
                    }
                    dp[i] = v;
                }
                dp[0]
            }
        }        
    }
}

fn main() {
    println!("Hello, world!");
}
