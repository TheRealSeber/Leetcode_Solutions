use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes_num = s.len();
        let mut res = 0_i32;
        let mut idx = 0_i32;
        let mut my_hash: HashMap<u8, i32> = HashMap::new();
        let bytes = s.into_bytes();
        while bytes_num as i32 != res + idx {
            my_hash
                .entry(bytes[(res + idx) as usize])
                .and_modify(|e| *e += 1)
                .or_insert(1);
            if res - k < *my_hash.values().max().unwrap() as i32 {
                res += 1;
                continue;
            }
            my_hash.entry(bytes[idx as usize]).and_modify(|v| *v -= 1);
            idx += 1;
        }
        res as i32
    }
}

fn main() {
    let result = Solution::character_replacement("AABABBA".to_string(), 1);
    let result2 = Solution::character_replacement("ABAB".to_string(), 2);
    println!("Hello, world!, result 1: {}, result 2: {}", result, result2);
}
