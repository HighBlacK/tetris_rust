//TODO:
//Pretty much everything. --- NOT DONE
//Oh and also, ACTUALLY LEARNING HOW TO USE SDL2 would be nice. --- NOT DONE
//add a velocity/movement speed variable to speed up the tetromino over time. --- NOT DONE
//add a timer to move the tetromino down every second. --- NOT DONE
//Create menus(Main menu, settings, highscores, gameplay, pause, game over, asking for replay, corrupt save date prompt). --- NOT DONE / NEED TO LEARN HOW TO USE SDL2
//Create an UI. --- NOT DONE / NEED TO LEARN HOW TO USE SDL2
//Create a save system. --- DONE / NEEDS TO BE REFINED
//Write the documentation. --- NOT DONE / SHOULD BE DONE LAST

use std::collections::BTreeMap;
use sdl2::VideoSubsystem;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, render::WindowCanvas};
use std::time::Duration;
use std::time::SystemTime;
use timestep::State;
use game_grid::Grid;
use tetromino::Tetromino;
use rand::Rng;
use sdl2::rect::Rect;
use sdl2::{Sdl, EventPump};

use self::error::{TetrisResult, HandleTetrisError, TetrisError, TetrisErrorKind};
use self::timestep::interpolate;

pub mod highscores; // --- DONE (FINISHING TOUCHES MIGHT BE NEEDED)
pub mod timestep;
pub mod tests;
pub mod tetromino;
pub mod game_grid;
pub mod error;

//Breakdown of the game:
//The game should start by creating a grid of 12x24 squares. --- DONE
//Then it should look at position 0,0 and if it's a wall, if yes then it should render a grey square. --- DONE
//Then it should look at position 0,1 until it reaches 0,22 and render a grey square for each wall. --- DONE
//Then it should look at position 1,0 and if it's a wall, if yes then it should render a grey square, etc... --- DONE
//After the walls are rendered, the game can start. --- NEED TO LEARN HOW TO USE SDL2
//
//The game should start by creating a tetromino. --- DONE
//The tetromino with the shape and color should be chosen randomly. --- DONE
//it's x position should be random between 1 and 10. --- DONE
//it's y position should be 24. --- DONE 
//the two line at the top should not be rendered  --- NOT DONE / NEED TO LEARN HOW TO USE SDL2
//The tetromino should be rendered. --- NOT DONE / NEED TO LEARN HOW TO USE SDL2
//
//During an update, the game should check the values of each positions of the grid to determine what to render. --- SOME COULD BE DONE BUT NEED TO LEARN HOW TO USE SDL2
//
//The tetromino should move down by one square with the move_down method every 1 second. --- PARTIALY DONE
//The game should listen for the player's input. And call the move_left, move_right, rotate methods on input. --- PARTIALY DONE
//When the tetromino moves, the values of the grid should be updated(occupied or not). --- NOT DONE
//
//If the tetromino is not at the bottom of the grid or if it's not on top of another tetromino, it should keep moving. --- NOT DONE
//
//If the tetromino is at the bottom of the grid or if it's on top of another tetromino, it should stop moving. --- NOT DONE
//After the tetromino stops moving, the game should change the tetromino as placed. --- NOT DONE
//The game should then check if there is a line that is full of placed tetrominos. --- NOT DONE
//
//If there is a line that is full, the game should clear the line. Add to the score. --- NOT DONE
//If the score is equal to 4294967295, the game should stop. --- NOT DONE
//The lines over the cleared line should move down by one square. --- NOT DONE
//then the game should create a new tetromino. --- PARTIALY DONE
//
//If there is no line that is full, the game should create a new tetromino. --- PARTIALY DONE
//If the placed tetromino is at the top of the grid, the game should stop. --- NOT DONE
//
//The game should ask the player for their name. --- NOT DONE
//The game should save the score and the name of the player if it's in the top 10. And show the highscores. --- MOSTLY DONE BUT NEED TO LEARN HOW TO USE SDL2
//The game should ask the player if they want to play again. --- NOT DONE / NEED TO LEARN HOW TO USE SDL2
//If the player wants to play again, the game should start again. --- NOT DONE
//If the player doesn't want to play again, the game should go back to the main menu. --- NOT DONE / NEED TO LEARN HOW TO USE SDL2

