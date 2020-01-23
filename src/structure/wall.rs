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
    //
}
