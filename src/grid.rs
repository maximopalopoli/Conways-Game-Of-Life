use std::{thread::sleep, time::Duration};

pub struct Grid {
    matrix: Vec<Vec<u8>>,
    width:  usize,
    height: usize
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Vec<u8>> = Vec::with_capacity(width);
        for _ in 0..width {
            cells.push(vec![0;height]);
        }

        Self { matrix: cells, width, height }
    }

    pub fn seed(&mut self , points: Vec<(i32, i32)>){
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

        for (x, row) in new_matrix.iter_mut().enumerate().take(self.width) {
            for (y, field) in row.iter_mut().enumerate().take(self.width) {
                let neighbours_ammount = self.count_neighbours(x, y);
                if ((neighbours_ammount == 2 || neighbours_ammount == 3) && *field == 1) || (neighbours_ammount == 3 && *field == 0) {
                    *field = 1;
                } else {
                    *field = 0;
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
        println!();
        println!("--|--------------------");

        for x in 0..self.width {
            print!("{} |", x);
            for y in 0..self.height {
                if self.matrix[x][y] == 1 { print!("X ")} else { print!("  ")}
                //print!(" {} ", self.matrix[x][y]);
            }
            println!();
        }
        println!();
    }

    pub fn start (&mut self) {
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
    use super::Grid;

    #[test]
    fn test_01_new_returns_a_valid_matrix() {
        // Create a matrix and it begins initialized with the right ammount of columns and rows, with al the fields in 0
        let grid = Grid::new(10, 10);
        assert_eq!(grid.height, grid.matrix.len());
        assert_eq!(grid.width, grid.matrix.get(0).unwrap().len());
        assert_eq!(0, grid.matrix[0][0]);
    }

    #[test]
    fn test_02_seed_makes_live_passed_points() {
        // As seed I can give the grid a vector of points, and those points are set in 1 
        let mut grid = Grid::new(10, 10);
        let points = vec![(4, 4), (3, 4)];
        grid.seed(points);

        assert_eq!(1, grid.matrix[4][4]);
        assert_eq!(1, grid.matrix[3][4]);
    }

    #[test]
    fn test_03_clock_makes_die_cells_with_underpopulation(){
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
    fn test_04_clock_doesnt_make_die_cells_without_underpopulation_or_overpopulation(){
        // When making a clock, the points that have at least 2 neighbours survive
        let mut grid = Grid::new(10, 10);
        let points = vec![(5, 4), (4, 4), (3, 4)];
        grid.seed(points);

        assert_eq!(1, grid.matrix[4][4]);


        grid.clock();

        assert_eq!(1, grid.matrix[4][4]);
    }

    #[test]
    fn test_05_clock_makes_die_cells_with_overpopulation(){
        // When making a clock, the points that have more than 3 neighbours die
        let mut grid = Grid::new(10, 10);
        let points = vec![(5, 4), (4, 4), (3, 4), (4,3), (5,3)];
        grid.seed(points);

        assert_eq!(1, grid.matrix[4][4]);

        grid.clock();

        assert_eq!(0, grid.matrix[4][4]);
    }

    #[test]
    fn test_06_clock_revives_dead_cells_with_exactly_three_neighbours(){
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

    #[test]
    fn test_07_scheme_is_the_expected_after_six_generations(){
        // Try some generations with a certain seed
        let mut grid = Grid::new(10, 10);
        let points = vec![(3, 4), (4, 4), (5, 4), (4,3), (4,5)];
        grid.seed(points);

        grid.clock();
        grid.clock();
        grid.clock();
        grid.clock();
        grid.clock();
        grid.clock(); // Generation N˚ 6

        assert_eq!(1, grid.matrix[1][3]);
        assert_eq!(1, grid.matrix[1][4]);
        assert_eq!(1, grid.matrix[1][5]);

        assert_eq!(1, grid.matrix[3][1]);
        assert_eq!(1, grid.matrix[4][1]);
        assert_eq!(1, grid.matrix[5][1]);

        assert_eq!(1, grid.matrix[7][3]);
        assert_eq!(1, grid.matrix[7][4]);
        assert_eq!(1, grid.matrix[7][5]);

        assert_eq!(1, grid.matrix[3][7]);
        assert_eq!(1, grid.matrix[4][7]);
        assert_eq!(1, grid.matrix[5][7]);
    }
}
