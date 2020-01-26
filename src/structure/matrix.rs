use rand::seq::SliceRandom;
use rand::thread_rng;

use super::types::*;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub markers: Vec<bool>, // Store as one dimensional array
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let mut markers: Vec<bool> = vec![];
        for _i in 0..(rows * cols) {
            markers.push(false);
        }

        Matrix {
            rows,
            cols,
            markers,
        }
    }

    /// Returns 1-D index into markers
    pub fn index(&self, i: usize, j: usize) -> usize {
        i * self.cols + j
    }

    pub fn at(&self, i: usize, j: usize) -> bool {
        self.markers[self.index(i, j)]
    }

    pub fn put(&mut self, i: usize, j: usize, val: bool) {
        let idx = self.index(i, j);
        self.markers[idx] = val;
    }

    /// Returns a randomized vec of possible locations surrounding
    /// cell.
    pub fn get_neighbours(&self, i: usize, j: usize) -> Vec<Location> {
        let mut locations = vec![];
        if i >= 1 {
            locations.push((i - 1, j));
        }
        if i + 1 < self.rows {
            locations.push((i + 1, j));
        }
        if j >= 1 {
            locations.push((i, j - 1));
        }
        if j + 1 < self.cols {
            locations.push((i, j + 1));
        }

        // Shuffle slice
        let mut rng = thread_rng();
        locations.as_mut_slice().shuffle(&mut rng);

        locations
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn get_correct_index() {
        let mat = Matrix::new(5, 5);
        assert_eq!(mat.index(0, 0), 0);
        assert_eq!(mat.index(4, 4), 24);
        assert_eq!(mat.index(2, 2), 12);
        assert_eq!(mat.index(3, 4), 19);
        assert_eq!(mat.index(1, 2), 7);
    }

    #[test]
    fn put_correct_value() {
        let mut mat = Matrix::new(5, 5);
        mat.put(1, 3, true);
        assert!(mat.at(1, 3));
        mat.put(2, 4, true);
        assert!(mat.at(2, 4));
        mat.put(0, 3, true);
        assert!(mat.at(0, 3));

        // Check remaining markers are still false.
        let mut index = 0;
        for marker in mat.markers.iter() {
            if index != (1 * 5 + 3) && index != (2 * 5 + 4) && index != (0 * 5 + 3) {
                assert!(!marker);
            }
            index += 1;
        }
    }

    #[test]
    fn get_correct_neighbours() {
        let mat = Matrix::new(5, 5);

        // CORNERS
        // top left corner
        let mut locations = mat.get_neighbours(0, 0);
        locations.sort();
        assert_eq!(locations, vec![(0, 1), (1, 0)]);
        // top right corner
        let mut locations = mat.get_neighbours(0, 4);
        locations.sort();
        assert_eq!(locations, vec![(0, 3), (1, 4)]);
        // bottom left corner
        let mut locations = mat.get_neighbours(4, 0);
        locations.sort();
        assert_eq!(locations, vec![(3, 0), (4, 1)]);
        // bottom right corner
        let mut locations = mat.get_neighbours(4, 4);
        locations.sort();
        assert_eq!(locations, vec![(3, 4), (4, 3)]);

        // EDGES
        // top edge
        let mut locations = mat.get_neighbours(0, 2);
        locations.sort();
        assert_eq!(locations, vec![(0, 1), (0, 3), (1, 2)]);
        // left edge
        let mut locations = mat.get_neighbours(2, 0);
        locations.sort();
        assert_eq!(locations, vec![(1, 0), (2, 1), (3, 0)]);
        // right edge
        let mut locations = mat.get_neighbours(2, 4);
        locations.sort();
        assert_eq!(locations, vec![(1, 4), (2, 3), (3, 4)]);
        // bottom edge
        let mut locations = mat.get_neighbours(4, 2);
        locations.sort();
        assert_eq!(locations, vec![(3, 2), (4, 1), (4, 3)]);

        // center
        let mut locations = mat.get_neighbours(2, 2);
        locations.sort();
        assert_eq!(locations, vec![(1, 2), (2, 1), (2, 3), (3, 2)]);
    }
}
