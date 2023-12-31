use std::vec;
use rand::Rng;
use super::game_grid::Grid;

static DEBUG: bool = true;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tetromino {
    pub shape: Vec<i8>,
    pub color: (u8, u8, u8),
    pub x: u8,
    pub y: u8,
}

impl Tetromino {
    
    pub fn new() -> Tetromino {
        
        //To render a tetromino, we need to look at it's shape with these rules:
        //1. The value represents a line on the grid
        //1. If the value is positive, render a square
        //3. If the value is negative, render nothing and move to the next position on the line
        //4. If the value is the actual value + 1, change line.
        //5. The fist value is at the x and y position. The rest of the values are relative to the first value.

        let straight = vec![1,2,3,4];
        let square = vec![1,1,2,2];
        let t = vec![1,2,2,3];
        let l = vec![1,1,1,-2,-2,2];
        let skew = vec![-1,1,2,2,3];

        let cyan = (0,255,255);
        let yellow = (255,255,0);
        let purple = (255,0,255);
        let green = (0,255,0);
        let red = (255,0,0);
        let orange = (255,165,0);

        let y = 24;
        let x = rand::thread_rng().gen_range(1..10);

        let mut rng = rand::thread_rng();
        
        let shape = match rng.gen_range(1..6) {
            1 => straight,
            2 => square,
            3 => t,
            4 => l,
            5 => skew,
            _ => {
                println!("Error: Shape not found");
                vec![]
            }
        };

        let color = match rng.gen_range(1..6) {
            1 => cyan,
            2 => yellow,
            3 => purple,
            4 => green,
            5 => red,
            6 => orange,
            _ => {
                println!("Error: Color not found");
                (0, 0, 0)
            }
        };

        Tetromino {
            shape,
            color,
            x,
            y,
        }

    }

    pub fn rotate_left(&mut self, left_grid: bool, right_grid: bool) {
        
        if left_grid {
            self.x += 1;
        }
        else if self.shape.iter().find(|&i| i == &3) != None && left_grid {
            self.x += 1; 
        }
        else if right_grid {
            self.x -= 1;
        }
        else if self.shape.iter().find(|&i| i == &3) != None && right_grid {
            self.x -= 1; 
        }

        //Straight
        if self.shape == vec![1,2,3,4] {
            self.shape = vec![1,1,1,1];
        } else if self.shape == vec![1,1,1,1] {
            self.shape = vec![1,2,3,4];
        } 
        //Square
        else if self.shape == vec![1,1,2,2] {
            self.shape = vec![1,1,2,2];
        }         
        //T
        else if self.shape == vec![1,2,2,3] {
            self.shape = vec![1,1,1,-2,2];
        } else if self.shape == vec![1,1,1,-2,2] {
            self.shape = vec![-1,1,2,2,-3,3];
        } else if self.shape == vec![-1,1,2,2,-3,3] {
            self.shape = vec![-1,1,2,2,2];
        } else if self.shape == vec![-1,1,2,2,2] {
            self.shape = vec![1,2,2,3];
        } 
        //L
        else if self.shape == vec![1,1,1,-2,-2,2] {
            self.shape = vec![-1,1,-2,2,3,3];
        } else if self.shape == vec![-1,1,-2,2,3,3] {
            self.shape = vec![1,2,2,2];
        } else if self.shape == vec![1,2,2,2] {
            self.shape = vec![1,1,2,3];
        } else if self.shape == vec![1,1,2,3] {
            self.shape = vec![1,1,1,-2,-2,2];
        }
        //Skew
        else if self.shape == vec![-1,1,2,2,3] {
            self.shape = vec![1,1,-2,2,2];
        } else if self.shape == vec![1,1,-2,2,2] {
            self.shape = vec![-1,1,2,2,3];
        } else {
            println!("Error: Shape not found");
            self.shape = vec![];
        }
    }

