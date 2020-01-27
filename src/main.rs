use std::env;

use mazinator::algo;
use mazinator::display::*;
use mazinator::utils::*;

fn main() {
    let config = Options::parse(env::args());
    let walls = algo::run(&config);
    // Finally print maze.
    generate_maze(config.cols, walls);
}
