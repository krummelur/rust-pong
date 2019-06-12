extern crate sdl2;
mod networkGame;
mod player;
mod server;
mod client;
use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::render::WindowCanvas;

fn initialize() -> (sdl2::render::WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("pong-game", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

        return (window.into_canvas().build().unwrap(), sdl_context.event_pump().unwrap())
}

fn main() {
    println!("starting server");
    let server_thread = thread::spawn(move || {    
        	server::start();
    });
    println!("starting client");
    client::start();
    println!("done");
    


    let (mut renderer, events) = initialize(); 
    let mut local_player = player::LocalPlayer::new(events);
    
    'running: loop {
        clear(&mut renderer);
        local_player.player.draw(&mut renderer);
        local_player.handle_input();
        thread::sleep(time::Duration::from_millis(10));
        renderer.present();
    }
}

fn clear(renderer: &mut WindowCanvas) {
    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
}
