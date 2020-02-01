use std::cmp::min;

use crate::structure::matrix::*;
use crate::structure::wall::*;

pub fn dfs(mat: &mut Matrix, walls: &mut Walls, i: usize, j: usize) {
    if i >= mat.rows || j >= mat.cols {
        return;
    }

    // Visit current cell.
    mat.put(i, j, true);

    let locations = mat.get_neighbours(i, j);
    for loc in locations.iter() {
        if !mat.at(loc.0, loc.1) {
            // Movement left or right involves
            // changing of column coordinates.
            let left_or_right = loc.1 != j;
            if left_or_right {
                walls.remove_wall(min(loc.1, j), i, left_or_right);
            } else {
                walls.remove_wall(min(loc.0, i), j, left_or_right);
            }
            dfs(mat, walls, loc.0, loc.1);
        }
    }
}
