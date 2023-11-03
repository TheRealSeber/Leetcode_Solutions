struct Solution {}

impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0{
            return vec![vec![]];
        }
        let last = nums.pop().unwrap();
        let remain_subsets = Self::subsets(nums);
        let mut res = remain_subsets.clone();
        for mut v in remain_subsets {
            v.push(last);
            res.push(v);
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
    let res = Solution::subsets(vec![1,2,3]);
    for v in res {
        for x in v {
            print!("{}", x);
        }
        println!()
    }
}

