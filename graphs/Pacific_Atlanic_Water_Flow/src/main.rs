pub struct Solution {}

fn dfs(
    island_map: &Vec<Vec<i32>>,
    (y, x): (usize, usize),
    to_pacific: bool,
    visited_map: &mut Vec<Vec<bool>>,
) -> bool {
    visited_map[y][x] = true;
    if to_pacific && (x == 0 || y == 0) {
        return true;
    }
    if !to_pacific && (y == island_map.len() - 1 || x == island_map[0].len() - 1) {
        return true;
    }

    let mut succeeded = false;

    if y > 0 && !visited_map[y - 1][x] && island_map[y - 1][x] <= island_map[y][x] {
        succeeded = succeeded || dfs(island_map, (y - 1, x), to_pacific, visited_map);
    }
    if !succeeded
        && y + 1 < island_map.len()
        && !visited_map[y + 1][x]
        && island_map[y + 1][x] <= island_map[y][x]
    {
        succeeded = succeeded || dfs(island_map, (y + 1, x), to_pacific, visited_map);
    }
    if !succeeded && x > 0 && !visited_map[y][x - 1] && island_map[y][x - 1] <= island_map[y][x] {
        succeeded = succeeded || dfs(island_map, (y, x - 1), to_pacific, visited_map);
    }
    if !succeeded
        && x + 1 < island_map[0].len()
        && !visited_map[y][x + 1]
        && island_map[y][x + 1] <= island_map[y][x]
    {
        succeeded = succeeded || dfs(island_map, (y, x + 1), to_pacific, visited_map);
    }

    succeeded
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let visited_map: Vec<Vec<bool>> = vec![vec![false; heights[0].len()]; heights.len()];
        (0..heights.len()).fold(vec![], |mut acc, y| {
            acc.extend((0..heights[0].len()).fold(vec![], |mut acc_2, x| {
                match dfs(&heights, (y, x), true, &mut visited_map.clone())
                    && dfs(&heights, (y, x), false, &mut visited_map.clone())
                {
                    true => {
                        acc_2.push(vec![y as i32, x as i32]);
                        acc_2
                    }
                    false => acc_2,
                }
            }));
            acc
        })
    }
}

fn main() {
    println!("Hello, world!");
}
