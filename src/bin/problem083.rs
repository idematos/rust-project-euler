// Problem #83: Path Sum: Four Ways
// https://projecteuler.net/problem=83

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: Vec<Vec<u32>>) -> u32 {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let n = grid.len();
    
    let mut pq = BinaryHeap::new();
    pq.push(State { cost: grid[0][0], position: (0, 0) });

    let mut dist: HashMap<(usize, usize), u32> = HashMap::new();
    dist.insert((0, 0), grid[0][0]);

    while let Some(State { cost, position }) = pq.pop() {
        let (x, y) = position;

        if x == n - 1 && y == n - 1 {
            return cost;
        }

        for &(dx, dy) in &directions {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                let next_cost = cost + grid[nx][ny];

                if next_cost < *dist.get(&(nx, ny)).unwrap_or(&u32::MAX) {
                    dist.insert((nx, ny), next_cost);
                    pq.push(State {
                        cost: next_cost,
                        position: (nx, ny),
                    });
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    let grid = vec![
        vec![131, 673, 234, 103, 18],
        vec![201, 96, 342, 965, 150],
        vec![630, 803, 746, 422, 111],
        vec![537, 699, 497, 121, 956],
        vec![805, 732, 524, 37, 331],
    ];

    let result = dijkstra(grid);
    println!("The minimum path sum is {}", result);
}
