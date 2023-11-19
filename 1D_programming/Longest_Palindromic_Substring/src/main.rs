pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s_len = s.len();
        let mut res = String::new();
        for i in 0..s_len {
            let (mut l, mut r) = (i as i32, i);
            while l >= 0 && r < s.len() && s.as_bytes()[l as usize] == s.as_bytes()[r] {
                if s[l as usize..r+1].len() > res.len() {
                    if r - l as usize >= res.len() {
                        res = s[l as usize..r+1].to_string();
                    }
                }
                l -= 1;
                r += 1;
            }
            let (mut l, mut r) = (i as i32 - 1, i);
            while l >= 0 && r < s.len() && s.as_bytes()[l as usize] == s.as_bytes()[r] {
                if s[l as usize..r+1].len() > res.len() {
                    if r - l as usize >= res.len() {
                        res = s[l as usize..r+1].to_string();
                    }
                }
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
