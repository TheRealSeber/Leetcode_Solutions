use std::collections::VecDeque;

pub struct Solution {}

fn bfs(
    island_map: &mut Vec<Vec<i32>>,
    (y, x, counter): (usize, usize, i32),
    queue: &mut VecDeque<(usize, usize, i32)>,
) {
    if y > 0 && island_map[y - 1][x] == 1 {
        island_map[y - 1][x] = 2;
        queue.push_back((y - 1, x, counter));
    }
    if y + 1 < island_map.len() && island_map[y + 1][x] == 1 {
        island_map[y + 1][x] = 2;
        queue.push_back((y + 1, x, counter));
    }
    if x > 0 && island_map[y][x - 1] == 1 {
        island_map[y][x - 1] = 2;
        queue.push_back((y, x - 1, counter));
    }
    if x + 1 < island_map[0].len() && island_map[y][x + 1] == 1 {
        island_map[y][x + 1] = 2;
        queue.push_back((y, x + 1, counter));
    }
}

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 2 {
                    queue.push_back((y, x, 0));
                }
            }
        }

        while let Some((y, x, counter)) = queue.pop_front() {
            bfs(&mut grid, (y, x, counter + 1), &mut queue);
            res = counter;
        }

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 1 {
                    return -1;
                }
            }
        }

        res
    }
}

fn main() {
    println!("Hello, world!");
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    println!("{}\n", Solution::oranges_rotting(grid));
}