//Note
//Data on each position of the grid should never leave memory. And sould be mutable. --- TO WATCH OUT FOR

static MAX_LEVEL: usize = 65;
static MAX_SCORE: u32 = 4294967295;


fn main() {
    
}

struct Level {
    level: usize,
    speed: f64,
}

pub fn get_speed(level: usize) -> f64 {
    if level > MAX_LEVEL {
        panic!("Level is too high");
    }
    let gravity_curve: Vec<f64> = vec!(0.01667f64, 0.021017f64, 0.026977f64, 0.035256f64, 0.04693f64, 0.06361f64, 0.0879f64, 0.1236f64, 0.1775f64, 0.259f64, 0.38f64, 0.56f64, 0.84f64, 1.3f64, 2.1f64, 3.5f64, 6.1f64, 11.1f64, 21.1f64, 41.1f64, 81.1f64, 161.1f64, 321.1f64, 641.1f64, 1281.1f64, 2561.1f64, 5121.1f64, 10241.1f64, 20481.1f64, 40961.1f64, 81921.1f64, 163841.1f64, 327681.1f64, 655361.1f64, 1310721.1f64, 2621441.1f64, 5242881.1f64, 10485761.1f64, 20971521.1f64, 41943041.1f64, 83886081.1f64, 167772161.1f64, 335544321.1f64, 671088641.1f64, 1342177281.1f64, 2684354561.1f64, 5368709121.1f64, 10737418241.1f64, 21474836481.1f64, 42949672961.1f64, 85899345921.1f64, 171798691841.1f64, 343597383681.1f64, 687194767361.1f64, 1374389534721.1f64, 2748779069441.1f64, 5497558138881.1f64, 10995116277761.1f64, 21990232555521.1f64, 43980465111041.1f64, 87960930222081.1f64, 175921860444161.1f64, 351843720888321.1f64, 703687441776641.1f64, 1407374883553281.1f64);
    let speed: f64 = (0.8 - ((0.007f64) * (gravity_curve[level-1]))).powf(gravity_curve[level-1]);
    return speed
}

impl Level {
    fn new(level: usize) -> Level {
        if level > MAX_LEVEL {
            panic!("Level is too high");
            //TODO: End the game there.
            //TODO: Add a custom error for this.
        }
        let speed: f64 = get_speed(level);
        Level {
            level: level,
            speed: speed,
        }
    }

    fn increase_level(&mut self) {
        self.level += 1;
    }
}

fn render(canvas: &mut WindowCanvas, color: Color, grid: Vec<Grid>, tetromino: Vec<Grid>, walls: Vec<Grid>) -> TetrisResult<()> {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();

    for grid in grid {
        canvas.set_draw_color(Color::RGB(grid.color.0, grid.color.1, grid.color.2));
        canvas.fill_rect(Rect::new(grid.x as i32 * 32, grid.y as i32 * 32, 32, 32))?;
    }

    for wall in walls {
        canvas.set_draw_color(Color::RGB(wall.color.0, wall.color.1, wall.color.2));
        canvas.fill_rect(Rect::new(wall.x as i32 * 32, wall.y as i32 * 32, 32, 32))?;
    }

    for tetromino in tetromino {
        canvas.set_draw_color(Color::RGB(tetromino.color.0, tetromino.color.1, tetromino.color.2));
        canvas.fill_rect(Rect::new(tetromino.x as i32 * 32, tetromino.y as i32 * 32, 32, 32))?;
    }
    Ok(())
}

fn video_subsystem_init(sdl_context: &Sdl) -> TetrisResult<VideoSubsystem> {
    let video_subsystem = match sdl_context.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(e) => return Err(Box::new(
            TetrisError::new(
                error::TetrisErrorKind::VideoSubsystem, 
                e,
            ))),
    };
    Ok(video_subsystem)
}

