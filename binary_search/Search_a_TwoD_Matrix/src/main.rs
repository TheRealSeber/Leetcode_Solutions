pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len() as i32;
        let columns = matrix[0].len() as i32;

        // first we will seatch between rows
        let mut lp = 0_i32;
        let mut rp = rows - 1;

        while lp <= rp {
            let mp = (lp + rp) / 2;
            if matrix[mp as usize][0] > target {
                rp = mp - 1;
            } else if *matrix[mp as usize].last().unwrap() < target {
                lp = lp + 1
            } else {
                break;
            }
        }

        let row = (lp + rp) / 2;
        lp = 0;
        rp = columns - 1;
        while lp <= rp {
            let mp = (lp + rp) / 2;
            if matrix[row as usize][mp as usize] > target {
                rp = mp - 1;
            } else if matrix[row as usize][mp as usize] < target {
                lp = mp + 1;
            } else {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
