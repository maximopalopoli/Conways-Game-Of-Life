use std::{thread::sleep, time::Duration};


fn main() {
    let mut grid = Grid::new(10, 10);
    let points = vec![(5, 4), (4, 4), (3, 4), (4, 3), (4, 5)];
    grid.seed(points);
    grid.start();
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

    fn count_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        count += if x > 0 {
            let mut partial_count = 0;

            partial_count += if y > 0 { if self.matrix[x-1][y-1] == 1 { 1 } else { 0 } } else { 0 };

            if self.matrix[x-1][y] == 1 {
                partial_count += 1;
            }

            partial_count += if y < (self.width - 1) { if self.matrix[x-1][y+1] == 1 { 1 } else { 0 } } else { 0 };

            partial_count
        } else { 0 };

        count += if y > 0 {
                let mut partial_count = 0;

                if self.matrix[x][y-1] == 1 {
                    partial_count += 1;
                }
                partial_count += if x < (self.height - 1) { if self.matrix[x+1][y-1] == 1 {1} else { 0 } } else { 0 };

                partial_count
        } else { 0 }; 

        count += if x < (self.height - 1) {
                let mut partial_count = 0;

                partial_count += if self.matrix[x+1][y] == 1 { 1 } else { 0 };
                
                partial_count += if y < (self.width - 1) {if self.matrix[x+1][y+1] == 1 { 1 } else { 0 } } else { 0 };

                partial_count
        } else { 0 };

        count += if y < (self.width - 1) { if self.matrix[x][y+1] == 1 { 1 } else { 0 } } else { 0 };
    
        count
    }

    fn clock(&mut self){
        let mut new_matrix = self.matrix.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbours_ammount = self.count_neighbours(x, y);
                if neighbours_ammount < 2 && new_matrix[x][y] == 1 {
                    new_matrix[x][y] = 0;
                }
                if (neighbours_ammount == 2 || neighbours_ammount == 3) && new_matrix[x][y] == 1 {
                    new_matrix[x][y] = 1;
                }
                if neighbours_ammount > 3 && new_matrix[x][y] == 1 {
                    new_matrix[x][y] = 0;
                }
                if neighbours_ammount < 2 && new_matrix[x][y] == 1 {
                    new_matrix[x][y] = 0;
                }
                if neighbours_ammount == 3 && new_matrix[x][y] == 0 {
                    new_matrix[x][y] = 1;
                }
            }
        }

        self.matrix = new_matrix;
    }

    fn print(&self) {
        print!("  |");
        for x in 0..self.width {
            print!("{} ", x);
        }
        print!("\n");
        print!("--|------------------------------\n");

        for x in 0..self.width {
            print!("{} |", x);
            for y in 0..self.height {
                if self.matrix[x][y] == 1 { print!("X ")} else { print!("  ")}
                //print!(" {} ", self.matrix[x][y]);
            }
            print!("\n");
        }
        print!("\n");
    }

    fn start (&mut self) {
        let mut generation = 0;
        
    
        loop {
            println!("Generation N˚ {}\n", generation);
            self.print();
            self.clock();
            generation += 1;
            sleep(Duration::from_secs(3));
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
        let points = vec![(4, 4), (3, 4)];
        grid.seed(points);

        assert_eq!(1, grid.matrix[4][4]);
        assert_eq!(1, grid.matrix[3][4]);
    }

    #[test]
    fn test_03(){
        // When making a clock, the points that don't have at least 2 neighbours die
        let mut grid = Grid::new(10, 10);
        let points = vec![(4, 4), (3, 4)];
        grid.seed(points);

        assert_eq!(1, grid.matrix[4][4]);
        assert_eq!(1, grid.matrix[3][4]);

        grid.clock();

        assert_eq!(0, grid.matrix[4][4]);
        assert_eq!(0, grid.matrix[3][4]);
    }

    #[test]
    fn test_04(){
        // When making a clock, the points that have at least 2 neighbours survive
        let mut grid = Grid::new(10, 10);
        let points = vec![(5, 4), (4, 4), (3, 4)];
        grid.seed(points);

        assert_eq!(1, grid.matrix[4][4]);


        grid.clock();

        assert_eq!(1, grid.matrix[4][4]);
    }

    #[test]
    fn test_05(){
        // When making a clock, the points that have more than 3 neighbours die
        let mut grid = Grid::new(10, 10);
        let points = vec![(5, 4), (4, 4), (3, 4), (4,3), (5,3)];
        grid.seed(points);

        assert_eq!(1, grid.matrix[4][4]);

        grid.clock();

        assert_eq!(0, grid.matrix[4][4]);
    }

    #[test]
    fn test_06(){
        // When making a clock, the dead points that have exactly 3 live neighbours revives
        let mut grid = Grid::new(10, 10);
        let points = vec![(3, 4), (4, 4), (5, 4)];
        grid.seed(points);

        assert_eq!(0, grid.matrix[4][3]);
        assert_eq!(0, grid.matrix[4][5]);

        grid.clock();

        assert_eq!(1, grid.matrix[4][3]);
        assert_eq!(1, grid.matrix[4][5]);
    }
}
