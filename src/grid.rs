use std::ops::Range;
use std::thread::sleep;
use std::time::Duration;

pub struct Grid {
    matrix: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells: Vec<Vec<bool>> = vec![vec![false; height]; width];

        Self {
            matrix: cells,
            width,
            height,
        }
    }

    pub fn seed(&mut self, points: Vec<(usize, usize)>) {
        for point in points {
            self.matrix[point.0][point.1] = true;
        }
    }

    fn limited_range_for_number(number: usize, limit: usize) -> Range<usize> {
        if number == 0 {
            number..(number + 2)
        } else if number == limit - 1 {
            (number - 1)..(number + 1)
        } else {
            (number - 1)..(number + 2)
        }
    }

    fn count_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        let range_x = Grid::limited_range_for_number(x, self.width);

        let range_y = Grid::limited_range_for_number(y, self.height);

        for pos_x in range_x.clone() {
            for pos_y in range_y.clone() {
                if pos_x == x && pos_y == y {
                    continue;
                }

                if self.matrix[pos_x][pos_y] {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn clock(&mut self) {
        let mut new_matrix = self.matrix.clone();

        for (x, row) in new_matrix.iter_mut().enumerate().take(self.width) {
            for (y, field) in row.iter_mut().enumerate().take(self.width) {
                let neighbours_amount = self.count_neighbours(x, y);
                *field = neighbours_amount == 3 || neighbours_amount == 2 && *field;
            }
        }

        self.matrix = new_matrix;
    }

    pub fn print(&self) {
        print!("  |");
        for x in 0..self.width {
            print!("{} ", x);
        }
        println!();
        println!("--|--------------------");

        for x in 0..self.width {
            print!("{} |", x);
            for y in 0..self.height {
                if self.matrix[x][y] {
                    print!("X ")
                } else {
                    print!("  ")
                }
                //print!(" {} ", self.matrix[x][y]);
            }
            println!();
        }
        println!();
    }

    pub fn start(&mut self) {
        let mut generation = 0;

        loop {
            println!("Generation N˚ {}\n", generation);
            self.print();
            self.clock();
            generation += 1;
            sleep(Duration::from_secs(3));
        }
    }

    pub fn at(&self, x: usize, y: usize) -> bool {
        self.matrix[x][y]
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn change_state_click(&mut self, x: usize, y: usize) {
        if x>=self.width || y>=self.height {
            return;
        }

        self.matrix[x][y] = !self.matrix[x][y];
    }

    pub fn reset(&mut self) {
        self.matrix = vec![vec![false; self.height]; self.width];
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test_01_new_returns_a_valid_matrix() {
        // Create a matrix and it begins initialized with the right amount of columns and
        // rows, with al the fields in 0
        let grid = Grid::new(10, 10);
        assert_eq!(grid.height, grid.matrix.len());
        assert_eq!(grid.width, grid.matrix.get(0).unwrap().len());
        assert_eq!(false, grid.matrix[0][0]);
    }

    #[test]
    fn test_02_seed_makes_live_passed_points() {
        // As seed I can give the grid a vector of points, and those points are set in 1
        let mut grid = Grid::new(10, 10);
        let points = vec![(4, 4), (3, 4)];
        grid.seed(points);

        assert_eq!(true, grid.matrix[4][4]);
        assert_eq!(true, grid.matrix[3][4]);
    }

    #[test]
    fn test_03_clock_makes_die_cells_with_underpopulation() {
        // When making a clock, the points that don't have at least 2 neighbours die
        let mut grid = Grid::new(10, 10);
        let points = vec![(4, 4), (3, 4)];
        grid.seed(points);

        assert_eq!(true, grid.matrix[4][4]);
        assert_eq!(true, grid.matrix[3][4]);

        grid.clock();

        assert_eq!(false, grid.matrix[4][4]);
        assert_eq!(false, grid.matrix[3][4]);
    }

    #[test]
    fn test_04_clock_doesnt_kill_cells_without_underpopulation_or_overpopulation() {
        // When making a clock, the points that have at least 2 neighbours survive
        let mut grid = Grid::new(10, 10);
        let points = vec![(5, 4), (4, 4), (3, 4)];
        grid.seed(points);

        assert_eq!(true, grid.matrix[4][4]);


        grid.clock();

        assert_eq!(true, grid.matrix[4][4]);
    }

    #[test]
    fn test_05_clock_makes_die_cells_with_overpopulation() {
        // When making a clock, the points that have more than 3 neighbours die
        let mut grid = Grid::new(10, 10);
        let points = vec![(5, 4), (4, 4), (3, 4), (4, 3), (5, 3)];
        grid.seed(points);

        assert_eq!(true, grid.matrix[4][4]);

        grid.clock();

        assert_eq!(false, grid.matrix[4][4]);
    }

    #[test]
    fn test_06_clock_revives_dead_cells_with_exactly_three_neighbours() {
        // When making a clock, the dead points that have exactly 3 live neighbours revives
        let mut grid = Grid::new(10, 10);
        let points = vec![(3, 4), (4, 4), (5, 4)];
        grid.seed(points);

        assert_eq!(false, grid.matrix[4][3]);
        assert_eq!(false, grid.matrix[4][5]);

        grid.clock();

        assert_eq!(true, grid.matrix[4][3]);
        assert_eq!(true, grid.matrix[4][5]);
    }

    #[test]
    fn test_07_scheme_is_the_expected_after_six_generations() {
        // Try some generations with a certain seed
        let mut grid = Grid::new(10, 10);
        let points = vec![(3, 4), (4, 4), (5, 4), (4, 3), (4, 5)];
        grid.seed(points);

        grid.clock();
        grid.clock();
        grid.clock();
        grid.clock();
        grid.clock();
        grid.clock(); // Generation N˚ 6

        assert_eq!(true, grid.matrix[1][3]);
        assert_eq!(true, grid.matrix[1][4]);
        assert_eq!(true, grid.matrix[1][5]);

        assert_eq!(true, grid.matrix[3][1]);
        assert_eq!(true, grid.matrix[4][1]);
        assert_eq!(true, grid.matrix[5][1]);

        assert_eq!(true, grid.matrix[7][3]);
        assert_eq!(true, grid.matrix[7][4]);
        assert_eq!(true, grid.matrix[7][5]);

        assert_eq!(true, grid.matrix[3][7]);
        assert_eq!(true, grid.matrix[4][7]);
        assert_eq!(true, grid.matrix[5][7]);
    }
}
