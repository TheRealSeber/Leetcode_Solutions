pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ll = 0_i32;
        let mut rr = (numbers.len() - 1) as i32;
        loop {
            if numbers[ll as usize] + numbers[rr as usize] < target {
                ll += 1;
                continue;
            } else if numbers[ll as usize] + numbers[rr as usize] > target {
                rr -= 1;
                continue;
            }  
            break;
        }
        vec![ll+1, rr+1]
    }
}

fn main() {
    println!("Hello, world!");
}
