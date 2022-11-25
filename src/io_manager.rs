use sdl2::{render::Canvas, video::Window};
use sdl2::pixels::Color;
use sdl2::rect::*;

use crate::constants::*;

pub struct IOManager {
    ram: [u8; RAM_SIZE_BYTES],
    vga: [bool; DISPLAY.0 * DISPLAY.1],
    program_counter: u16,
    index_register: u16,
    stack: u16,
    delay_timer: u8,
    sound_timer: u8,
    variables_register: [u8; REGISTRER_SIZE],
    vf: bool // flag register

}

impl IOManager {


    pub fn initialize() -> IOManager
    {
        return IOManager {
            ram: [0; RAM_SIZE_BYTES],
            vga: [false; DISPLAY.0 * DISPLAY.1],
            program_counter: 0,
            index_register: 0,
            stack: 0,
            delay_timer: DELAY_TIMER_MAX_RATE,
            sound_timer: 0,
            variables_register: [0; REGISTRER_SIZE],
            vf: false
        }

    }

    pub fn clear_screen(&self, canvas: &mut Canvas<Window>)
    {

        for y in 0..DISPLAY.1 {

            for x in 0..DISPLAY.0 {

                if self.vga[(x + y * DISPLAY.0)] {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                }
            }

        }

        canvas.present();
        println!("clear screen");
    }

    pub fn display(&mut self, vx: u8, vy: u8, n: u8, canvas: &mut Canvas<Window>)
    {
        let x = vx % DISPLAY.0 as u8 - 1;
        let y = vy % DISPLAY.1 as u8 - 1;

        self.vf = false;
        
        // TODO comprendre le display du chip-8
        todo!();

    }

    pub fn set_index_register(&mut self, nnn: u16)
    {
        self.index_register = nnn;
    }
    
    pub fn set_value_registe_vx(&mut self, x: u8, nn: u8)
    {
        self.variables_register[x as usize] = nn;
    }

    pub fn add_value_register(&mut self, x: u8, nn: u8)
    {
        if self.variables_register[x as usize] < 0xf 
        {
            self.variables_register[x as usize] += nn;
        }
        else
        {
            self.variables_register[x as usize] = 0;
        }
    }

    pub fn jump(&mut self, byte: u16)
    {
        self.set_pc(byte);
        println!("jump {:x}", byte);
    }

    pub fn inc_pc(&mut self)
    {
        self.program_counter += 2;
    }

    pub fn set_pc(&mut self, pos: u16)
    {
        self.program_counter = pos;
    }

    pub fn get_pc(&self) -> u16
    {
        return self.program_counter;
    }

}