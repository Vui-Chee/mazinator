use colored::*;

use super::structure::wall::*;

// TODO alternate the colors?
const BLOCK: &str = "\u{2588}\u{2588}";
const SPACE: &str = "  ";

pub fn generate_maze(rows: usize, cols: usize, walls: Walls) {
    println!("");

    // print top horizontal wall
    for _i in 0..(2 * cols + 1) {
        print!("{}", BLOCK.yellow());
    }
    println!("");

    let mut turn = true; // true == vertical walls turn
    let mut side_idx = 0;
    let mut flat_idx = 0;
    let total_iterations = walls.flat_walls.len() + walls.side_walls[0].len();
    for _i in 0..total_iterations {
        if turn {
            print!("{}", BLOCK.yellow());
            print!("{}", SPACE);
            for wall_vector in walls.side_walls.iter() {
                if wall_vector[side_idx] == WallType::Vertical {
                    print!("{}", BLOCK.yellow());
                    print!("{}", SPACE);
                } else if wall_vector[side_idx] == WallType::Missing {
                    print!("{}", SPACE);
                    print!("{}", SPACE);
                }
            }
            println!("{}", BLOCK.yellow());
            side_idx += 1;
        } else {
            print!("{}", BLOCK.yellow());
            for wall in walls.flat_walls[flat_idx].iter() {
                if *wall == WallType::Horizontal {
                    print!("{}", BLOCK.yellow());
                } else if *wall == WallType::Missing {
                    print!("{}", SPACE);
                }
                print!("{}", BLOCK.yellow());
            }
            println!("");
            flat_idx += 1;
        }
        turn = !turn;
    }

    // Print bottom wall.
    for _i in 0..(2 * cols + 1) {
        print!("{}", BLOCK.yellow());
    }
    println!("");
    println!("");
}
