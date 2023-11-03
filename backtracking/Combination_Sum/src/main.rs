pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(curr_sub: &[i32], candidates: &[i32], target: i32, res: &mut Vec<Vec<i32>>) {
            match curr_sub.iter().sum::<i32>().cmp(&target) {
                std::cmp::Ordering::Equal => {
                    res.push(curr_sub.to_vec());
                    return;
                }
                std::cmp::Ordering::Greater => return,
                _ => (),
            }

            for (i, v) in candidates.iter().enumerate() {
                let mut new_vec = curr_sub.to_vec();
                new_vec.push(*v);
                backtrack(&new_vec, &candidates[i..], target, res)
            }
        }
        let mut result: Vec<Vec<i32>> = vec![];
        backtrack(&vec![], &candidates, target, &mut result);
        result
    }
}

fn main() {
    let res = Solution::combination_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 12);
    for i in res {
        print!("[");
        for a in i {
            print!("{},", a);
        }
        println!("]");
    }
    println!("Hello, world!");
}
