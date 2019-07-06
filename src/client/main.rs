extern crate sdl2;

#[path = "../protocol.rs"]          mod protocol;
#[path = "../game_objects.rs"]      mod game_objects;
#[path = "../number_helpers.rs"]    mod number_helpers;
#[path = "../constants.rs"]         mod constants;
mod client;
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use std::{env, thread, time};
use sdl2::{pixels::Color, EventPump, render::WindowCanvas, ttf};

const FONT_PATH: &str = "retro_gaming.ttf";



fn initialize<'a ,'b>() -> (sdl2::render::WindowCanvas, EventPump, ttf::Sdl2TtfContext) {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).expect("error initializing sdl::ttf");
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("pong-game", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

        (window.into_canvas().build().unwrap(), sdl_context.event_pump().unwrap(), ttf_context)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        panic!("NO SERVER ADDRESS SUPPLIED");
    }
    println!("bin location: {}", args[0]);
    let remoteAddress = &args[args.len()-1];
    /*
    println!("starting server");
    let server_thread = thread::spawn(move || {    
        	server::start();
    });
    */
    println!("starting client");
    let mut client = client::Client::new(remoteAddress);
    
    let (mut renderer, events, ttf_context) = initialize();
    let mut local_player = game_objects::LocalPlayer::new(events);
    let mut remote_player = game_objects::RemotePlayer::new();
    let mut ball = game_objects::Ball::new();
    let texture_creator = renderer.texture_creator();
    let font = ttf_context.load_font(FONT_PATH, 128).unwrap();
    let target1 = sdl2::rect::Rect::new(100, 0, 30, 50);
    let target2 = sdl2::rect::Rect::new(constants::WINDOW_WIDTH as i32-135, 0, 30, 50);
        

    'running: loop {
        let player_movement = local_player.handle_input();
        let new_game_state = client.send_message_i32(player_movement);
        
        clear(&mut renderer);
        let score1_surf = font.render(&new_game_state.scores[0].to_string())
            .blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string()).expect("could not render font");
        let score2_surf = font.render(&new_game_state.scores[1].to_string())
            .blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string()).expect("could not render font");
        let texture1 = texture_creator.create_texture_from_surface(&score1_surf)
            .map_err(|e| e.to_string()).expect("could not create texture");
        let texture2 = texture_creator.create_texture_from_surface(&score2_surf)
            .map_err(|e| e.to_string()).expect("could not create texture");
    
        renderer.set_draw_color(Color::RGBA(195, 217, 255, 255));
        renderer.copy(&texture1, None, Some(target1)).expect("could not copy tecture to target");
        renderer.copy(&texture2, None, Some(target2)).expect("could not copy tecture to target");
        
        local_player.player.set_x_position(new_game_state.player_x_positions[0]);
        local_player.player.set_y_position(new_game_state.player_y_positions[0]);
        remote_player.player.set_x_position(new_game_state.player_x_positions[1]);
        remote_player.player.set_y_position(new_game_state.player_y_positions[1]);
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
