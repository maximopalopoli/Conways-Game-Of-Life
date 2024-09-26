pub mod grid;
use grid::Grid;
use macroquad::prelude::*;
use macroquad::ui::*;
use std::env;

const FLOOR_X: f32 = 10.0;
const FLOOR_Y: f32 = 50.0;
const TEXT_FONT_SIZE: f32 = 20.0;
const MATRIX_STEP_X: f32 = 20.0;
const MATRIX_STEP_Y: f32 = 20.0;
const SQUARE_LENGTH: f32 = 20.0;
const SQUARE_OFFSET: f32 = 5.0;
const ARROW_WIDTH: f32 = 5.5;

/// Prints the numbers of the Y axis, the Y and the arrow
fn draw_y_axis(grid_width: usize) {
    for i in 0..grid_width {
        draw_text(
            i.to_string().as_str(),
            FLOOR_X + MATRIX_STEP_X + (i as f32) * 20.0 + (i as f32) * SQUARE_OFFSET,
            FLOOR_Y + 20.0,
            TEXT_FONT_SIZE,
            BLACK,
        );
    }
    let common_x1 =
        FLOOR_X + MATRIX_STEP_X + (grid_width as f32) * 20.0 + (grid_width as f32) * SQUARE_OFFSET
            - 5.0;
    draw_line(
        common_x1,
        FLOOR_Y + 15.0,
        common_x1 + 12.5,
        FLOOR_Y + 15.0,
        2.0,
        BLACK,
    );
    draw_line(
        common_x1 + ARROW_WIDTH,
        FLOOR_Y + 10.0,
        common_x1 + 12.5,
        FLOOR_Y + 15.0,
        2.0,
        BLACK,
    );
    draw_line(
        common_x1 + ARROW_WIDTH,
        FLOOR_Y + 20.0,
        common_x1 + 12.5,
        FLOOR_Y + 15.0,
        2.0,
        BLACK,
    );
    draw_text("Y", common_x1 + 15.0, FLOOR_Y + 20.0, TEXT_FONT_SIZE, BLACK);
}

/// Prints the numbers of the X axis, the X and the arrow
fn draw_x_axis(grid_height: usize) {
    for i in 0..grid_height {
        draw_text(
            i.to_string().as_str(),
            FLOOR_X,
            FLOOR_Y + MATRIX_STEP_Y + 20.0 + (i as f32) * 20.0 + (i as f32) * SQUARE_OFFSET,
            TEXT_FONT_SIZE,
            BLACK,
        );
    }
    let common_y1 = FLOOR_Y
        + MATRIX_STEP_Y
        + (grid_height as f32) * 20.0
        + (grid_height as f32) * SQUARE_OFFSET
        - 5.0;
    draw_line(
        FLOOR_X + 10.0,
        common_y1 + 5.,
        FLOOR_X + 10.0,
        common_y1 + 17.5,
        2.0,
        BLACK,
    );
    draw_line(
        FLOOR_X + 5.0,
        common_y1 + 5. + ARROW_WIDTH,
        FLOOR_X + 10.0,
        common_y1 + 17.5,
        2.0,
        BLACK,
    );
    draw_line(
        FLOOR_X + 15.0,
        common_y1 + 5. + ARROW_WIDTH,
        FLOOR_X + 10.0,
        common_y1 + 17.5,
        2.0,
        BLACK,
    );
    draw_text(
        "X",
        FLOOR_X + 2.5,
        FLOOR_Y
            + MATRIX_STEP_Y
            + 25.5
            + (grid_height as f32) * 20.0
            + (grid_height as f32) * SQUARE_OFFSET,
        TEXT_FONT_SIZE,
        BLACK,
    );
}

/// Draws the grid, when the cells are alive draws a black square, and if not, a blank one
fn draw_grid(grid: &mut Grid) {
    let (grid_width, grid_height) = grid.dimensions();

    draw_y_axis(grid_width);

    draw_x_axis(grid_height);

    // To draw the cells
    for x in 0..grid_width {
        for y in 0..grid_height {
            match grid.at(x, y){  
                Err(error) => {
                    println!(
                        "Error: {}. The bounds are: width: {}, height: {}",
                        error,
                        grid.dimensions().0,
                        grid.dimensions().1
                    );
                },
                Ok(res) => {                    
                    if res {
                        draw_rectangle(
                            FLOOR_X
                                + MATRIX_STEP_X
                                + (x as f32) * SQUARE_LENGTH
                                + (x as f32) * SQUARE_OFFSET,
                            FLOOR_Y
                                + MATRIX_STEP_Y
                                + (y as f32) * SQUARE_LENGTH
                                + (y as f32) * SQUARE_OFFSET,
                            SQUARE_LENGTH,
                            SQUARE_LENGTH,
                            BLACK,
                        );
                    } else {
                        draw_rectangle(
                            FLOOR_X
                                + MATRIX_STEP_X
                                + (x as f32) * SQUARE_LENGTH
                                + (x as f32) * SQUARE_OFFSET,
                            FLOOR_Y
                                + MATRIX_STEP_Y
                                + (y as f32) * SQUARE_LENGTH
                                + (y as f32) * SQUARE_OFFSET,
                            SQUARE_LENGTH,
                            SQUARE_LENGTH,
                            BLANK,
                        );
                    }
                },
            }
        }
    }
}

