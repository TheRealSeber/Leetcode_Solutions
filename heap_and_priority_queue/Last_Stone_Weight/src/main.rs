use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = stones.into_iter().collect::<BinaryHeap<i32>>();
        loop {
            match (heap.pop(), heap.pop()) {
                (None, None) => return 0,
                (Some(rock), None) => return rock,
                (Some(l_rock), Some(r_rock)) => match l_rock - r_rock {
                    0 => (),
                    any => heap.push(any),
                },
                _ => (),
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
