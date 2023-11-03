use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);    
        nums.into_iter().for_each(|num| {
            heap.push(Reverse(num));
            if heap.len() > k {
                heap.pop();
            }
        });

        heap.peek().unwrap().0
    }
}


fn main() {
    println!("Hello, world!");
}
