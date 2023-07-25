use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut my_hash = HashMap::new();
        let mut result = 0;
        s.as_bytes().into_iter().enumerate().for_each(|(idx, ch)| {
            if let Some(&val) = my_hash.get(&ch) {
                result = result.max(my_hash.len());
                my_hash.retain(|_, v| *v > val);
            };
            my_hash.insert(ch, idx);
        });
        result.max(my_hash.len()) as i32
    }
}

fn main() {
    println!("Hello, world!");
}
