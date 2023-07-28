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

    pub fn _get_ram_array(&self, x: usize, y: usize)
    {
        let mut range = vec![];
        for i in x..y+1 {
            range.push(self.ram[i]);
        }
        
        println!("RAM range data: {:?}", range);
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
        let vx = cpu_manager.get_values_register(vx_i as usize) % DISPLAY.0 as u8;
        let vy = cpu_manager.get_values_register(vy_i as usize) % DISPLAY.1 as u8;

        // flag
        cpu_manager.set_values_register(0xf, 0); // on reset

        

        for p_h in 0..n as u8 { // ligne de la sprite
            
            let mut sprite = self.ram[cpu_manager.get_index_register() as usize + p_h as usize]; // informations concernant les pixels

            for p_w in 0..8 as u8 { // chaque colonne fait 8 pixel

                if (sprite & 0x80) > 0 // si le premier bit à gauche = 1, on dessine ou écrase
                {

                    // valeurs absolues
                    let mut x = vx + p_w;
                    let mut y = vy + p_h;
    
                    // limite d'écran
                    if x > DISPLAY.0 as u8
                    {
                        x -= DISPLAY.0 as u8;
                    }
                    if y > DISPLAY.1 as u8
                    {
                        y -= DISPLAY.1 as u8;
                    }
    
    
                    let pixel_loc = (x as u16 + (y as u16 * DISPLAY.1 as u16)) as usize;
    
                    // si vga = 1 OR sprite = 1 alors on allume, sinon on éteint
                    self.vga[ pixel_loc ] |= 1; // OR


                    if self.vga[pixel_loc] == 1 // si pixel allumé
                    {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                        canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                    }
                    else // sinon forcément éteint
                    {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                        canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();

                        cpu_manager.set_values_register(0xf, 1);

                    }

                }



                sprite <<= 1; // on décale à gauche d'un bit (left shift)


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
        let x = bytes[0] & 0x0f;

        match bytes[1] {
            
            0x07 => cpu_manager.set_values_register(x as usize, self.delay_timer),

            0x15 => self.delay_timer = cpu_manager.get_values_register(x as usize),

            0x18 => self.sound_timer = cpu_manager.get_values_register(x as usize),

            0x1e => {
                let i = cpu_manager.get_index_register();
                let vx = cpu_manager.get_values_register(x as usize) as u16;

                cpu_manager.set_index_register(i.wrapping_add(vx));

            }

            0x29 => {
                let vx = cpu_manager.get_values_register(x as usize);

                cpu_manager.set_index_register( vx as u16 * 5 ); // vx * 5 car chaque sprit fait 5 bytes de long
            }

            0x33 => {


                let vx = cpu_manager.get_values_register(x as usize);

                let first = vx / 100;
                let middle = (vx % 100 ) / 10;
                let last = vx % 10;

                let i = cpu_manager.get_index_register();

                self.ram[i as usize + 0] = first;
                self.ram[i as usize + 1] = middle;
                self.ram[i as usize + 2] = last;

            }

            0x55 => { // save register to memory

                let i = cpu_manager.get_index_register();

                for v_i in 0..=(x as u16) {
                    
                    self.ram[(v_i + i) as usize] = cpu_manager.get_values_register(v_i as usize);

                }

            }

            0x65 => { // load register from memory

                let i = cpu_manager.get_index_register();

                for v_i in 0..=(x as u16) {

                    cpu_manager.set_values_register(v_i as usize, self.ram[(v_i + i) as usize]);
                }
            }

           
            _ => println!("[!] exception sub-opcode (io_manager.rs):  {:x}", bytes[1])
        }

   
    }


}