use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use sdl2::render::WindowCanvas;
use constants::{BALL_SIZE, PADDLE_HEIGHT, PADDLE_WIDTH};

/// Returns true if two objects are encroaching
///     
/// # Arguments
/// 
/// * `obj1_pos` - an array with x, y positions of the 1st object
/// * `obj2_pos` - an array with x, y positions of the 2nd object
/// * `obj1_dim` - an array with x, y dimensions of the 1st object
/// * `obj2_dim` - an array with x, y dimensions of the 2nd object
pub fn is_colliding(obj1_pos: [i32;2], obj2_pos: [i32;2], obj1_dim: [u32;2], obj2_dim: [u32;2]) -> bool {
    let distance_x = (obj1_pos[0] - obj2_pos[0]).abs();
    let mut leftmost_obj_dim = obj1_dim;
    if obj1_pos[0] > obj2_pos[0] {
        leftmost_obj_dim = obj2_dim;
    }

    if distance_x < leftmost_obj_dim[0] as i32 {
        let distance_y = (obj1_pos[1] - obj2_pos[1]).abs();
        let mut uppermost_obj_dim = obj1_dim;
        if obj1_pos[1] > obj2_pos[1] {
            uppermost_obj_dim = obj2_dim;
        }
        if distance_y < uppermost_obj_dim[1] as i32 {
            println!("leftmost: {:?}", leftmost_obj_dim);
            println!("Distance_x: {}", distance_x);
            return true;
        }
    }
    false
}

#[derive(Copy, Clone)]
pub struct PlayerBase {
    score: i32,
    position_y: i32,
    position_x: i32,
}

impl PlayerBase {
    /// Returns a new PlayerBase instance
    pub fn new() -> PlayerBase {
        PlayerBase {
            score: 0,
            position_x: 10,
            position_y: 100,
        }
    }

    /// Sets a new position_y
    pub fn set_y_position(&mut self, new_position: i32) {
        self.position_y = new_position;
    } 
    
    /// Sets a new position_x
    pub fn set_x_position(&mut self, new_position: i32) {
        self.position_x = new_position;
    } 

    /// Sets a new position_y from a desired move distance
    pub fn translate(&mut self, distance: i32) {
        self.position_y += distance;
    }

    /// Increments the players score
    pub fn score(&mut self) {
        self.score += 1;
    }
    
    /// Draws the player onto the canvas
    /// 
    /// # Arguments
    /// 
    /// * `renderer` - a mutable reference to the canvas to draw onto
    pub fn draw(self, renderer: &mut WindowCanvas) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.fill_rect(Rect::new(self.position_x as i32, self.position_y as i32, PADDLE_WIDTH, PADDLE_HEIGHT)).expect("Could not draw player");
    }
}

#[derive(Copy, Clone)]
pub struct Ball {
    pub position: [i32; 2]
}

impl Ball {
    /// Returns a new Ball instance
    pub fn new() -> Ball {
        Ball {
            position: [200,200]
        }
    }

    /// Sets a new position
    ///  
    /// # Arguments
    /// 
    /// * `new_position` - an array of 2 32-bit integers representing the new x, and y-positions of the ball
    pub fn set_position(&mut self, new_position: [i32;2]) {
        self.position = new_position;
    } 
    /// Draws the ball onto the canvas
    /// 
    /// # Arguments
    /// 
    /// * `renderer` - a mutable reference to the canvas to draw onto
    pub fn draw(self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.fill_rect(Rect::new(self.position[0], self.position[1], BALL_SIZE, BALL_SIZE)).expect("Could not draw player");
    }
}

pub struct LocalPlayer {
    input_handler: InputHandler,
    pub player: PlayerBase,
}

impl LocalPlayer {
    /// Returns a new LocalPlayer reference
    pub fn new(ev_pump: sdl2::EventPump) -> LocalPlayer {
        LocalPlayer {
            input_handler: InputHandler::new(ev_pump),
            player: PlayerBase::new(),
        }
    }

    /// Checks the keyboard input and updates the players position accordingly
    pub fn handle_input(&mut self) -> i32 {
        self.input_handler.handle_input()
    }
}

pub struct RemotePlayer {
    pub player: PlayerBase,
}

impl RemotePlayer {
    /// Returns a new RemotePlayer instance
    pub fn new() -> RemotePlayer {
        RemotePlayer {
            player: PlayerBase::new(),
        }
    }
}

struct InputHandler {
    ev_pump: sdl2::EventPump,
}

impl InputHandler {
    fn new(ev_pump: sdl2::EventPump) -> InputHandler {
        InputHandler { ev_pump: ev_pump }
    }

    fn handle_input(&mut self) -> i32 {
        let increment: i32 = 3;
        let mut speed = 0;
            for event in self.ev_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                std::process::exit(0)}
            };

            let keys: HashSet<sdl2::keyboard::Keycode> = self
                .ev_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            for keycode in &keys {
                speed += match keycode {
                    Keycode::W  => -increment,
                    Keycode::S => increment,
                    Keycode::Up  => -increment,
                    Keycode::Down  => increment,
                    _ => 0
                }
            }
            return speed;       
    }

}