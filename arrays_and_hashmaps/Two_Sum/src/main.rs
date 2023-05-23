impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (k, num) in nums.iter().enumerate() {
            if let Some(i) = nums.iter().skip(k+1).position(|&nnum| nnum + num == target){
                return vec![k as i32, (i+k+1) as i32]
            }
        }
        vec![]
    }
}