fn event_pump_init(sdl_context: &Sdl) -> TetrisResult<EventPump> {
    let event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(e) => return Err(Box::new(
            TetrisError::new(
                error::TetrisErrorKind::EventPump, 
                e,
            ))),
    };
    Ok(event_pump)
}

pub fn initialize() -> TetrisResult<()>{

    let mut score: u32 = 0;
    let mut highscores = highscores::load_highscores();
    
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = video_subsystem_init(&sdl_context)?;

    let window = video_subsystem.window("Tetris", 800, 600).position_centered().build().expect("Failed to build the window!");

    let mut canvas = window.into_canvas().build().expect("Failed to build the canvas!");

    let mut grid = create_grid();
    let mut walls = place_walls();
    grid.append(&mut walls);
    let mut tetromino = Tetromino::new();
    let mut drawn_tetromino = draw_tetromino(tetromino.clone());
    
    let mut left_side = false;
    let mut right_side = false;
    let mut under_side = false;

    for grids in grid.clone() {
        
        let grid = grids.get_from_position(tetromino.x - 1, 0);
        if grid.is_occupied() {
            left_side = true;
        }
        else if grid.is_wall() {
            left_side = true;
        }
        else {
            left_side = false;
        }
        let one = grids.get_from_position(tetromino.x + 2, 0); 
        if one.is_occupied() {
            right_side = true;
        }
        else if one.is_wall() {
            right_side = true;
        }
        else {
            if tetromino.shape.iter().find(|&i| i == &3) != None {
                let two = grids.get_from_position(tetromino.x + 3, 0);
                if two.is_occupied(){
                    right_side = true;
                }
                else if two.is_wall() {
                    right_side = true;
                }
                else {
                    right_side = false;
                }
            }
        }
    };

    let mut event_pump = event_pump_init(&sdl_context)?;
    let mut i = 0;

    //Frametime calculations
    let mut time : f64 = 0.0;
    let mut delta_time: f64 = 0.01;
    let quit = false;

    let mut current_time = SystemTime::now();
    let mut accumulator: f64 = 0.0;

    let mut previous_state = State::new();
    let mut current_state = State::new();


    'running: loop {
        
        
        //Handle events
        let new_time = SystemTime::now();
        let mut frame_time = new_time.duration_since(current_time)?.as_secs_f64();
        if frame_time > 0.25 {
            frame_time = 0.25;
        }
        current_time = new_time;

        accumulator += frame_time;

        while accumulator >= delta_time {
            previous_state = current_state;
            timestep::integrate(current_state, time, delta_time);
            time += delta_time;
            accumulator -= delta_time;
        }
        let alpha = accumulator / delta_time;
        
        interpolate(current_state, previous_state, alpha);
        
        let level = Level::new(1);
        
        let tetromino_velocity = level.speed * delta_time;

        if tetromino_velocity > 1.0 {
            tetromino.move_down();
        }

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    tetromino.move_left(left_side);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    tetromino.move_right(right_side);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    tetromino.rotate_left(left_side, right_side);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    tetromino.rotate_right(left_side, right_side);
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    todo!();
                    tetromino.move_down();
                },

                _ => {}
            }
        }
        
  
        //Update
        i = (i + 1) % 255;
        //Render
        render(&mut canvas, Color::RGB(i,64, 255 - i, ), grid.clone(), drawn_tetromino.clone(), walls.clone())?;
        
    }

    return Ok(())
  
}

fn create_grid() -> Vec<Grid> {
    let mut grid0: Vec<Grid> = Vec::new();
    for y in 0..25 {
        for x in 0..12 {
            let grid = Grid {
                color: (0,0,0),
                is_wall: false,
                occupied: false,
                placed: false,
                x,
                y,
            };
            grid0.push(grid);
        }
    }
    return grid0
}

