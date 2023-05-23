use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut map = HashSet::with_capacity(9);
        for row in board.iter() {
            for &ch in row.iter().filter(|&ch| ch != &'.') {
                if !map.insert(ch) {
                    return false;
                }
            }
            map.clear();
        }
        for i in 0..9 {
            for j in 0..9 {
                if board[j][i] != '.' {
                    if !map.insert(board[j][i]) {
                        return false;
                    }
                }
            }
            map.clear();
        }
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for z in 0..3 {
                        if board[k + i * 3][z + j * 3] != '.' {
                            if !map.insert(board[k + i * 3][z + j * 3]) {
                                return false;
                            }
                        }
                    }
                }
                map.clear();
            }
        }
        true
    }
}
