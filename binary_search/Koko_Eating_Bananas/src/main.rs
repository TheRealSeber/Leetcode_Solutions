pub struct Solution {}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        if piles.len() == h as usize {
            return *piles.iter().max().unwrap();
        }
        let mut lp = 0;
        let mut rp = *piles.iter().max().unwrap();
        'outer: while lp != rp {
            let mp = (lp + rp) / 2;
            let mut sum_hours = 0;
            for bananas in piles.iter() {
                sum_hours += (*bananas as f64 / mp as f64).ceil() as i32;
                if sum_hours > h {
                    lp = mp + 1;
                    continue 'outer;
                }
            }
            rp = mp;
        }
        rp
    }
}


fn main() {
    let res = Solution::min_eating_speed(vec![30,11,23,4,20], 6);
    println!("Hello, world! The result is: {}", res);
}
