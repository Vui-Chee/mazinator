use std::cmp::min;
use std::collections::VecDeque;

use crate::structure::matrix::*;
use crate::structure::types::*;
use crate::structure::wall::*;

pub fn bfs(mat: &mut Matrix, walls: &mut Walls, i: usize, j: usize) {
    if i >= mat.rows || j >= mat.cols {
        return;
    }

    let mut queue = VecDeque::<(Location, Option<Location>)>::new();
    queue.push_back(((i, j), None));
    mat.put(i, j, true); // Start cell is immediately visited.

    while !queue.is_empty() {
        // Present and optional last locations.
        let (curr_loc, prev_loc) = queue.pop_front().unwrap();

        // Remove wall from this location if not visited.
        if prev_loc != None {
            // First determine which direction
            let left_or_right = curr_loc.1 != prev_loc.unwrap().1;
            if left_or_right {
                // Remove wall between current and last known location.
                walls.remove_wall(
                    min(curr_loc.1, prev_loc.unwrap().1),
                    curr_loc.0,
                    left_or_right,
                );
            } else {
                walls.remove_wall(
                    min(curr_loc.0, prev_loc.unwrap().0),
                    curr_loc.1,
                    left_or_right,
                );
            }
        }

        // Get permissible neighbours.
        let locations = mat.get_neighbours(curr_loc.0, curr_loc.1);
        // For each of its neighbours, add them to the queue.
        for neighbour in locations.iter() {
            if !mat.at(neighbour.0, neighbour.1) {
                // Must mark location otherwise, same location
                // can be placed in the queue.
                mat.put(neighbour.0, neighbour.1, true);
                queue.push_back((*neighbour, Some(curr_loc)));
            }
        }
    }
}

#[cfg(test)]
mod bfs_test {
    use super::*;

    #[test]
    fn visited_all_cells() {
        let rows = 5;
        let cols = 5;
        let mut mat = Matrix::new(rows, cols);
        let mut walls = Walls::new(rows - 1, cols - 1);
        bfs(&mut mat, &mut walls, 1, 3);
        for i in 0..mat.rows {
            for j in 0..mat.cols {
                assert_eq!(mat.at(i, j), true);
            }
        }
    }
}
