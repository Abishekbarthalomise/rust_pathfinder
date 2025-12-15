use crate::grid;
use crate::dijkstra;
use std::{thread, time::Duration};

pub fn run(filename: &str) {
    let grid_u8 = grid::load_from_file(filename);
    let grid_u64: Vec<Vec<u64>> = grid_u8.iter()
        .map(|row| row.iter().map(|&x| x as u64).collect())
        .collect();

    let path = dijkstra::shortest_path(&grid_u64);

    for &(r,c) in path.iter() {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        for (ri, row) in grid_u8.iter().enumerate() {
            for (ci, &val) in row.iter().enumerate() {
                if ri==r && ci==c {
                    print!("[*] ");
                } else {
                    print!("{:02X} ", val);
                }
            }
            println!();
        }
        thread::sleep(Duration::from_millis(200));
    }

    println!("Animation complete!");
}
