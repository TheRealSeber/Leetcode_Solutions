use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut left_brackets = VecDeque::with_capacity(s.len());
        for ch in s.chars() {
            match ch {
                '(' => left_brackets.push_back(')'),
                '{' => left_brackets.push_back('}'),
                '[' => left_brackets.push_back(']'),
                ')' | '}' | ']' if Some(ch) != left_brackets.pop_back() => return false,
                _ => ()
            }
        }
        left_brackets.is_empty()
    }
}
