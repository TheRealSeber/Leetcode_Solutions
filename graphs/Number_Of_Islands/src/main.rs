pub struct Solution {}

fn dfs(island_map: &mut Vec<Vec<char>>, (y, x): (usize, usize)) {
    if y > 0 && island_map[y - 1][x] == '1' {
        island_map[y - 1][x] = '0';
        dfs(island_map, (y - 1, x));
    }
    if y + 1 < island_map.len() && island_map[y + 1][x] == '1' {
        island_map[y + 1][x] = '0';
        dfs(island_map, (y + 1, x));
    }
    if x > 0 && island_map[y][x - 1] == '1' {
        island_map[y][x - 1] = '0';
        dfs(island_map, (y, x - 1));
    }
    if x + 1 < island_map[0].len() && island_map[y][x + 1] == '1' {
        island_map[y][x + 1] = '0';
        dfs(island_map, (y, x + 1));
    }
}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut counter: i32 = 0;

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == '1' {
                    grid[y][x] = '0';
                    dfs(&mut grid, (y, x));
                    counter += 1;
                }
            }
        }

        counter
    }
}

fn main() {
    println!("Hello, world!");
}
