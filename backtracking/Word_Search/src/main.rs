pub struct Solution {}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let (height, width) = (board.len(), board[0].len());
        for i in 0..height {
            for j in 0..width {
                if check_next_char(&word, &mut board, i, j).is_some() {
                    return true;
                }
            }
        }
        false
    }
}

fn backtrack(board: &mut Vec<Vec<char>>, column: usize, row: usize, word_to_find: &str) -> bool {
    if word_to_find.is_empty() {
        return true;
    }
    if column > 0 {
        if check_next_char(word_to_find, board, column - 1, row).is_some() {
            return true;
        }
    }
    if row > 0 {
        if check_next_char(word_to_find, board, column, row - 1).is_some() {
            return true;
        }
    } 
    if column < board.len() - 1 {
        if check_next_char(word_to_find, board, column + 1, row).is_some() {
            return true;
        }
    } 
    if row < board[0].len() - 1 {
        if check_next_char(word_to_find, board, column, row + 1).is_some() {
            return true;
        }
    }
    false
}

fn check_next_char(word_to_find: &str, board: &mut Vec<Vec<char>>, column: usize, row: usize) -> Option<()> {
    if word_to_find.starts_with(board[column][row]) {
        let temp = board[column][row];
        board[column][row] = ' ';
        if backtrack(board, column, row, &word_to_find[1..]) {
            return Some(());
        }
        board[column][row] = temp;
    }
    None
}

fn main() {
    println!("Hello, world!");
}
