pub mod grid;
use grid::Grid;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run start 1y 1x 2y 2x ... Ny Nx");
        return;
    }

    let points: Vec<(usize, usize)> = args[2..]
        .chunks(2)
        .map(|pair| {
            let x: usize = pair[0].parse().expect(
                "Error parseando el primer valor de la coordenada",
            );
            let y: usize = pair[1].parse().expect(
                "Error parseando el segundo valor de la coordenada",
            );
            (x, y)
        })
        .collect();

    let mut grid = Grid::new(10, 10);

    //let points = vec![(5, 4), (4, 4), (3, 4), (4, 3), (4, 5)];
    grid.seed(points);
    grid.start();
}
