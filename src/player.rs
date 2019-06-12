use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use sdl2::render::WindowCanvas;

#[derive(Copy, Clone)]
pub struct PlayerBase {
    score: i32,
    position_y: f32,
    position_x: f32,
}

impl PlayerBase {
    pub fn new() -> PlayerBase {
        PlayerBase {
            score: 0,
            position_x: 10.0,
            position_y: 100.0,
        }
    }
    fn translate(&mut self, distance: f32) {
        self.position_y += distance;
    }
    fn score(&mut self) {
        self.score += 1;
    }
    
    pub fn draw(self, renderer: &mut WindowCanvas) {
    renderer.set_draw_color(Color::RGB(100, 100, 100));
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
    pub fn handle_input(&mut self) {
        self.player.translate(self.input_handler.handle_input());
    }
}

struct RemotePlayer {
    player: PlayerBase,
}

impl RemotePlayer {}

struct InputHandler {
    ev_pump: sdl2::EventPump,
}

impl InputHandler {
    fn new(ev_pump: sdl2::EventPump) -> InputHandler {
        InputHandler { ev_pump: ev_pump }
    }

    fn handle_input(&mut self) -> f32 {
        let increment: f32 = 3.0;
        let mut speed = 0.0;
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
                    _ => 0.0
                }
            }
            return speed;       
    }

}