use rand::Rng;
use std::{env, process};

use mazinator::algo::dfs::*;
use mazinator::display::*;
use mazinator::structure::matrix::*;
use mazinator::structure::wall::*;

fn unwrap_int_or_panic(arg: &String) -> usize {
    match arg.to_string().parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // First arg is the program itself.
    if args.len() != 3 {
        eprintln!("Need 2 integer arguments.");
        process::exit(1);
    }

    let rows = unwrap_int_or_panic(&args[1]);
    let cols = unwrap_int_or_panic(&args[2]);

    if rows <= 1 || cols <= 1 {
        eprintln!("rows, cols must be >= 1.");
        process::exit(1);
    }

    let mut mat = Matrix::new(rows, cols);
    let mut walls = Walls::new(rows - 1, cols - 1);

    // Randomly select a start point.
    let rand_x = rand::thread_rng().gen_range(0, rows);
    let rand_y = rand::thread_rng().gen_range(0, cols);
    // Run algorithm.
    dfs(&mut mat, &mut walls, rand_x, rand_y);

    // Finally print maze.
    generate_maze(cols, walls);
}
