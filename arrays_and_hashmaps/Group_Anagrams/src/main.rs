use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hashes: Vec<(HashMap<char, i32>, Vec<String>)> = Vec::new();
        strs.iter().for_each(|string| {
            let mut map = HashMap::new();
            string.chars().for_each(|ch| *map.entry(ch).or_insert(0) += 1);
            match hashes.iter_mut().filter(|(hash_map, _)| hash_map == &map).next() {
                Some(data) => data.1.push(string.clone()),
                None => hashes.push((map, vec![string.clone()])),
            }
        });
        hashes .into_iter().map(|(_, vecstr)| vecstr).collect()   
    }
}