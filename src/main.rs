extern crate sdl2;

use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("pong-game", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.into_canvas().build().unwrap();
    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.fill_rect(Rect::new(10, 10, 100, 100)).ok();
    renderer.present();

    let events = sdl_context.event_pump().unwrap();

    let mut local_player = LocalPlayer::new(events);
    local_player.start();

    thread::sleep(time::Duration::from_millis(2000));
}

struct InputHandler {
    ev_pump: sdl2::EventPump,
}

impl InputHandler {
    fn new(ev_pump: sdl2::EventPump) -> InputHandler {
        InputHandler { ev_pump: ev_pump }
    }

    ///starts the player
    fn start(&mut self) {
        'running: loop {
            for event in self.ev_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'running;
                };
            }
            let keys: HashSet<sdl2::keyboard::Keycode> = self
                .ev_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            if !keys.is_empty() {
                println!("keys pressed: {:?}", keys);
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

}

struct PlayerBase {
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
        println!("playter is moving in direction {}", distance);
        self.position_y += distance;
    }
    fn score(&mut self) {
        self.score += 1;
    }
}

struct LocalPlayer {
    input_handler: InputHandler,
    player: PlayerBase,
}

impl LocalPlayer {
    fn new(ev_pump: sdl2::EventPump) -> LocalPlayer {
        LocalPlayer {
            input_handler: InputHandler::new(ev_pump),
            player: PlayerBase::new(),
        }
    }
    fn start(&mut self) {
        self.input_handler.start();
    }
    fn handle_input(&mut self) {
        self.player.translate(1.0);
    }
}

struct RemotePlayer {
    player: PlayerBase,
}

impl RemotePlayer {}
