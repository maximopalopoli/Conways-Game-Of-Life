pub mod grid;
use grid::Grid;

fn main() {
    let mut grid = Grid::new(10, 10);
    let points = vec![(5, 4), (4, 4), (3, 4), (4, 3), (4, 5)];
    grid.seed(points);
    grid.start();
}
