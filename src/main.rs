use std::env;

use mazinator::algo;
use mazinator::display::*;
use mazinator::utils::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Options::parse(&args);
    let walls = algo::run(&config);
    // Finally print maze.
    generate_maze(config.cols, walls);
}
