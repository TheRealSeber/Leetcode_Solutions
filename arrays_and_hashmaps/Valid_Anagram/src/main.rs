use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut my_hash = HashMap::new();
        s.chars().for_each(|ch| *my_hash.entry(ch).or_insert(0) += 1);
        t.chars().for_each(|ch| *my_hash.entry(ch).or_insert(0) -= 1);
        my_hash.into_values().all(|v| v == 0)
    }
}