/// Here remains the logic of the windows of options on the right side of the screen.
/// The main window handles the advance options and the reset one. The second handles
/// the setting of the time between generations, and the third shows the last grid that
/// had a change of state (and handles the change)
pub fn windows_logic(
    grid: &mut Grid,
    generation: &mut i32,
    auto_advance: &mut bool,
    chosen_cell: &mut (usize, usize),
    time_sleep: &mut f32,
) {
    let (grid_width, grid_height) = grid.dimensions();

    widgets::Window::new(hash!(), vec2(575., 50.), vec2(220., 170.))
        .label("Options")
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            if ui.button(Vec2::new(10., 10.), "Next Generation") {
                grid.clock();
                *generation += 1;
            }
            if ui.button(Vec2::new(10., 50.), "Automatic advance") {
                *auto_advance = true;
            }
            if ui.button(Vec2::new(10., 90.), "Stop automatic advance") {
                *auto_advance = false;
            }
            if ui.button(Vec2::new(10., 130.), "Clear") {
                grid.reset();
                *generation = 0;
                *auto_advance = false;
                chosen_cell.0 = 0;
                chosen_cell.0 = 0;
            }
        });

    widgets::Window::new(hash!(), vec2(575., 240.), vec2(220., 50.))
        .label("Set time between generations")
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            ui.slider(hash!(), "[0 .. 5] secs", 0f32..5f32, time_sleep);
        });

    widgets::Window::new(hash!(), vec2(575., 300.), vec2(220., 40.))
        .label("Grid change")
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                // Note: the literals on the upper bounds are related to the common_x and common_y values
                // of the drawing axes functions. Should be replaced in future versions
                if mouse_x > 30. && mouse_x < 525. && mouse_y > 70. && mouse_y < 565. {
                    let x_position = mouse_x - 30.0;
                    let y_position = mouse_y - 70.0;
                    chosen_cell.0 = (x_position / 25.) as usize;
                    chosen_cell.1 = (y_position / 25.) as usize;

                    if chosen_cell.0 < grid_width && chosen_cell.1 < grid_height {
                        *auto_advance = false;
                        grid.change_state_click(chosen_cell.0, chosen_cell.1);
                    }
                }
            }

            ui.label(
                None,
                &format!("Chosen cell: {} {}", chosen_cell.0, chosen_cell.1),
            );
        });
}

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
                let x: usize = pair[0]
                    .parse()
                    .expect("Error parsing first coordinate value");
                let y: usize = pair[1]
                    .parse()
                    .expect("Error parsing second coordinate value");
                (x, y)
            })
            .collect()
    } else {
        Vec::new()
    };

    let mut grid = Grid::new(20, 20);

    if let Err(error) = grid.seed(points) {
        println!(
            "Error: {}. The bounds are: width: {}, height: {}",
            error,
            grid.dimensions().0,
            grid.dimensions().1
        );
        return;
    }

    let mut generation = 0;
    let mut acc_time = 0.;
    let mut auto_advance = false;
    let mut time_sleep = 1.0;
    let mut chosen_cell = (0, 0);

    loop {
        clear_background(WHITE);
        let text = "Generation Number ".to_string() + generation.to_string().as_str();
        draw_text(&text, 10.0, 30.0, 40.0, BLACK);

        draw_grid(&mut grid);

        windows_logic(
            &mut grid,
            &mut generation,
            &mut auto_advance,
            &mut chosen_cell,
            &mut time_sleep,
        );

        let frame_t = get_frame_time();
        acc_time += frame_t;

        // For waiting until the time between iterations has been elapsed
        if auto_advance && acc_time > time_sleep {
            grid.clock();
            generation += 1;
            acc_time = 0.;
        }

        next_frame().await
    }
}
