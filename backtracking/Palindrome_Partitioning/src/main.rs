pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        fn backtrack(curr: &mut Vec<String>, candidates: &Vec<char>, res: &mut Vec<Vec<String>>, idx: usize) {
            if idx == candidates.len() {
                res.push(curr.clone());
            }

            for i in idx..candidates.len() {
                if candidates[idx..i+1].into_iter().ne(candidates[idx..i+1].into_iter().rev()) {
                    continue;
                }
                curr.push(candidates[idx..i+1].into_iter().collect());
                backtrack(curr, candidates, res, i+1);
                curr.pop();
            }
        }
        backtrack(&mut vec![], &s.chars().collect::<Vec<char>>(), &mut res, 0);
        res
    }
}

fn main() {
    println!("Hello, world!");
}
