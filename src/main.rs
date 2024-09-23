
fn main() {

    println!("Hello, world!");
}

struct Grid {
    matrix: Vec<Vec<u8>>,
    width:  usize,
    height: usize
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Vec<u8>> = Vec::with_capacity(width);
        for _ in 0..width {
            let mut column = Vec::with_capacity(height);
            for _ in 0..height {
                column.push(0);
            }
            
            cells.push(column);
        }

        Self { matrix: cells, width, height }
    }

    fn seed(&mut self , points: Vec<(i32, i32)>){
        for point in points{
            self.matrix[point.0 as usize][point.1 as usize] = 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_01() {
        // Create a matrix and it begins initialized with the right ammount of columns and rows, with al the fields in 0
        let grid = Grid::new(10, 10);
        assert_eq!(grid.height, grid.matrix.len());
        assert_eq!(grid.width, grid.matrix.get(0).unwrap().len());
        assert_eq!(0, grid.matrix[0][0]);
    }

    #[test]
    fn test_02() {
        // As seed I can give the grid a vector of points, and those points are set in 1 
        let mut grid = Grid::new(10, 10);
        let points = vec![(4, 4), (3, 4), (5, 4)];
        grid.seed(points);

        assert_eq!(1, grid.matrix[4][4]);
        assert_eq!(1, grid.matrix[3][4]);
        assert_eq!(1, grid.matrix[5][4]);
    }
}
