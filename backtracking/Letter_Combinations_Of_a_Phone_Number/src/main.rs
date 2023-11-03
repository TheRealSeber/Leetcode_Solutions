pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let letters: Vec<&str> = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut res: Vec<String> = Vec::new();
        fn backtrack(res: &mut Vec<String>, letters: &Vec<&str>, curr_comb: String, next_digits: &[char]) {
            if next_digits.is_empty() {
                res.push(curr_comb);
            } else {
                let curr_letters = letters[next_digits[0] as usize - '2' as usize];
                for letter in curr_letters.chars() {
                    let new_combination = format!("{}{}", curr_comb, letter);
                    backtrack(res, letters, new_combination, &next_digits[1..]);
                }
            }
        }

        backtrack(&mut res, &letters, String::new(), &digits.chars().into_iter().collect::<Vec<char>>());
        res
    }
}

fn main() {
    println!("Hello, world!");
}
