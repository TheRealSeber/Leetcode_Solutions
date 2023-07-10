use std::collections::HashSet;

pub struct Solution {}

// solution without using stupid sort at the beggining of the func (causes timeout on leetcode)
// impl Solution {
//     pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut result: HashSet<Vec<i32>> = HashSet::new();
//         if nums.len() == 3 {
//             return match nums.iter().sum::<i32>() == 0 {
//                 true => vec![nums],
//                 false => vec![]
//             }
//         }
//         for x in 0..nums.len()-3 {
//             let mut x_pos_direction = false;
//             let mut rr = nums.len() - 1;
//             let mut ll = nums.len() - 2;
//             loop {
//                 if nums[x] + nums[ll] + nums[rr] == 0 {
//                     let mut nums = vec![nums[x], nums[ll], nums[rr]];
//                     nums.sort_unstable();
//                     result.insert(nums);
//                 }
//                 if x == ll - 1 && ll == rr - 1 {
//                     break;
//                 }
//                 if ll == rr - 1 && x_pos_direction{
//                     rr -= 1;
//                     x_pos_direction = false;
//                 }
//                 if ll == x + 1 {
//                     x_pos_direction = true;
//                 }
//                 match x_pos_direction {
//                     true => ll += 1,
//                     false => ll -= 1,
//                 }
//             }
//         }
//         result.into_iter().collect::<Vec<Vec<i32>>>()
//     }
// }

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // interesting thing is the difference between stable and unstable sort.
        nums.sort_unstable();
        let mut result: HashSet<Vec<i32>> = HashSet::new();
        let const_len = nums.len() - 1;

        for x in 0..nums.len() - 2 {
            if nums[x] > 0 {
                break;
            }
            let mut ll = x + 1;
            let mut rr = const_len;
            while ll < rr {
                let sum = nums[x] + nums[ll] + nums[rr];
                if sum > 0 {
                    rr -= 1;
                    continue;
                } else if sum < 0 {
                    ll += 1;
                    continue;
                } else {
                    while ll < rr && nums[ll] == nums[ll+1] {
                        ll += 1;
                    }
                    let mut nums = vec![nums[x], nums[ll], nums[rr]];
                    nums.sort_unstable();
                    result.insert(nums);
                    ll += 1;
                }
            }
        }
        result.into_iter().collect::<Vec<Vec<i32>>>()
    }
}

fn main() {
    Solution::three_sum(vec![-1,0,1,2,-1,-4]);
    println!("Hello, world!");
}
