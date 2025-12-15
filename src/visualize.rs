use crate::grid;
use crate::dijkstra;
use std::collections::HashSet;

pub fn show() {
    let grid_u64 = grid::load_from_file("map.txt");
    for row in &grid_u64 {
        let row_str: Vec<String> = row.iter().map(|v| format!("{:02X}", v)).collect();
        println!("{}", row_str.join(" "));
    }
}

pub fn show_both() {
    let grid_u64 = grid::load_from_file("map.txt");

    let min_path = dijkstra::shortest_path(&grid_u64);
    let max_path = dijkstra::longest_path(&grid_u64);

    let min_set: HashSet<(usize,usize)> = min_path.into_iter().collect();
    let max_set: HashSet<(usize,usize)> = max_path.into_iter().collect();

    for (r, row) in grid_u64.iter().enumerate() {
        let mut out = Vec::new();
        for (c, &val) in row.iter().enumerate() {
            if min_set.contains(&(r,c)) && max_set.contains(&(r,c)) {
                out.push(format!("[B]")); // both
            } else if min_set.contains(&(r,c)) {
                out.push(format!("[S]")); // shortest
            } else if max_set.contains(&(r,c)) {
                out.push(format!("[L]")); // longest
            } else {
                out.push(format!("{:02X}", val));
            }
        }
        println!("{}", out.join(" "));
    }
}

