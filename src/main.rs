pub mod grid;
use grid::Grid;
use std::{env, thread::sleep, time::Duration};
use macroquad::prelude::*;

const PISO_X:f32 = 10.0;
const PISO_Y:f32 = 50.0;
const TEXT_FONT_SIZE:f32 = 20.0;
const MATRIX_STEP_X:f32 = 20.0;
const MATRIX_STEP_Y:f32 = 20.0;
const SQUARE_LENGTH:f32 = 20.0;
const SQUARE_OFFSET:f32 = 5.0;
const ARROW_WIDTH: f32 = 5.5;

#[macroquad::main("GameOfLife")]
async fn main() {
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

    let mut grid = Grid::new(20, 20);

    //let points = vec![(5, 4), (4, 4), (3, 4), (4, 3), (4, 5)];
    grid.seed(points);

    let mut generation = 0;

    loop {
        clear_background(WHITE);
        let text = "Generation Number ".to_string() + generation.to_string().as_str();
        draw_text(&text, 10.0, 30.0, 40.0, BLACK);

        let (grid_width, grid_height) = grid.dimensions();

        // Printea la fila de numeros
        for i in 0..grid_width {
            draw_text(i.to_string().as_str(), PISO_X + MATRIX_STEP_X + (i as f32)*20.0 + (i as f32)*SQUARE_OFFSET, PISO_Y + 20.0, TEXT_FONT_SIZE, BLACK);
            //draw_line(PISO_X + MATRIX_STEP_X + (i as f32)*20.0, PISO_Y + 20.0, PISO_X + MATRIX_STEP_X + (i as f32)*20.0, PISO_Y + 20.0 + SQUARE_LENGTH*19.0, 2.5, BLACK);
            // El problema con las lineas es que todavia no pude calibrar el grueso necesario y la posicion adecuada
        }   
        let common_x1 = PISO_X + MATRIX_STEP_X + (grid_width as f32)*20.0 + (grid_width as f32)*SQUARE_OFFSET - 5.0;
        draw_line(common_x1, PISO_Y + 15.0, common_x1 + 12.5, PISO_Y + 15.0, 2.0, BLACK); // Medio
        draw_line(common_x1 + ARROW_WIDTH, PISO_Y + 10.0, common_x1 + 12.5, PISO_Y + 15.0, 2.0, BLACK);
        draw_line(common_x1 + ARROW_WIDTH, PISO_Y + 20.0, common_x1 + 12.5, PISO_Y + 15.0, 2.0, BLACK);
        draw_text("Y", common_x1 + 15.0 , PISO_Y + 20.0, TEXT_FONT_SIZE, BLACK);


        // Printea la columna de numeros
        for i in 0..grid_height {
            draw_text(i.to_string().as_str(), PISO_X, PISO_Y + MATRIX_STEP_Y + 20.0 + (i as f32)*20.0 + (i as f32)*SQUARE_OFFSET, TEXT_FONT_SIZE, BLACK);
        }
        let common_y1 = PISO_Y + MATRIX_STEP_Y + (grid_width as f32)*20.0 + (grid_width as f32)*SQUARE_OFFSET;
        draw_line(PISO_X + 10.0, common_y1, PISO_X + 10.0, common_y1 + 12.5, 2.0, BLACK);
        draw_line(PISO_X + 5.0, common_y1 + ARROW_WIDTH, PISO_X + 10.0, common_y1 + 12.5, 2.0, BLACK);
        draw_line(PISO_X + 15.0, common_y1 + ARROW_WIDTH, PISO_X + 10.0, common_y1 + 12.5, 2.0, BLACK);
        draw_text("X", PISO_X + 2.5, PISO_Y + MATRIX_STEP_Y + 25.5 + (grid_width as f32)*20.0 + (grid_width as f32)*SQUARE_OFFSET, TEXT_FONT_SIZE, BLACK);

        for x in 0..grid_width {
            for y in 0..grid_height {
                if grid.at(x, y) == 1 {
                    draw_rectangle(PISO_X + MATRIX_STEP_X + (x as f32)*SQUARE_LENGTH + (x as f32)*SQUARE_OFFSET, PISO_Y + MATRIX_STEP_Y + (y as f32)*SQUARE_LENGTH + (y as f32)*SQUARE_OFFSET, SQUARE_LENGTH, SQUARE_LENGTH, BLACK);
                } else {
                    draw_rectangle(PISO_X + MATRIX_STEP_X + (x as f32)*SQUARE_LENGTH + (x as f32)*SQUARE_OFFSET, PISO_Y + MATRIX_STEP_Y + (y as f32)*SQUARE_LENGTH + (y as f32)*SQUARE_OFFSET, SQUARE_LENGTH, SQUARE_LENGTH, BLANK);
                }
            }
        }

        grid.clock();
        generation += 1;

        sleep(Duration::from_secs(3));

        next_frame().await
    }
}

/*
    Remaining:
    - Center the numbers
    - Lines between the cells
    - A simple way for the user to define the seed (like a click on the interface)
    - An alternate way to make the transitions (Like a next button)
*/
