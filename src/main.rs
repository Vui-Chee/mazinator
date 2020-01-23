use mazinator::algo::dfs::*;
use mazinator::structure::matrix::*;
use mazinator::structure::wall::*;

use mazinator::display::*;

fn main() {
    let rows = 7;
    let cols = 5;
    let mut mat = Matrix::new(rows, cols);
    let mut walls = Walls::new(rows - 1, cols - 1);
    // println!("{:?}", walls);
    dfs(&mut mat, &mut walls, 4, 2);
    // println!("{:?}", walls);
    generate_maze(rows, cols, walls);
}
