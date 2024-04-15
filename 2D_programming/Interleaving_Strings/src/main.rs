pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let (s1_l, s2_l) = (s1.len(), s2.len());
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp2 = vec![vec![false; s2_l + 1]; s1_l + 1];
        dp2[s1_l][s2_l] = true;

        for i in (0..s1_l).rev() {
            dp2[i][s2_l] = s1[i] == s3[i + s2_l] && dp2[i + 1][s2_l];
        }

        for j in (0..s2_l).rev() {
            dp2[s1_l][j] = s2[j] == s3[s1_l + j] && dp2[s1_l][j + 1];
        }

        for i in (0..s1_l).rev() {
            for j in (0..s2_l).rev() {
                if s1[i] == s3[i+j] && dp2[i + 1][j] {
                    dp2[i][j] = true;
                }
                if s2[j] == s3[i+j] && dp2[i][j + 1] {
                    dp2[i][j] = true;
                }
            }
        }
        dp2[0][0]
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()))
    }
}