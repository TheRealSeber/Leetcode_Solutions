pub struct Solution {}

fn find_median_single_array(nums: &Vec<i32>) -> f64 {
    match nums.len() % 2 == 1 {
        true => nums[nums.len() / 2] as f64,
        false => {
            let mp = nums.len() / 2;
            (nums[mp] + nums[mp - 1]) as f64 / 2.0
        }
    }
}
// THIS IS HARD BRO!!! TOO HARD, i give up
// [2,2,5,6,8]
// [1,3,3,4,5]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 0 {
            find_median_single_array(&nums1)
        } else if nums2.len() == 0 {
            find_median_single_array(&nums2)
        } else {
            let mut lp_nums1 = 0_i32;
            let mut rp_nums1 = (nums1.len() - 1) as i32;
            let mut lp_nums2 = 0_i32;
            let mut rp_nums2 = (nums2.len() - 1) as i32;
            while lp_nums1 <= rp_nums1 && lp_nums2 <= rp_nums2 {
                let mp_nums1 = (lp_nums1 + rp_nums1) / 2;
                let mp_nums2 = (lp_nums2 + rp_nums2) / 2;
                if mp_nums1 > mp_nums2 {
                    rp_nums1 = mp_nums1 - 1;
                }
            }
            todo!()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
