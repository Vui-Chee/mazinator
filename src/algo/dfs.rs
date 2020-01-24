use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::structure::matrix::*;
use crate::structure::wall::*;

pub fn dfs(mat: &mut Matrix, walls: &mut Walls, i: usize, j: usize) {
    if i >= mat.rows {
        panic!("i == {} is Out of bounds", i);
    }

    if j >= mat.cols {
        panic!("j == {} is Out of bounds", j);
    }

    // Visit current cell.
    mat.put(i, j, true);
    // println!("{} {}", i, j);

    let mut rng = thread_rng();
    let mut possibilities = [1, 2, 3, 4];
    possibilities.shuffle(&mut rng);

    for ticket in possibilities.iter() {
        // For valid adjacent cell, call dfs.
        // Only lateral movement is permitted.
        if *ticket == 1 && i >= 1 && !mat.at(i - 1, j) {
            walls.remove_wall(i - 1, j, false);
            dfs(mat, walls, i - 1, j);
        }
        if *ticket == 2 && i + 1 < mat.rows && !mat.at(i + 1, j) {
            walls.remove_wall(i, j, false);
            dfs(mat, walls, i + 1, j);
        }
        if *ticket == 3 && j >= 1 && !mat.at(i, j - 1) {
            walls.remove_wall(j - 1, i, true);
            dfs(mat, walls, i, j - 1);
        }
        if *ticket == 4 && j + 1 < mat.cols && !mat.at(i, j + 1) {
            walls.remove_wall(j, i, true);
            dfs(mat, walls, i, j + 1);
        }
    }
}

#[cfg(test)]
mod dfs_test {
    use super::*;

    #[test]
    fn visited_all_cells() {
        let rows = 5;
        let cols = 5;
        let mut mat = Matrix::new(rows, cols);
        let mut walls = Walls::new(rows - 1, cols - 1);
        dfs(&mut mat, &mut walls, 1, 3);
        for i in 0..mat.rows {
            for j in 0..mat.cols {
                assert_eq!(mat.at(i, j), true);
            }
        }
    }
}
