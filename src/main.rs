extern crate sdl2;

use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};

fn  main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.into_canvas().build().unwrap();
    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.fill_rect(Rect::new(10, 10, 100, 100));   
    renderer.present();
    
    thread::sleep(time::Duration::from_millis(2000));
}

#[test]
fn should_fail() {
    unimplemented!();
}