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
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>,
    stack_pointer: u16
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
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![],
            stack_pointer: 0
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

    pub fn set_index_register_annn(&mut self, bytes: [u8; 2])
    {
        let shifted = ((bytes[0] as u16) & 0x0f) << 8;

        self.index_register = shifted | bytes[1] as u16;
    }
    
    pub fn set_value_register_vx(&mut self, x: u8, nn: u8)
    {
        self.variables_register[x as usize] = nn;
    }

    pub fn add_value_register_vx_7xnn(&mut self, x: u8, nn: u8)
    {
        let var = self.variables_register[x as usize];

        // on bloque au max d'un 8bit
        self.variables_register[x as usize] = (var as u16 + nn as u16) as u8 % 255;
     
    }

    pub fn jump(&mut self, bytes: [u8; 2])
    {
        let shifted = ((bytes[0] & 0x0f) as u16) << 8;

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

    /// word : data | i : index
    pub fn put_into_ram(&mut self, word: [u8; 2], i: u16)
    {
        self.ram[i as usize] = word[0];
        self.ram[i as usize + 1] = word[1];
    }

    pub fn inst_3xnn(&mut self, x: u8, nn: u8)
    {
        if self.variables_register[x as usize] == nn // if VX == NN
        {  
            // skip instruction
            self.inc_pc();
        }
    }

    pub fn inst_2nnn(&mut self, nnn: [u8; 2])
    {
        // on incrémente le stack
        self.stack_pointer += 1;

        // on récupère le nouveau PC
        let first = ((nnn[0] & 0x0f) as u16) << 8;
        let secnd = nnn[1] as u16;

        // on push to stack
        self.stack.push(self.get_pc());

        //on change le pc à nnn
        self.set_pc(first | secnd);

    }

    pub fn inst_00ee(&mut self)
    {
        // popping from stack
        let subroutine = self.stack.pop().unwrap();

        self.set_pc(subroutine);

    }

    pub fn inst_4xnn(&mut self, x: u8, nn: u8)
    {
        if self.variables_register[x as usize] != nn // if VX != NN
        {  
            // skip instruction
            self.inc_pc();
        }
    }

    pub fn inst_5xy0(&mut self, bytes: [u8; 2])
    {
        let x = bytes[0] & 0x0f;
        let y = (bytes[1] & 0xf0) >> 4;

        if self.variables_register[x as usize] == self.variables_register[y as usize] // VX == VY
        {
            self.inc_pc(); // skip
        }
    }

    pub fn inst_9xy0(&mut self, bytes: [u8; 2])
    {
        let x = bytes[0] & 0x0f;
        let y = (bytes[1] & 0xf0) >> 4;

        if self.variables_register[x as usize] != self.variables_register[y as usize] // VX != VY
        {
            self.inc_pc(); // skip
        }
    }

    pub fn inst_8xy_x(&mut self, bytes: [u8; 2])
    {
        let x = bytes[0] & 0x0f;
        let y = (bytes[1] & 0xf0) >> 4;
        let last = bytes[1] & 0x0f;

        match last {
            
            0x0 => self.set_value_register_vx(x, y),

            0x1 => self.set_value_register_vx(x, self.variables_register[x as usize] | x),

            _ => println!("[!] exception sub-opcode: {:x}", last)
        }

   
    }

    pub fn inst_fx_xx(&mut self, bytes: [u8; 2])
    {
        let x: u8 = bytes[0] & 0x0f;

        println!("{:x}", x);
        println!("{:x}", bytes[1]);

        match bytes[1] {
            
            0x07 => self.set_value_register_vx(x, self.delay_timer),

            0x15 => self.delay_timer = self.variables_register[x as usize],

            0x18 => self.sound_timer = self.variables_register[x as usize],

            0x29 => {
                let ram_data = self.get_from_ram(x as u16);
                self.index_register = (ram_data[0] | ram_data[1]) as u16;
            }

           
            _ => println!("[!] exception sub-opcode: {:x}", bytes[1])
        }

   
    }


}