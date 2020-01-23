pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    markers: Vec<bool>, // Store as one dimensional array
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
}

#[cfg(test)]
mod matrix_tests {
    //
}
