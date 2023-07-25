use std::collections::HashMap;

pub struct Solution {}

fn check_for_perm(s1_hash: &HashMap<u8, i32>, perm_hash: &HashMap<u8, i32>) -> bool {
    for (s1_key, s1_val) in s1_hash.iter() {
        println!("Check!");
        if let Some(perm_value) = perm_hash.get(s1_key) {
            println!("Key: {}, V: {}, Pv: {}", s1_key, s1_val, perm_value);
            if perm_value != s1_val {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let bytes = s2.into_bytes();
        let bytes_len = bytes.len();
        let s1_len = s1.len();
        let mut s1_hash: HashMap<u8, i32> = HashMap::new();
        s1.into_bytes().into_iter().for_each(|v| {
            s1_hash.entry(v).and_modify(|e| *e += 1).or_insert(1);
        });
        let mut perm_hash: HashMap<u8, i32> = HashMap::new();
        let mut l_ptr = 0;
        let mut r_ptr = 0;
        while r_ptr < bytes_len {
            if s1_hash.contains_key(&bytes[r_ptr]) {
                perm_hash
                    .entry(bytes[r_ptr])
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                if s1_len == r_ptr + 1 - l_ptr {
                    match check_for_perm(&s1_hash, &perm_hash) {
                        true => return true,
                        false => {
                            perm_hash.entry(bytes[l_ptr]).and_modify(|e| *e -= 1);
                            l_ptr += 1;
                        }
                    }
                }
                r_ptr += 1;
            } else {
                r_ptr += 1;
                l_ptr = r_ptr;
                perm_hash.clear();
            }
        }
        false
    }
}

fn main() {
    let res = Solution::check_inclusion("abc".to_string(), "bhkbdsbkbca".to_string());
    println!("Hello, world! The result is: {}", if res { 1 } else { 0 });
    let res = Solution::check_inclusion("hello".to_string(), "ooolleoooleh".to_string());
    println!("Hello, world! The result is: {}", if res { 1 } else { 0 });
}
