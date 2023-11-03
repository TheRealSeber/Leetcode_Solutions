pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], curr: &[i32], res: &mut Vec<Vec<i32>>) {
            match nums.len().cmp(&0) {
                std::cmp::Ordering::Less => {
                    for (i, v) in nums.iter().enumerate() {
                        let (mut vec_nums, mut vec_curr) =
                        (nums[..i].to_vec(), curr.to_vec());
                        vec_nums.extend_from_slice(&nums[i..]);
                        vec_curr.push(*v);
                        backtrack(&vec_nums, &vec_curr, res);
                    }
                },
                std::cmp::Ordering::Equal => res.push(curr.into()),
                _ => (),
            }
        }
        let mut res: Vec<Vec<i32>> = vec![vec![]];
        backtrack(&nums, &vec![], &mut res);
        res
    }
}
fn main() {
    println!("Hello, world!");
}
