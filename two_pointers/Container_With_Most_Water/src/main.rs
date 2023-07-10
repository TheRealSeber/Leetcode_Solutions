pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let height_len = height.len() - 1;
        let mut ll = 0;
        let mut rr = height_len;
        let mut result = 0;

        while ll < rr {
            let min_heigth = height[ll].min(height[rr]);
            result = result.max(min_heigth*(rr-ll) as i32);
            while ll < rr && min_heigth >= height[ll] {
                ll += 1;
            }
            while ll < rr && min_heigth >= height[rr] {
                rr -= 1;
            }
        }
        result
    }
}

fn main() {
    let res = Solution::max_area(vec![1,8,6,2,5,4,8,3,7]);
    println!("Hello, world, the result is: {}!", res);
}
