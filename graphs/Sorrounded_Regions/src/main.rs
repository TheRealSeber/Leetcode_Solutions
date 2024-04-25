pub struct Solution {}

fn dfs(board: &Vec<Vec<char>>, (y, x): (usize, usize), visited_map: &mut Vec<Vec<bool>>) {
    if y > 0 && !visited_map[y - 1][x] && board[y - 1][x] == 'O' {
        visited_map[y - 1][x] = true;
        dfs(board, (y - 1, x), visited_map);
    }
    if y + 1 < board.len() && !visited_map[y + 1][x] && board[y + 1][x] == 'O' {
        visited_map[y + 1][x] = true;
        dfs(board, (y + 1, x), visited_map);
    }
    if x > 0 && !visited_map[y][x - 1] && board[y][x - 1] == 'O' {
        visited_map[y][x - 1] = true;
        dfs(board, (y, x - 1), visited_map);
    }
    if x + 1 < board[0].len() && !visited_map[y][x + 1] && board[y][x + 1] == 'O' {
        visited_map[y][x + 1] = true;
        dfs(board, (y, x + 1), visited_map);
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut visited_map: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
        for y in 0..board.len() {
            if !visited_map[y][0] && board[y][0] == 'O' {
                dfs(board, (y, 0), &mut visited_map);
            }
            if !visited_map[y][board[0].len() - 1] && board[y][board[0].len() - 1] == 'O' {
                dfs(board, (y, board[0].len() - 1), &mut visited_map);
            }
        }
        for x in 0..board[0].len() {
            if !visited_map[0][x] && board[0][x] == 'O' {
                dfs(board, (0, x), &mut visited_map);
            }
            if !visited_map[board.len() - 1][x] && board[board.len() - 1][x] == 'O' {
                dfs(board, (board.len() - 1, x), &mut visited_map);
            }
        }

        dbg!(&visited_map);

        for y in 1..board.len() - 1 {
            for x in 1..board[0].len() - 1 {
                if !visited_map[y][x] {
                    board[y][x] = 'X';
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