fn place_walls() -> Vec<Grid> {
    let mut grid1: Vec<Grid> = Vec::new();
    for y in 0..25 {
        let grid = Grid {
            color: (120,120,120),
            is_wall: true,
            occupied: false,
            placed: false,
            x: 0,
            y,
        };
        grid1.push(grid);
    }
    
    let mut grid2: Vec<Grid> = Vec::new();
    for x in 0..12 {
        let grid = Grid {
            color: (120,120,120),
            is_wall: true,
            occupied: false,
            placed: false,
            x,
            y: 0,
        };
        grid2.push(grid);
    }
    
    let mut grid3: Vec<Grid> = Vec::new();
    for y in 0..25 {
        let grid = Grid {
            color: (120,120,120),
            is_wall: false,
            occupied: false,
            placed: false,
            x: 12,
            y,
        };
        grid3.push(grid);
    }

    grid1.append(&mut grid2);
    grid1.append(&mut grid3);
    return grid1
    
}

//FIXME: The shape of the tetromino affects the possible positions of the tetromino. To avoid bugs, the shape should be taken into account when generating the random position of the tetromino.

fn place_tetromino() -> Vec<Grid> {
    let mut grid0: Vec<Grid> = Vec::new();
    let tetromino = tetromino::Tetromino::new();
    let shape = tetromino.shape;
    let color = tetromino.color;
    let x = rand::thread_rng().gen_range(1..10);
    let y = 24;
    for y in 0..4 {
        for x in 0..4 {
            let grid = Grid {
                color,
                is_wall: false,
                occupied: false,
                placed: false,
                x,
                y,
            };
            grid0.push(grid);
        }
    }
    return grid0
}

fn draw_tetromino(tetromino: Tetromino) -> Vec<Grid> {
    let position = [tetromino.x, tetromino.y];
    let shape = tetromino.shape;
    let color = tetromino.color;
    let blank = (0,0,0);
    let mut grid = vec!(Grid::new());
    //To render a tetromino, we need to look at it's shape with these rules:
        //1. The value represents a line on the grid
        //1. If the value is positive, render a square
        //3. If the value is negative, render nothing and move to the next position on the line
        //4. If the value is the actual value + 1, change line.
        //5. The fist value is at the x and y position. The rest of the values are relative to the first value.

    let x = position[0];
    let y = position[1];

    for vector in shape {
        let mut new = Grid::new();
        let mut x = 0;
        if vector == 1 {
            new = Grid {
                color: color,
                is_wall: false,
                occupied: true,
                placed: false,
                x: x,
                y: y,

            };
            grid.push(new)
        }
        else if vector == -1 || x == 0 {
            new = Grid {
                color: blank,
                is_wall: false,
                occupied: false,
                placed: false,
                x: x,
                y: y - 1,
            };
            grid.push(new);
            x += 1
        }
        else if vector == -1 || x == 1 {
            new = Grid {
                color: blank,
                is_wall: false,
                occupied: false,
                placed: false,
                x: x,
                y: y - 2,
            };
            grid.push(new);
            x = 0
        }
        if vector == 2 {
            new = Grid {
                color: color,
                is_wall: false,
                occupied: true,
                placed: false,
                x: x + 1,
                y: y,

            };
            grid.push(new)
        }
        else if vector == -2 || x == 0 {
            new = Grid {
                color: blank,
                is_wall: false,
                occupied: false,
                placed: false,
                x: x + 1,
                y: y - 1,
            };
            grid.push(new);
            x += 1
        }
        else if vector == -2 || x == 1 {
            new = Grid {
                color: blank,
                is_wall: false,
                occupied: false,
                placed: false,
                x: x + 1,
                y: y - 2,
            };
            grid.push(new);
            x = 0
        }
        if vector == 3 {
            new = Grid {
                color: color,
                is_wall: false,
                occupied: true,
                placed: false,
                x: x + 2,
                y: y,

            };
            grid.push(new)
        }
        else if vector == -3 || x == 0 {
            new = Grid {
                color: blank,
                is_wall: false,
                occupied: false,
                placed: false,
                x: x + 2,
                y: y - 1,
            };
            grid.push(new);
            x += 1
        }
        else if vector == -3 || x == 1 {
            new = Grid {
                color: blank,
                is_wall: false,
                occupied: false,
                placed: false,
                x: x + 2,
                y: y - 2,
            };
            grid.push(new);
            x = 0
        }
    }
    return grid
    
}