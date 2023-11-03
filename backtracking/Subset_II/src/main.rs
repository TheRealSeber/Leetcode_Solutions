use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // nums.sort_unstable();
        let mut res: HashSet<Vec<i32>> = HashSet::new();
        for num in nums.into_iter() {
            let mut new_vectors: HashSet<Vec<i32>> = HashSet::new();
            for v in res.iter() {
                let mut new_vec  = v.clone();
                new_vec.push(num);
                new_vectors.insert(new_vec);
            }
            res.insert(vec![num]);
            res.extend(new_vectors);
        }
        res.insert(vec![]);
        res.into_iter().collect()
    }
}

fn main() {
    let sol = Solution::subsets_with_dup(vec![1,2,2]);
    for k in sol {
        print!("[");
        for i in k {
            print!("{}, ", i);
        }
        println!("]")
    }
}
