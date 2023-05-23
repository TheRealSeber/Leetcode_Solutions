use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::with_capacity(k as usize);
        let mut res = Vec::with_capacity(k as usize);
        nums.iter().for_each(|&num| *map.entry(num).or_insert(0) += 1);
        map.iter().for_each(|(key, v)| heap.push((v, key)));
        while !heap.is_empty() && res.len() < k as usize{
            let (_, &key) = heap.pop().unwrap();
            res.push(key);
        }
        res
    }
}