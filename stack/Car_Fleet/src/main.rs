pub struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut initial_stack = position.into_iter().zip(speed.into_iter()).collect::<Vec<(i32, i32)>>();
        initial_stack.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        let mut prev_car_time = {
            let car = initial_stack.pop().unwrap();
            (target as f64 - car.0 as f64) / car.1 as f64
        };
        while let Some(car) = initial_stack.pop() {
            let curr_car_time = (target as f64- car.0 as f64) / car.1 as f64;
            match curr_car_time > prev_car_time {
                true => {
                    result += 1;
                    prev_car_time = curr_car_time;
                },
                false => {},
            }
        }
        result
    }
}

fn main() {
    println!("{}", Solution::car_fleet(12, vec![10,11,0,5,3], vec![2,4,1,1,3]));
}
