use colored::*;
use rand::Rng;

use crate::structure::wall::*;

const BLOCK: &str = "\u{2588}\u{2588}";

fn space_color(idx: &mut usize) -> ColoredString {
    let tmp_idx = *idx;
    let space_colors = ["yellow", "purple", "red", "blue", "green", "cyan"];
    *idx = (*idx + 1) % space_colors.len();
    match space_colors[tmp_idx] {
        "yellow" => BLOCK.yellow(),
        "purple" => BLOCK.purple(),
        "red" => BLOCK.red(),
        "magenta" => BLOCK.magenta(),
        "green" => BLOCK.green(),
        "cyan" => BLOCK.cyan(),
        "blue" => BLOCK.blue(),
        _ => BLOCK.normal(),
    }
}

pub fn generate_maze(cols: usize, walls: Walls) {
    let block_colors = ["black", "grey", "white"];
    let num = rand::thread_rng().gen_range(0, block_colors.len());
    let color_block = || match block_colors[num] {
        "black" => BLOCK.black(),
        "grey" => BLOCK.black().bold(),
        "white" => BLOCK.white(),
        _ => BLOCK.black(),
    };

    let mut space_color_idx = 0;

    println!("");
    // print top horizontal wall
    for _i in 0..(2 * cols + 1) {
        print!("{}", color_block());
    }
    println!("");

    let mut turn = true; // true == vertical walls turn
    let mut side_idx = 0;
    let mut flat_idx = 0;
    let total_iterations = walls.flat_walls.len() + walls.side_walls[0].len();
    for _i in 0..total_iterations {
        if turn {
            print!("{}", color_block());
            print!("{}", space_color(&mut space_color_idx));
            for wall_vector in walls.side_walls.iter() {
                if wall_vector[side_idx] == WallType::Vertical {
                    print!("{}", color_block());
                    print!("{}", space_color(&mut space_color_idx));
                } else if wall_vector[side_idx] == WallType::Missing {
                    print!("{}", space_color(&mut space_color_idx));
                    print!("{}", space_color(&mut space_color_idx));
                }
            }
            println!("{}", color_block());
            side_idx += 1;
        } else {
            print!("{}", color_block());
            for wall in walls.flat_walls[flat_idx].iter() {
                if *wall == WallType::Horizontal {
                    print!("{}", color_block());
                } else if *wall == WallType::Missing {
                    print!("{}", space_color(&mut space_color_idx));
                }
                print!("{}", color_block());
            }
            println!("");
            flat_idx += 1;
        }
        turn = !turn;
    }

    // Print bottom wall.
    for _i in 0..(2 * cols + 1) {
        print!("{}", color_block());
    }
    println!("");
    println!("");
}
