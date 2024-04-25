pub struct Solution {}

#[derive(Clone)]
pub enum Status {
    Unvisited,
    Visiting,
    Visited
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;

        let mut graph = vec![Vec::new(); num_courses];
        for edge in prerequisites {
            graph[edge[0] as usize].push(edge[1] as usize);
        }

        let mut status = vec![Status::Unvisited; num_courses];
        let mut res_order = vec![];
        match (0..num_courses).all(|id| !Solution::has_cycle(id, &graph, &mut status, &mut res_order)) {
            true => res_order,
            false => vec![]
        }
    }

    pub fn has_cycle(id: usize, graph: &Vec<Vec<usize>>, status: &mut Vec<Status>, res_order: &mut Vec<i32>) -> bool {
        match status[id] {
            Status::Visited => false,
            Status::Visiting => true,
            Status::Unvisited => {
                status[id] = Status::Visiting;
                if graph[id].iter().any(|i| Solution::has_cycle(*i, &graph, status, res_order)) {
                    return true;
                }
                res_order.push(id as i32);
                status[id] = Status::Visited;
                false
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
