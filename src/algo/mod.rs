use rand::Rng;
use std::process;

use super::structure::matrix::*;
use super::structure::wall::*;
use super::utils::*;
use bfs::*;
use dfs::*;

pub mod bfs;
pub mod dfs;

pub fn run(config: &Config) -> Walls {
    let rows = config.rows;
    let cols = config.cols;
    let mut mat = Matrix::new(rows, cols);
    let mut walls = Walls::new(rows - 1, cols - 1);
    // // Randomly select a start point.
    let rand_x = rand::thread_rng().gen_range(0, rows);
    let rand_y = rand::thread_rng().gen_range(0, cols);

    if config.algorithm == "bfs" {
        bfs(&mut mat, &mut walls, rand_x, rand_y);
    } else if config.algorithm == "dfs" {
        dfs(&mut mat, &mut walls, rand_x, rand_y);
    } else {
        eprintln!("Unknown algorithm.");
        process::exit(1);
    }

    walls
}
