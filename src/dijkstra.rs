use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Node {
    cost: u64,
    position: (usize, usize),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path(grid: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dist = vec![vec![u64::MAX; cols]; rows];
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut heap = BinaryHeap::new();
    dist[0][0] = grid[0][0];
    heap.push(Node { cost: dist[0][0], position: (0, 0) });

    while let Some(Node { cost, position }) = heap.pop() {
        let (r, c) = position;
        if cost > dist[r][c] { continue; }

        for &(dr, dc) in &[(0,1),(1,0),(0,-1_i32 as usize),(usize::MAX,0)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < rows && nc < cols {
                let new_cost = dist[r][c].saturating_add(grid[nr][nc]);
                if new_cost < dist[nr][nc] {
                    dist[nr][nc] = new_cost;
                    prev.insert((nr,nc),(r,c));
                    heap.push(Node { cost: new_cost, position: (nr,nc) });
                }
            }
        }
    }

    // Reconstruct path
    let mut path = Vec::new();
    let mut curr = (rows-1, cols-1);
    while curr != (0,0) {
        path.push(curr);
        curr = prev[&curr];
    }
    path.push((0,0));
    path.reverse();
    path
}

pub fn longest_path(grid: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
    let max_val = grid.iter().flat_map(|r| r.iter()).sum::<u64>();
    let neg_grid: Vec<Vec<u64>> = grid.iter()
        .map(|row| row.iter().map(|&x| max_val - x).collect())
        .collect();

    shortest_path(&neg_grid)
}
