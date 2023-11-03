pub struct Solution {}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut res: Vec<Vec<i32>> = vec![];
        let mut curr: Vec<i32> = vec![];
        fn backtrack(curr: &mut Vec<i32>, candidates: &[i32], res: &mut Vec<Vec<i32>>, target: i32) {
            for i in 0..candidates.len() {
                if i > 0 && candidates[i] == candidates[i-1] {
                    continue;
                }
                match target.cmp(&candidates[i]) {
                    std::cmp::Ordering::Greater => {
                        curr.push(candidates[i]);
                        backtrack(curr, &candidates[i+1..], res, target - candidates[i]);
                        curr.pop();        
                    }
                    std::cmp::Ordering::Equal => {
                        curr.push(candidates[i]);
                        res.push(curr.clone());
                        curr.pop();
                    },
                    _ => {
                        return;
                    },
                }
            }
        }
        backtrack(&mut curr, &candidates, &mut res, target);
        res
    }
}

fn main() {
    println!("Hello, world!");
}
