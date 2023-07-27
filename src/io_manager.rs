use sdl2::{render::Canvas, video::Window};
use sdl2::pixels::Color;
use sdl2::rect::*;

use crate::constants::*;
use crate::misc_cpu::MiscCPU;

pub struct IOManager {
    ram: [u8; RAM_SIZE_BYTES],
    vga: [u8; DISPLAY.0 * DISPLAY.1],
    delay_timer: u8,
    sound_timer: u8,

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
            delay_timer: 0,
            sound_timer: 0
        }

    }

    pub fn clear_screen(&self, canvas: &mut Canvas<Window>)
    {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

    }

    pub fn display(&mut self, bytes: [u8; 2], canvas: &mut Canvas<Window>, cpu_manager: &mut MiscCPU)
    {

        // les indexes de registre
        let vx_i = bytes[0] & 0x0f;
        let vy_i = (bytes[1] & 0xf0) >> 4; // right shift

        // le nombre de sprites
        let n = bytes[1] & 0x0f;

        // les positions
        let vx = cpu_manager.get_values_register(vx_i as usize) as u16 % DISPLAY.0 as u16;
        let vy = cpu_manager.get_values_register(vy_i as usize) as u16 % DISPLAY.1 as u16;

        // flag
        cpu_manager.set_values_register(0xf, 0); // on reset

        

        for p_h in 0..n as u16 { // ligne de la sprite
            
            for p_w in 0..8 as u16 { // chaque colonne fait 8 pixel

                let pixel = self.ram[cpu_manager.get_index_register() as usize + p_h as usize]; // informations concernant les pixels

                let mut bit = pixel & (1 << p_w); // masque
                bit = bit & (1 << p_w); // on reverse et on prend le n bit
                bit >>= p_w; // on décale à droite pour prendre le bit le plus haut


                // positions absolue
                let x: u16 = vx + 8 - p_w; // on inverse l'ordre de lecture avec 8 - p_w
                let y: u16 = vy + p_h;


                // limites d'affichage
                if x >= DISPLAY.0 as u16
                {
                    break; // on arrête la ligne
                }
                if y >= DISPLAY.1 as u16
                {
                    return; // on arrête le déssin
                }

              
                // position dans l'écran
                let vga_pos = (x + y * DISPLAY.0 as u16) as usize;


                // on check
                if bit == 1 && self.vga[ vga_pos ] == 1 // collision de pixels; on les éteints
                {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();

                    // on met le flag à 1
                    cpu_manager.set_values_register(0xf, 1); // flag register. à 1 quand collision

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

    pub fn get_from_ram(&self, i: u16) -> [u8; 2]
    {
        return [self.ram[i as usize], self.ram[i as usize + 1]];
    }

    /// word : data | i : index
    pub fn put_into_ram(&mut self, word: [u8; 2], i: u16)
    {
        self.ram[i as usize] = word[0];
        self.ram[i as usize + 1] = word[1];
    }


    pub fn inst_fx_xx(&mut self, cpu_manager: &mut MiscCPU, bytes: [u8; 2])
    {
        let x: u8 = bytes[0] & 0x0f;

        match bytes[1] {
            
            0x07 => cpu_manager.add_value_register_vx_7xnn(x, self.delay_timer),

            0x15 => self.delay_timer = cpu_manager.get_values_register(x as usize),

            0x18 => self.sound_timer = cpu_manager.get_values_register(x as usize),

            0x29 => {
                let vx = cpu_manager.get_values_register(x as usize);

                let ram_data = self.get_from_ram(vx as u16);
                
                cpu_manager.set_index_register( (ram_data[0] | ram_data[1]) as u16 );
            }

            0x33 => {

                let vx = cpu_manager.get_values_register(x as usize);

                let secnds = vx % 100;

                let last = vx % 10;
                let first = (vx - secnds) / 100;
                let middle = (secnds - last) / 10;

                let i = cpu_manager.get_index_register();

                self.ram[i as usize + 0] = first;
                self.ram[i as usize + 1] = middle;
                self.ram[i as usize + 2] = last;

            }

            0x55 => { // store V0 to VX in ram at I starting point

                let i = cpu_manager.get_index_register() as usize;

                for v_i in 0..(x+1) as usize {
 
                    self.ram[i + v_i] = cpu_manager.get_values_register(v_i);
                }

            }

            0x65 => {

                let i = cpu_manager.get_index_register() as usize;

                for v_i in 0..(x+1) as usize {
 
                    cpu_manager.set_values_register(v_i, self.ram[i + v_i]);
                }

            }


           
            _ => println!("[!] exception sub-opcode (io_manager.rs):  {:x}", bytes[1])
        }

   
    }


}