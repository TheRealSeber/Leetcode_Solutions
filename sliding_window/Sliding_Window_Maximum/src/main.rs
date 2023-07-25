pub struct Solution {}

fn find_highest(nums: &Vec<i32>, l_ptr: i32, k: i32, max_in_range: &mut (i32, i32), to_reset: bool) {
    if to_reset {
        *max_in_range = (nums[l_ptr as usize], l_ptr)
    }
    println!("{}, {}", l_ptr, l_ptr + k);
    for (idx, i) in nums[l_ptr as usize..(l_ptr + k) as usize].iter().enumerate() {
        println!("{}", *i);
        if max_in_range.0 < *i {
            *max_in_range = (*i, idx as i32);
        }
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len() - k as usize);
        let mut max_in_range: (i32, i32) = (nums[0], 0);
        let nums_len = nums.len() as i32;
        let mut l_ptr = 0;
        find_highest(&nums, l_ptr, k, &mut max_in_range, false);
        res.push(max_in_range.0);
        while l_ptr + k < nums_len {
            let next_num = nums[(l_ptr + k) as usize];
            if next_num >= max_in_range.0 {
                max_in_range = (next_num, l_ptr + k);
            }
            l_ptr += 1;
            if l_ptr > max_in_range.1 {
                find_highest(&nums, l_ptr, k, &mut max_in_range, true);
            }
            res.push(max_in_range.0);
        }
        res
    }
}

fn main() {
    let res = Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
    println!("Hello, world! The result is: ");
    for i in res {
        print!("{} ", i);
    }
    println!();
    let res = Solution::max_sliding_window(vec![1, -1], 1);
    println!("The result is: ");
    for i in res {
        print!("{} ", i);
    }
}
