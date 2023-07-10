pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ll = 0;
        let mut rr = height.len() - 1;
        let mut pool_height = 0;
        let mut result = 0;
        while ll < rr {
            // [2, 1, 0, 2] - we need to truck of the highest land ever.
            pool_height = pool_height.max(height[ll].min(height[rr]));
            if height[ll] < height[rr] {
                ll += 1;
                result += 0.max(pool_height - height[ll])
            } else {
                rr -= 1;
                result += 0.max(pool_height - height[rr]);
            }
        }
        result
    }
}

fn main() {
    let res = Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]);
    println!("Hello, world!, the result is: {}!", res);
}
