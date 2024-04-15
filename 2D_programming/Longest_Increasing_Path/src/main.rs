pub struct Solution {}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m ,n) = (matrix[0].len(), matrix.len());
        let mut dp2 = vec![vec![0; m ]; n];
        let mut res = 0;
        for i in 0..n {
            for j  in 0..m {
                let mut max_step_into = 1;
                let mut max_step_out = 1;
                if i > 0 {
                    if matrix[i - 1][j] < matrix[i][j] {
                        max_step_into = dp2[i - 1][j] + 1;
                    } else if matrix[i - 1][j] > matrix[i][j] {
                        max_step_out = dp2[i - 1][j] + 1;
                    }
                }
                if j > 0 {
                    if matrix[i][j - 1] < matrix[i][j] {
                        max_step_into = max_step_into.max(dp2[i][j - 1] + 1)
                    } else if matrix[i][j - 1] > matrix[i][j] {
                        max_step_out = max_step_out.max(dp2[i][j - 1] + 1)
                    }
                }
                dbg!(&max_step_into, &max_step_out);
                dp2[i][j] = max_step_into.max(max_step_out);
                res = res.max(dp2[i][j]);
            }
        }
        res
    }
}

fn main() {
    Solution::longest_increasing_path(vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]]);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::longest_increasing_path(vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]]));
    }
}