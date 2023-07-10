pub struct Solution {}

impl Solution {
    fn second_solution(s: String) -> bool {
        let s_u8 = s.as_bytes();
        let mut ll = 0_i32;
        let mut rr = (s_u8.len() - 1) as i32;

        while ll < rr {
            while ll < rr && !(s_u8[ll as usize] as char).is_alphanumeric() {
                ll += 1;
            }
            while ll < rr && !(s_u8[rr as usize] as char).is_alphanumeric() {
                rr -= 1;
            }

            if (s_u8[rr as usize] as char).to_ascii_lowercase() != (s_u8[ll as usize] as char).to_ascii_lowercase() {
                return false;
            }
            ll += 1;
            rr -= 1;
        }
        true
    }

    pub fn is_palindrome(s: String) -> bool {
        Self::second_solution(s)
        // let cleared_string = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_lowercase().to_string()).collect::<String>();
        // cleared_string.chars().rev().collect::<String>() == cleared_string
    }
}

fn main() {
    println!("Hello, world!");
}
