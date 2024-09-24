pub mod grid;
use grid::Grid;
use std::env;
use macroquad::prelude::*;
use macroquad::ui::*;

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
    if args.len() < 2 {
        println!("Usage: cargo run start 1y 1x 2y 2x ... Ny Nx");
        return;
    }

    let points: Vec<(usize, usize)> = if args.len() >= 4 {
        args[2..]
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
        .collect()
    } else {
        Vec::new()
    };

    let mut grid = Grid::new(20, 20);

    grid.seed(points);

    let mut generation = 0;
    let mut auto_advance = false;
    let mut time_sleep = 1.0;
    let mut acc_time = 0.;
    let mut chosen_cell_x = 0;
    let mut chosen_cell_y = 0;

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

        widgets::Window::new(hash!(), vec2(575., 50.), vec2(220., 170.))
        .label("Options")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            if ui.button(Vec2::new(10., 10.), "Next Generation") {
                grid.clock();
                generation += 1;
            }
            if ui.button(Vec2::new(10., 50.), "Automatic advance"){
                auto_advance = true;
            }
            if ui.button(Vec2::new(10., 90.), "Stop automatic advance"){
                auto_advance = false;
            }           
            if ui.button(Vec2::new(10., 130.), "Clear"){
                grid = Grid::new(grid_width, grid_height);
                generation = 0;
                auto_advance = false;
            }
        });

        widgets::Window::new(hash!(), vec2(575., 240.), vec2(220., 50.))
        .label("Set time between generations")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
                ui.slider(hash!(), "[0 .. 100]", 0f32..10f32, &mut time_sleep);
        });

        
        widgets::Window::new(hash!(), vec2(575., 300.), vec2(220., 40.))
        .label("Grid change")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                let x_position = mouse_x - 30.0;
                let y_position = mouse_y - 70.0;
                chosen_cell_x = (x_position/25.) as usize;
                chosen_cell_y = (y_position/25.) as usize;

                if chosen_cell_x<grid_width && chosen_cell_y<grid_height {
                    auto_advance = false;
                    println!("Chosen: {chosen_cell_x}, {chosen_cell_y}, Medidas: {grid_width}, {grid_height}.");
                    grid.change_state_click(chosen_cell_x, chosen_cell_y);
                }
            }

            ui.label(None, &format!("Chosen cell: {} {}", chosen_cell_x, chosen_cell_y));
        });


        let frame_t = get_frame_time();
        acc_time += frame_t;

        if auto_advance {
            if acc_time > time_sleep {
                grid.clock();
                generation += 1;
                acc_time = 0.;
            }
        }

        next_frame().await
    }
}

/*
    Remaining:
    - Center the numbers
    - Lines between the cells
    - Custom Tables
*/
