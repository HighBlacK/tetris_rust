// Grid size is 12x22 (12x20 with 2 hidden rows)
// line 0 and 21 and column 0 and 11 are walls 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Grid {
    pub color: (u8, u8, u8),
    pub is_wall: bool,
    pub occupied: bool,
    pub placed: bool,
    pub x: u8,
    pub y: u8,
}

impl Grid {

    pub fn new() -> Grid {
        Grid {
            color: (0, 0, 0),
            is_wall: false,
            occupied: false,
            placed: false,
            x: 0,
            y: 0,
        }
    }

    pub  fn set_color(&mut self, color: (u8, u8, u8)) -> Grid{
        self.color = color;
        *self
    }

    pub fn get_from_position(&self, x: u8, y: u8) -> Grid{
        *self
    }
    
    pub fn is_wall(&self) -> bool{
        self.is_wall
    }
    
    pub fn is_occupied(&self) -> bool{
        self.occupied
    }
    
    fn set_occupied(&mut self) -> Grid{
        self.occupied = true;
        *self
    }

    fn set_unoccupied(&mut self) -> Grid{
        self.occupied = false;
        *self
    }

    fn get_x(&self) -> u8{
        self.x
    }

    fn get_y(&self) -> u8{
        self.y
    }

    //FIXME: This function doesn't make sense
    fn clear_line( &mut self, line: u8) -> Grid{
        for i in 0..12 {
            self.set_unoccupied();
        }
        *self
    }

    fn clear_grid(){



    }

    fn check_for_lines(){


    }

}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

fn render_grid(){

    

}
