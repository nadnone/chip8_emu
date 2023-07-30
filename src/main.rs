use std::env;

use constants::DISPLAY;
use sdl2; 
use sdl2::pixels::Color;

mod cpu_main;
mod constants;
mod io_manager;
mod misc_cpu;
mod inputs;

pub fn main() {


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("chip8 emulator", DISPLAY.0 as u32 * 10, DISPLAY.1 as u32* 10)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.set_logical_size(DISPLAY.0 as u32, DISPLAY.1 as u32).unwrap();
    canvas.clear();
    canvas.present();


    let mut event_pump = sdl_context.event_pump().unwrap();
   
    cpu_main::cpu_main(&mut canvas, env::args().collect(), &mut event_pump);
}