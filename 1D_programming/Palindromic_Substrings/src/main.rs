pub struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s_len = s.len();
        let mut res = 0;
        for i in 0..s_len {
            let (mut l, mut r) = (i as i32, i);
            while l >= 0 && r < s.len() && s.as_bytes()[l as usize] == s.as_bytes()[r] {
                res += 1;
                l -= 1;
                r += 1;
            }
            let (mut l, mut r) = (i as i32 - 1, i);
            while l >= 0 && r < s.len() && s.as_bytes()[l as usize] == s.as_bytes()[r] {
                res += 1;
                l -= 1;
                r += 1;
            }
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
