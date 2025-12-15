mod grid;
mod visualize;
mod animation;
mod dijkstra;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <command> [args]");
        eprintln!("Commands: --generate NxM | --visualize | --both | --animate <file>");
        return;
    }

    match args[1].as_str() {
        "--generate" => {
            if args.len() < 3 {
                eprintln!("Usage: --generate NxM");
                return;
            }

            let dims: Vec<&str> = args[2].split('x').collect();
            if dims.len() != 2 {
                eprintln!("Usage: --generate NxM");
                return;
            }

            let rows: usize = dims[0].parse().expect("Invalid number for rows");
            let cols: usize = dims[1].parse().expect("Invalid number for cols");

            let grid_data = grid::generate_map(rows, cols);
            grid::save_to_file(&grid_data, "map.txt");
            println!("Saved map to map.txt");
        }

        "--visualize" => {
            visualize::show();
        }

        "--both" => {
            visualize::show_both();
        }

        "--animate" => {
            if args.len() < 3 {
                eprintln!("Usage: --animate <file>");
                return;
            }
            let file = &args[2];
            animation::run(file);
        }

        _ => eprintln!("Unknown command"),
    }
}
