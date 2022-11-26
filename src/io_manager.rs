use sdl2::{render::Canvas, video::Window};
use sdl2::pixels::Color;
use sdl2::rect::*;

use crate::constants::*;

pub struct IOManager {
    ram: [u8; RAM_SIZE_BYTES],
    vga: [bool; DISPLAY.0 * DISPLAY.1],
    program_counter: u16,
    index_register: u16,
    variables_register: [u8; REGISTER_SIZE],
}

impl IOManager {


    pub fn initialize() -> IOManager
    {

        // on charges les FONTS SPRITES 
        let mut ram_tmp = [0; RAM_SIZE_BYTES];
        for i in 0x050..=0x09f 
        {
            ram_tmp[i] = SPRITES_DATA[i - 0x050];
        }


        return IOManager {
            ram: ram_tmp,
            vga: [false; DISPLAY.0 * DISPLAY.1],
            program_counter: PROGRAM_COUNTER_START_ADDR,
            index_register: 0,
            variables_register: [0; REGISTER_SIZE],
        }

    }

    pub fn clear_screen(&self, canvas: &mut Canvas<Window>)
    {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

    }

    pub fn display(&mut self, bytes: [u8; 2], canvas: &mut Canvas<Window>)
    {

        let vx = bytes[0] & 0x0f;
        let vy = (bytes[1] & 0xf0) >> 3; // right shift


        let n = bytes[1] & 0x0f;

        let x = vx % (DISPLAY.0 as u8);
        let y = vy % (DISPLAY.1 as u8);

        self.variables_register[0xf] = 0; // on reset le flag

        let pixel = self.ram[self.index_register as usize]; // informations concernant les pixels
        
        for p_h in 0..n { // sprite de n lignes 
            for p_w in 0..8 { // chaque colonne fait 8 pixel

                let mut bit = pixel & (2 as u32).pow(p_w as u32) as u8; // masque
                bit = bit >> p_w; // right shift

                // positions absolue
                let px = x + p_w; 
                let py = y + p_h;

                // position dans l'écran
                let vga_pos = (px as u16 + py as u16 * DISPLAY.0 as u16) as usize;


                // on check chaque bit de l'octet
                if bit == 1 && self.vga[ vga_pos ]
                {
                    // si les deux sont allumées, on éteint le pixel et on met le flag à 1
                    self.vga[ vga_pos ] = true;
                    self.variables_register[0xf] = 1; // flag register
                }
                // si l'écran est off mais le pixel du sprit est on, on déssine le pixel
                else if bit == 1 && !self.vga[ vga_pos ]
                {
                    self.vga[ vga_pos ] = true;

                    // on dessine sur la fenêtre
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.draw_point(Point::new((px) as i32, (py) as i32)).unwrap();
                }

            }

        }
        
        
        
    }

    pub fn set_index_register(&mut self, bytes: [u8; 2])
    {
        // Probablement à revoir

        let shifted = ((bytes[0] as u16) & 0x0f) << 7;

        self.index_register = shifted | bytes[1] as u16;
    }
    
    pub fn set_value_register_vx(&mut self, x: u8, nn: u8)
    {
        self.variables_register[x as usize] = nn;
    }

    pub fn add_value_register_vx(&mut self, x: u8, nn: u8)
    {
        let var = self.variables_register[x as usize];

        if (var as u16 + nn as u16) > 0xff
        {
            self.variables_register[x as usize] = nn -1;
        }
        else
        {
            self.variables_register[x as usize] += nn;
        }
     
    }

    pub fn jump(&mut self, bytes: [u8; 2])
    {
        // Probablement à revoir

        let shifted = ((bytes[0] as u16) & 0x0f) << 7;

        self.set_pc(shifted | bytes[1] as u16);
    }

    pub fn inc_pc(&mut self)
    {
        self.program_counter += 2;
        self.program_counter %= 4096;
    }

    pub fn set_pc(&mut self, pos: u16)
    {
        self.program_counter = pos;
    }

    pub fn get_pc(&self) -> u16
    {
        return self.program_counter;
    }

    pub fn _get_ram_array(&mut self)
    {
        println!("{:?}", self.ram);
    }

}