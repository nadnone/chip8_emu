use sdl2::{render::Canvas, video::Window};
use sdl2::pixels::Color;
use sdl2::rect::*;

use crate::constants::*;

pub struct IOManager {
    ram: [u8; RAM_SIZE_BYTES],
    vga: [u8; DISPLAY.0 * DISPLAY.1],
    program_counter: u16,
    index_register: u16,
    variables_register: [u8; REGISTER_SIZE],
}

impl IOManager {


    pub fn initialize() -> IOManager
    {

        // on charges les FONTS SPRITES 
        let mut ram_tmp = [0; RAM_SIZE_BYTES];
        for i in 0..80 
        {
            ram_tmp[i] = SPRITES_DATA[i];
        }


        return IOManager {
            ram: ram_tmp,
            vga: [0; DISPLAY.0 * DISPLAY.1],
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

        // les indexes de registre
        let vx_i = bytes[0] & 0x0f;
        let vy_i = (bytes[1] & 0xf0) >> 4; // right shift

        // le nombre de sprites
        let n = bytes[1] & 0x0f;

        // les positions
        let vx = self.variables_register[vx_i as usize] as u16 % DISPLAY.0 as u16;
        let vy = self.variables_register[vy_i as usize] as u16 % DISPLAY.1 as u16;

        // flag
        self.variables_register[0xf] = 0; // on reset

        

        for p_h in 0..n as u16 { // ligne de la sprite
            
            for p_w in 0..8 as u16 { // chaque colonne fait 8 pixel

                let pixel = self.ram[self.index_register as usize + p_h as usize]; // informations concernant les pixels

                let mut bit = pixel & (1 << p_w); // masque
                bit = bit & (1 << p_w); // on reverse et on prend le n bit
                bit >>= p_w; // on décale à droite pour prendre le bit le plus haut


                // positions absolue
                let x: u16 = vx + 8-p_w;
                let y: u16 = vy + p_h;


                // limites d'affichage
                if x >= DISPLAY.0 as u16
                {
                    break;
                }
                if y >= DISPLAY.1 as u16
                {
                    return;
                }

              
                // position dans l'écran
                let vga_pos = (x + y * DISPLAY.0 as u16) as usize;


                // on check
                if bit == 1 && self.vga[ vga_pos ] == 1 // collision de pixels; on les éteints
                {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();

                    // on met le flag à 1
                    self.variables_register[0xf] = 1; // flag register. à 1 quand collision

                    self.vga[ vga_pos ] = 0;
                }
                else if bit == 1// si bit == 1 , on dessine 
                {
                    // on dessine sur la fenêtre
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();

                    self.vga[ vga_pos ] = 1;
                }
                else if self.vga[ vga_pos ] == 1 // on éteint le pixel
                {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();

                    self.vga[ vga_pos ] = 0;
                }
                

            } 
            
        }


        
        
        
    }

    pub fn set_index_register(&mut self, bytes: [u8; 2])
    {
        let shifted = ((bytes[0] as u16) & 0x0f) << 8;

        self.index_register = shifted | bytes[1] as u16;
    }
    
    pub fn set_value_register_vx(&mut self, x: u8, nn: u8)
    {
        self.variables_register[x as usize] = nn;
    }

    pub fn add_value_register_vx(&mut self, x: u8, nn: u8)
    {
        let var = self.variables_register[x as usize];

        // on bloque au max d'un 8bit
        self.variables_register[x as usize] = (var as u16 + nn as u16) as u8 % 255;
     
    }

    pub fn jump(&mut self, bytes: [u8; 2])
    {
        let shifted = ((bytes[0] as u16) & 0x0f) << 8;

        self.set_pc(shifted | bytes[1] as u16);
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

    pub fn _get_ram_array(& self)
    {
        println!("{:?}", self.ram);
    }

    pub fn get_from_ram(&self, i: u16) -> [u8; 2]
    {
        return [self.ram[i as usize], self.ram[i as usize + 1]];
    }

    pub fn put_into_ram(&mut self, word: [u8; 2], i: u16)
    {
        self.ram[i as usize] = word[0];
        self.ram[i as usize + 1] = word[1];
    }

}