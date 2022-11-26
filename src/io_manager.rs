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

        for y in 0..DISPLAY.1 {

            for x in 0..DISPLAY.0 {

                if self.vga[(x + y * DISPLAY.0)] {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                }
            }

        }

        canvas.present();
    }

    pub fn display(&mut self, bytes: [u8; 2], canvas: &mut Canvas<Window>)
    {

        let vx = bytes[0] & 0x0f;
        let vy = (bytes[1] & 0xf0) >> 4; // right shift
        let n = bytes[1] & 0x0f;

        let x = vx % (DISPLAY.0 as u8);
        let y = vy % (DISPLAY.1 as u8);

        self.variables_register[0xf] = 0; // on reset le flag

        // 0x050 = adresse de départ des sprites
        let pixel = self.ram[self.index_register as usize]; // informations concernant les pixels

        for p_h in 0..n { // sprite de n pixel lignes 
            for p_w in 0..8 { // chaque colonne fait 8 pixel

                let bit = pixel & ((2 as u32).pow(p_w as u32 +1) as u8);
                
                // positions absolue
                let px = x + p_w; 
                let py = y + p_h;

                let vga_pos = (px as u16 + py as u16 * DISPLAY.0 as u16) as usize;


                // on check chaque bit de l'octet
                if bit.count_ones() == 1 && self.vga[ vga_pos ]
                {
                    // si les deux sont allumées, on éteint le pixel et on met le flag à true
                    self.vga[ vga_pos ] = false;
                    self.variables_register[0xf] = 1; // flag register
                }
                // si l'écran est off mais le pixel du sprit est on, on déssine le pixel
                else if bit.count_ones() == 1 && !self.vga[ vga_pos ]
                {
                    self.vga[ vga_pos ] = true;

                    // on dessine sur la fenêtre
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.draw_point(Point::new((px) as i32, (py) as i32)).unwrap();
                }

            }

        }
        
        
        
        canvas.present();

    }

    pub fn set_index_register(&mut self, bytes: [u8; 2])
    {

        let first_n = (bytes[0] & 0x0f) << 4; // right shift

        self.index_register = first_n as u16 & bytes[1] as u16;
        self.index_register %= 4096;
    }
    
    pub fn set_value_register_vx(&mut self, x: u8, nn: u8)
    {
        self.variables_register[x as usize] = nn;
    }

    pub fn add_value_register_vx(&mut self, x: u8, nn: u8)
    {
        if (self.variables_register[x as usize] + nn) < 0xff
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

        let first_n = (bytes[0] & 0x0f) << 4; // right shift

        self.set_pc(first_n as u16 & bytes[1] as u16);
    }

    pub fn inc_pc(&mut self)
    {
        self.program_counter += 2;
        self.program_counter %= 4096;
    }

    pub fn set_pc(&mut self, pos: u16)
    {
        self.program_counter = pos;
        self.program_counter %= 4096;
    }

    pub fn get_pc(&self) -> u16
    {
        return self.program_counter;
    }

    pub fn load_into_ram(&mut self, bytes: [u8; 1], i: u16)
    {
        self.ram[i as usize +1] = bytes[0];
    }

    pub fn get_ram_array(&mut self)
    {
        println!("{:?}", self.ram);
    }
}