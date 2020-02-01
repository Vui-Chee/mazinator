use mazinator::algo::bfs::*;
use mazinator::algo::dfs::*;
use mazinator::structure::matrix::*;
use mazinator::structure::wall::*;

fn visit_all_cells_check(algo: &dyn Fn(&mut Matrix, &mut Walls, usize, usize)) {
    let rows = 5;
    let cols = 5;
    let mut mat = Matrix::new(rows, cols);
    let mut walls = Walls::new(rows - 1, cols - 1);
    algo(&mut mat, &mut walls, 1, 3);
    for i in 0..mat.rows {
        for j in 0..mat.cols {
            assert_eq!(mat.at(i, j), true);
        }
    }
}

#[test]
fn visited_all_cells() {
    visit_all_cells_check(&bfs);
    visit_all_cells_check(&dfs);
}
