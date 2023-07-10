use std::cmp::max;

pub struct Solution {}

impl Solution {

    fn find_biggest_rectangle(heights: &mut Vec<i32>) -> i32 {
        let mut popped = 1;
        let highest_num: i32 = heights.pop().unwrap();
        let mut result = highest_num;
        while let Some(height) = heights.pop() {
            popped += 1;
            if height < highest_num {
                heights.extend(vec![height; popped]);
                
                if !heights.is_empty() {
                    return max(result, Self::find_biggest_rectangle(heights))
                }
            }
            result += highest_num;
        }
        result
    }

    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut histogram = vec![];
        let mut result = 0;
        for height in heights {
            let mut temp_histogram = vec![];
            while let Some(height_h) = histogram.last() {
                if height < *height_h {
                    temp_histogram.push(histogram.pop().unwrap());
                } else {
                    break;
                }
            }
            let temp_h_len = temp_histogram.len();
            if !temp_histogram.is_empty() {
                temp_histogram.reverse();
                result = max(result, Self::find_biggest_rectangle(&mut temp_histogram));
                histogram.extend(vec![height; temp_h_len]);
            }
            histogram.push(height);
        }
        max(result, Self::find_biggest_rectangle(&mut histogram))
    }
}

fn main() {
    let a = Solution::largest_rectangle_area(vec![3,6,5,7,4,8,1,0]);
    println!("{}", a);
}
