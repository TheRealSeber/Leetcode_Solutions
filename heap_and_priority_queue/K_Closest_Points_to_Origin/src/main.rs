use std::collections::BinaryHeap;
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::convert::From;

pub struct Solution {}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Point> = BinaryHeap::with_capacity(k as usize + 1);
        for point in points.iter() {
            heap.push(point.into());
            if heap.len() == k as usize + 1 {
                heap.pop();
            }
        }
        heap.into_sorted_vec().iter().collect::<Vec<Vec<i32>>>()
    }
}

#[derive(Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn distance(&self) -> i32 {
        self.x * self.x + self.y *self.y
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance().cmp(&other.distance())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&Vec<i32>> for Point {
    fn from(point: &Vec<i32>) -> Self {
        Point {
            x: point[0],
            y: point[1]
        }
    }
}

impl From<&Point> for Vec<i32> {
    fn from(point: &Point) -> Self {
        vec![point.x, point.y]
    }
}

impl<'a> FromIterator<&'a Point> for Vec<Vec<i32>> {
    fn from_iter<T: IntoIterator<Item = &'a Point>>(iter: T) -> Self {
        let mut points: Vec<Vec<i32>> = Vec::new();
        for point in iter {
            points.push(point.into());
        }
        points
    }
}

fn main() {
    println!("Hello, world!");
}
