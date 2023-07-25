pub struct Solution {}

// NOTE: this solution works, but is not so efficient.

use std::collections::{HashMap, VecDeque};

fn check_for_perm(goal_hash: &HashMap<u8, i32>, perm_hash: &HashMap<u8, i32>) -> bool {
    for (goal_key, goal_val) in goal_hash.iter() {
        if let Some(perm_val) = perm_hash.get(goal_key) {
            if perm_val < goal_val {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return "".to_string();
        }
        let t_len = t.len();
        let mut res: Option<(usize, usize)> = None;
        let mut goal_hash: HashMap<u8, i32> = HashMap::new();
        let mut idx_stack: VecDeque<usize> = VecDeque::new();
        let mut perm_hash: HashMap<u8, i32> = HashMap::new();
        let mut l_ptr = 0;
        let s_bytes = s.into_bytes();
        t.into_bytes().into_iter().for_each(|b| {
            goal_hash.entry(b).and_modify(|v| *v += 1).or_insert(1);
        });
        s_bytes.iter().enumerate().for_each(|(r_ptr, v)| {
            if goal_hash.contains_key(&v) {
                idx_stack.push_back(r_ptr);
                perm_hash.entry(*v).and_modify(|e| *e += 1).or_insert(1);
                if idx_stack.len() >= t_len {
                    while check_for_perm(&goal_hash, &perm_hash) {
                        l_ptr = idx_stack.pop_front().unwrap();
                        match res {
                            Some((r_in_mem, l_in_mem)) => {
                                if r_in_mem - l_in_mem > r_ptr - l_ptr {
                                    res = Some((r_ptr, l_ptr));
                                }
                            }
                            None => res = Some((r_ptr, l_ptr)),
                        }
                        perm_hash.entry(s_bytes[l_ptr]).and_modify(|e| *e -= 1);
                    }
                }
            }
        });
        match res {
            Some((r_ptr, l_ptr)) => String::from_utf8_lossy(&s_bytes[l_ptr..r_ptr + 1]).to_string(),
            None => "".to_string(),
        }
    }
}

fn main() {
    let res = Solution::min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string());
    println!("Hello, world! The res is: {}", res);
}
