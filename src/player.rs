use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use sdl2::render::WindowCanvas;

#[derive(Copy, Clone)]
pub struct PlayerBase {
    score: i32,
    position_y: i32,
    position_x: i32,
}

impl PlayerBase {
    pub fn new() -> PlayerBase {
        PlayerBase {
            score: 0,
            position_x: 10,
            position_y: 100,
        }
    }
    pub fn set_y_position(&mut self, new_position: i32) {
        self.position_y = new_position;
    } 
    
    pub fn set_x_position(&mut self, new_position: i32) {
        self.position_x = new_position;
    } 

    pub fn translate(&mut self, distance: i32) {
        self.position_y += distance;
    }
    pub fn score(&mut self) {
        self.score += 1;
    }
    
    pub fn draw(self, renderer: &mut WindowCanvas) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.fill_rect(Rect::new(self.position_x as i32, self.position_y as i32, 10, 100)).expect("Could not draw player");
    }
}


pub struct LocalPlayer {
    input_handler: InputHandler,
    pub player: PlayerBase,
}

impl LocalPlayer {
    pub fn new(ev_pump: sdl2::EventPump) -> LocalPlayer {
        LocalPlayer {
            input_handler: InputHandler::new(ev_pump),
            player: PlayerBase::new(),
        }
    }

    pub fn handle_input(&mut self) -> i32 {
        self.input_handler.handle_input()
    }
}

pub struct RemotePlayer {
    pub player: PlayerBase,
}

impl RemotePlayer {
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