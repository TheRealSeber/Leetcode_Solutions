use std::{collections::VecDeque, vec};

struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: VecDeque<(usize, i32)> = VecDeque::with_capacity(temperatures.len());
        let mut res = vec![0; temperatures.len() as usize];

        for (index, number) in temperatures.iter().enumerate() {
            while let Some((s_idx, s_number)) = stack.pop_back() {
                match s_number < *number {
                    true => {
                        res[s_idx] = (index - s_idx) as i32;
                    }
                    false => {
                        stack.push_back((s_idx, s_number));
                        break;
                    }
                }
            }
            stack.push_back((index, *number));
        }
        res
    }
}

fn main() {
    let result = Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
    for r in result {
        println!("{}", r);
    }
}
