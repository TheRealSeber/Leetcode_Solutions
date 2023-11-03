use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let tasks_len = tasks.len() as i32;
        let mut my_hash: HashMap<char, i32> = HashMap::new();
        let mut max_f = 1_i32;
        for b in tasks.into_iter() {
            my_hash.entry(b).and_modify(|v| {
                *v += 1;
                if max_f < *v {
                    max_f = *v
                }
            }).or_insert(1);
        }
        tasks_len.max((n + 1) * (max_f - 1) + my_hash.into_iter().fold(0, |mut acc, (_, v)| {
            match max_f == v {
                true => acc + 1,
                false => acc,
            }
        }))
    }
}

fn main() {
    println!("Hello, world!");
}