    pub fn rotate_right(&mut self, left_grid: bool, right_grid: bool) {

        if left_grid {
            self.x += 1;
        }
        else if self.shape.iter().find(|&i| i == &3) != None && left_grid {
            self.x += 1; 
        }
        else if right_grid {
            self.x -= 1;
        }
        else if self.shape.iter().find(|&i| i == &3) != None && right_grid {
            self.x -= 1; 
        }

        //Straight
        if self.shape == vec![1,2,3,4] {
            self.shape = vec![1,1,1,1];
        } else if self.shape == vec![1,1,1,1] {
            self.shape = vec![1,2,3,4];
        }
        //Square
        else if self.shape == vec![1,1,2,2] {
            self.shape = vec![1,1,2,2];
        }
        //T
        else if self.shape == vec![1,2,2,3] {
            self.shape = vec![-1,1,2,2,2];
        } else if self.shape == vec![-1,1,2,2,2] {
            self.shape = vec![-1,1,2,2,-3,3]; 
        } else if self.shape == vec![-1,1,2,2,-3,3] { 
            self.shape = vec![1,1,1,-2,2]; 
        } else if self.shape == vec![1,1,1,-2,2] { 
            self.shape = vec![1,2,2,3]; 
        }
        //L
        else if self.shape == vec![1,1,1,-2,-2,2] { 
            self.shape = vec![1,1,2,3]; 
        } else if self.shape == vec![1,1,2,3] { 
            self.shape = vec![1,2,2,2]; 
        } else if self.shape == vec![1,2,2,2] { 
            self.shape = vec![-1,1,-2,2,3,3]; 
        } else if self.shape == vec![-1,1,-2,2,3,3] { 
            self.shape = vec![1,1,1,-2,-2,2];
        }
         //Skew
         else if self.shape == vec![-1,1,2,2,3] {
            self.shape = vec![1,1,-2,2,2];
        } else if self.shape == vec![1,1,-2,2,2] {
            self.shape = vec![-1,1,2,2,3];
        } else {
            println!("Error: Shape not found");
            self.shape = vec![];
        }
        
    }

    //must call the function with the grid position on the left of the tetromino
    pub fn move_left(&mut self, left_grid: bool) {

        if self.x >= 1 && !left_grid {
            self.x -= 1;

            if DEBUG {
                println!("Moved left");
            }
        }
        else {
            self.x = self.x;
           
            if DEBUG {
                println!("Error: Cannot move left");
            }
        }

    }

    //must call the function with the grid position on the right of the tetromino
    pub fn move_right(&mut self, right_grid: bool) {
        
        if self.x <= 10 && !right_grid {
            self.x += 1;

            if DEBUG {
                println!("Moved right");
            }
        }
        else {
            self.x = self.x;
           
            if DEBUG {
                println!("Error: Cannot move right");
            }
        }
    }

    pub fn move_down(&mut self) {
        //implement collision detection
        if self.y >= 1 {
            self.y -= 1;

            if DEBUG {
                println!("Moved down");
            }
        }
        //add else if to check if the tetromino is placed
        //if it is placed, add it to the placed tetromino vector
        else {
            self.y = self.y;
           
            if DEBUG {
                println!("Error: Cannot move down");
            }
        }
    }

    pub fn get_shape(&mut self) -> String {
        let shape: &Vec<i8> = &self.shape;

        let mut shape_type = String::new();

        if *shape == vec![1,2,3,4] || *shape == vec![1,1,1,1] {
            shape_type = String::from("Straight");
        } else if *shape == vec![1,1,2,2] {
            shape_type = String::from("Square");
        } else if *shape == vec![1,2,2,3] || *shape == vec![1,1,1,-2,2] || *shape == vec![-1,1,2,2,-3,3] || *shape == vec![-1,1,2,2,2] {
            shape_type = String::from("T");
        } else if *shape == vec![1,1,1,-2,-2,2] || *shape == vec![-1,1,-2,2,3,3] || *shape == vec![1,2,2,2] || *shape == vec![1,1,2,3] {
            shape_type = String::from("L");
        } else if *shape == vec![-1,1,2,2,3] || *shape == vec![1,1,-2,2,2] {
            shape_type = String::from("Skew");
        } else {
            println!("Error: Shape not found");
        }

        if DEBUG {
            println!("Shape: {}", shape_type);
        }

        return shape_type
        
    }

    fn speed_up(&mut self) {
        todo!("Speed up")
    }

    fn speed_down(&mut self) {

    }

    fn to_placed(&mut self) {

    }
}

//if the tetromino is placed, it will be added to the placed tetromino vector
//After all lines have been checked to not be filled, if the placed tetromino y is 21, it will trigger a game over 
struct Placed_Tetromino {
    shape: Vec<i8>,
    color: (u8, u8, u8),
    x: u16,
    y: u16,
}


impl Placed_Tetromino {
    
    fn new() -> Placed_Tetromino {
        Placed_Tetromino {
            shape: vec![],
            color: (0, 0, 0),
            x: 0,
            y: 0,
        }
    }

    fn remove_line(&mut self) {

    }
    
    fn move_down(&mut self) {

    }

    fn get_x(&mut self) {

    }

    fn get_y(&mut self) {

    }

    fn get_size(&mut self) {

    }

}

fn place_tetromino(tetromino: Tetromino, grid: Grid) {

    let placed = Placed_Tetromino::new();
    let shape = tetromino.shape;
    let color = tetromino.color;
    let x = tetromino.x;
    let y = tetromino.y;


  
}