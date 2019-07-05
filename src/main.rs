extern crate sdl2;
mod protocol;
mod game_objects;
mod server;
mod client;
mod number_helpers;
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
    let mut client = client::Client::new();
    println!("done");
    


    let (mut renderer, events) = initialize(); 
    let mut local_player = game_objects::LocalPlayer::new(events);
    let mut remote_player = game_objects::RemotePlayer::new();
    let mut ball = game_objects::Ball::new();
    
    'running: loop {
        clear(&mut renderer);
        let player_movement = local_player.handle_input();
        let new_game_state = client.send_message_i32(player_movement);
        //println!("===== PLAYER POS: {:?}, {:?}, {:?}======", new_game_state.player_x_positions, new_game_state.player_y_positions, new_game_state.ball_position);
        local_player.player.set_y_position(new_game_state.player_y_positions[0]);
        remote_player.player.set_y_position(new_game_state.player_y_positions[1]);
        remote_player.player.set_x_position(new_game_state.player_x_positions[1]);
        ball.set_position(new_game_state.ball_position);
        local_player.player.draw(&mut renderer);
        remote_player.player.draw(&mut renderer);
        ball.draw(&mut renderer);
        thread::sleep(time::Duration::from_millis(10));
        renderer.present();
    }
}

fn clear(renderer: &mut WindowCanvas) {
    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
}
