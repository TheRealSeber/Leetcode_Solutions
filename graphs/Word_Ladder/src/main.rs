use std::collections::VecDeque;

pub struct Solution {}

fn is_1_diff(s1: &[u8], s2: &[u8]) -> bool {
    let mut count = 0;
    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            count += 1;
        }
        if count > 1 {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut visited: Vec<bool> = vec![false; word_list.len()];

        let mut queue: VecDeque<(&str, i32)> = VecDeque::new();

        for i in 0..word_list.len() {
            if is_1_diff(begin_word.as_bytes(), word_list[i].as_bytes()) {
                if end_word == word_list[i] {
                    return 2;
                }
                queue.push_back((&word_list[i], 2));
                visited[i] = true;
            }
        }

        while let Some((next, num_words_passed)) = queue.pop_front() {
            for idx in 0..word_list.len() {
                if !visited[idx] && is_1_diff(word_list[idx].as_bytes(), next.as_bytes()) {
                    if word_list[idx] == end_word {
                        return num_words_passed + 1;
                    }
                    visited[idx] = true;
                    queue.push_back((&word_list[idx], num_words_passed + 1));
                }
            }
        }
        0
    }
}

fn main() {
    println!("Hello, world!");
}
