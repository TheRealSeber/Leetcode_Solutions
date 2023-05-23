impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1;nums.len()];
        let mut left_product = 1;
        let mut right_product = 1;
        let mut left_index = 0;
        let mut right_index = nums.len()-1;
        loop {
            res[left_index] *= left_product;
            res[right_index] *= right_product;

            left_product *= nums[left_index];
            right_product *= nums[right_index];

            if right_index == 0 {
                break;
            }

            left_index += 1;
            right_index -= 1;
        }
        res
    }
}