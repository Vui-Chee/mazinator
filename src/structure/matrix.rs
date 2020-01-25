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
    // use super::*;
    //
    // fn get_correct_neighbours() {
    // //
    // }
}
