#[derive(Debug, PartialEq)]
pub enum WallType {
    Horizontal,
    Vertical,
    Missing,
}

fn create_walls_type(a: usize, b: usize, walltype: WallType) -> Vec<Vec<WallType>> {
    let mut walls = vec![];

    for i in 0..a {
        walls.push(vec![]);
        for _j in 0..b + 1 {
            match walltype {
                WallType::Horizontal => walls[i].push(WallType::Horizontal),
                WallType::Vertical => walls[i].push(WallType::Vertical),
                WallType::Missing => walls[i].push(WallType::Missing),
            }
        }
    }

    walls
}

/// This refer to the inner walls,
/// the outer walls are not passable.
///
/// There are (rows-1) * (cols) inner flat walls
/// and (cols-1) * (rows) inner side walls.
#[derive(Debug)]
pub struct Walls {
    pub flat_walls: Vec<Vec<WallType>>,
    pub side_walls: Vec<Vec<WallType>>,
}

impl Walls {
    pub fn new(n: usize, m: usize) -> Walls {
        let flat_walls = create_walls_type(n, m, WallType::Horizontal);
        let side_walls = create_walls_type(m, n, WallType::Vertical);
        Walls {
            flat_walls,
            side_walls,
        }
    }

    pub fn remove_wall(&mut self, i: usize, j: usize, left_or_right: bool) {
        if left_or_right {
            // Side walls removal
            self.side_walls[i][j] = WallType::Missing;
        } else {
            // Flat walls removal
            self.flat_walls[i][j] = WallType::Missing;
        }
    }
}

#[cfg(test)]
mod wall_tests {
    use super::*;

    #[test]
    fn constructed_correct_walls() {
        let rows = 5;
        let cols = 7;
        // Should produce rows - 1 vectors of vectors with cols number of WallType.
        let flat_walls = create_walls_type(rows - 1, cols - 1, WallType::Horizontal);
        assert!(flat_walls.len() == rows - 1);
        assert!(flat_walls[0].len() == cols);
        // Also all WallType must be horizontal.
        for row_vec in flat_walls.iter() {
            for wall in row_vec.iter() {
                assert_eq!(*wall, WallType::Horizontal);
            }
        }

        // Should produce cols - 1 vectors of vectors with rows number of WallType.
        let side_walls = create_walls_type(cols - 1, rows - 1, WallType::Vertical);
        assert!(side_walls.len() == cols - 1);
        assert!(side_walls[0].len() == rows);
        // Also all WallType must be horizontal.
        for row_vec in side_walls.iter() {
            for wall in row_vec.iter() {
                assert_eq!(*wall, WallType::Vertical);
            }
        }
    }

    #[test]
    fn remove_wall_correctly() {
        // Two possible cases, remove flat or side wall.
        // Removing a wall involves toggling the enum.
        let rows = 5;
        let cols = 7;
        let mut inner_walls = Walls::new(rows - 1, cols - 1);
        // Remove side wall
        inner_walls.remove_wall(0, 0, true);
        assert_eq!(inner_walls.side_walls[0][0], WallType::Missing);
        // Remove flat wall
        inner_walls.remove_wall(1, 3, false);
        assert_eq!(inner_walls.flat_walls[1][3], WallType::Missing);
    }
}
