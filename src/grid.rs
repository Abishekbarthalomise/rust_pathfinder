use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn generate_map(rows: usize, cols: usize) -> Vec<Vec<u64>> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..rows)
        .map(|_| (0..cols).map(|_| rng.gen_range(0..=255) as u64).collect())
        .collect()
}

pub fn save_to_file(grid: &Vec<Vec<u64>>, filename: &str) {
    use std::fs::write;
    let content = grid
        .iter()
        .map(|row| row.iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n");
    write(filename, content).expect("Failed to write file");
}

pub fn load_from_file(filename: &str) -> Vec<Vec<u64>> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| u64::from_str_radix(x, 16).unwrap())
                .collect()
        })
        .collect()
}
