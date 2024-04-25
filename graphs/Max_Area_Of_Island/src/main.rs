pub struct Solution {}

fn dfs(island_map: &mut Vec<Vec<i32>>, (y, x): (usize, usize), counter: &mut i32) -> i32 {
    if y > 0 && island_map[y - 1][x] == 1 {
        *counter += 1;
        island_map[y - 1][x] = 0;
        dfs(island_map, (y - 1, x), counter);
    }
    if y + 1 < island_map.len() && island_map[y + 1][x] == 1 {
        *counter += 1;
        island_map[y + 1][x] = 0;
        dfs(island_map, (y + 1, x), counter);
    }
    if x > 0 && island_map[y][x - 1] == 1 {
        *counter += 1;
        island_map[y][x - 1] = 0;
        dfs(island_map, (y, x - 1), counter);
    }
    if x + 1 < island_map[0].len() && island_map[y][x + 1] == 1 {
        *counter += 1;
        island_map[y][x + 1] = 0;
        dfs(island_map, (y, x + 1), counter);
    }
    *counter
}

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut res: i32 = 0;

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 1 {
                    grid[y][x] = 0;
                    res = res.max(dfs(&mut grid, (y, x), &mut 1));
                }
            }
        }

        res
    }
}

fn main() {
    println!("Hello, world!");
}
