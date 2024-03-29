use sdl2::{render::Canvas, video::Window};
use sdl2::pixels::Color;
use sdl2::{rect::*, EventPump, AudioSubsystem};

use crate::buzzer::Buzzer;
use crate::{constants::*, xo_chip_extended};
use crate::cpu_opcodes::CPUOpcodes;
use crate::inputs::Inputs;

pub struct IOManager {
    pub ram: [u8; RAM_SIZE_BYTES],
    vga: [bool; DISPLAY.0 * DISPLAY.1],
    delay_timer: u8,
    sound_timer: u8,
    pub audio_pattern_buffer: u16,
    pub beeper: Buzzer,
}

impl IOManager {


    pub fn initialize(audio_subsys: &AudioSubsystem) -> IOManager
    {

        // on charges les FONTS SPRITES 
        let mut ram_tmp = [0; RAM_SIZE_BYTES];
        for i in 0..80 
        {
            ram_tmp[i] = SPRITES_DATA[i];
        }


        return IOManager {
            ram: ram_tmp,
            vga: [false; DISPLAY.0 * DISPLAY.1],
            delay_timer: 0,
            sound_timer: 0,
            audio_pattern_buffer: 0,
            beeper: Buzzer::init(audio_subsys),
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

    pub fn dec_delay_timer(&mut self)
    {
        if self.delay_timer > 0
        {
            self.delay_timer -= 1;
        }
    }

    pub fn sound_timer_check(&mut self)
    {
        
        if self.sound_timer > 0
        {
            self.beeper.play();
            self.sound_timer -= 1;

        }
        else
        {
            self.beeper.stop();            
        }
    }

    pub fn clear_screen(&mut self, canvas: &mut Canvas<Window>)
    {
        for i in 0..self.vga.len() {
            self.vga[i as usize] = false; // turn all pixels to 0
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

    }

    pub fn display(&mut self, bytes: u16, canvas: &mut Canvas<Window>, cpu_manager: &mut CPUOpcodes)
    {

        // les indexes de registre
        let vx_i = (bytes & 0xf00) >> 8; 
        let vy_i = (bytes & 0xf0) >> 4;

        // le nombre de sprites
        let n = bytes & 0xf;

        // les positions
        let vx = cpu_manager.variables_register[vx_i as usize] as u8;
        let vy = cpu_manager.variables_register[vy_i as usize] as u8;

        // flag
        cpu_manager.variables_register[0xf] =  0; // on reset

    

        for col in 0..n as u8 { // colonne de la sprite
            
            let sprite = self.ram[cpu_manager.index_register as usize + col as usize]; // informations concernant les pixels

            for row in 0..8 as u8 { // chaque ligne fait 8 pixel

                let sprite_bit = (sprite << row).reverse_bits() & 0x1; // le premier bit à gauche 

                if sprite_bit == 0 { // si 0 on continue, pas besoin d'agir
                    continue;
                };

                // valeurs absolues
                let x = (vx + row) % DISPLAY.0 as u8;
                let y = (vy + col) % DISPLAY.1 as u8;

                let pixel_loc = (x as u16 + (y as u16 * DISPLAY.0 as u16)) as usize;

                self.vga[pixel_loc] ^= true;

                if self.vga[pixel_loc] == true
                {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
                else
                {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    cpu_manager.variables_register[0xf] = 1;
                }
                
                canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();


            } 
            
        }

        canvas.present();

        
        
        
    }

    pub fn get_from_ram(&self, i: u16) -> [u8; 2]
    {
        if i + 1 > self.ram.len() as u16
        {
            return [0,0];
        }

        return [self.ram[i as usize], self.ram[i as usize + 1]];
    }

    /// word : data | i : index
    pub fn put_into_ram(&mut self, word: [u8; 2], i: u16)
    {
        self.ram[i as usize] = word[0];
        self.ram[i as usize + 1] = word[1];
    }


    pub fn inst_fx_xx(&mut self, cpu_manager: &mut CPUOpcodes, bytes: u16, event: &mut EventPump)
    {
        let x = ((bytes & 0xf00) >> 8) as u8;

        match bytes & 0xff {
            
            /*
                XO-CHIP Extension http://johnearnest.github.io/Octo/docs/XO-ChipSpecification.html

             */

            0x02 => xo_chip_extended::inst_f002(cpu_manager, self),
            

            0x3a => xo_chip_extended::inst_fn3a(self, &cpu_manager, x),


            /*
                **************************************************************************************
             */



            0x07 => cpu_manager.variables_register[x as usize] = self.delay_timer,

            0x15 => self.delay_timer = cpu_manager.variables_register[x as usize],

            0x18 => self.sound_timer = cpu_manager.variables_register[x as usize],

            0x1e => {
                let i = cpu_manager.index_register;
                let vx = cpu_manager.variables_register[x as usize] as u16;

                cpu_manager.index_register = i.wrapping_add(vx);

            }

            0x29 => {
                let vx = cpu_manager.variables_register[x as usize];

                cpu_manager.index_register =  vx as u16 * 5; // vx * 5 car chaque sprit fait 5 bytes de long
            }

            0x33 => {


                let vx = cpu_manager.variables_register[x as usize];

                let first = vx / 100;
                let middle = (vx % 100 ) / 10;
                let last = vx % 10;

                let i = cpu_manager.index_register;

                self.ram[i as usize + 0] = first;
                self.ram[i as usize + 1] = middle;
                self.ram[i as usize + 2] = last;

            }

            0x55 => { // save register to memory

                let i = cpu_manager.index_register;

                for v_i in 0..=(x as u16) {
                    
                    self.ram[(v_i + i) as usize] = cpu_manager.variables_register[v_i as usize];
                    cpu_manager.index_register += 1;

                }

            }

            0x65 => { // load register from memory

                let i = cpu_manager.index_register;

                for v_i in 0..=(x as u16) {

                    cpu_manager.variables_register[v_i as usize] = self.ram[(v_i + i) as usize];
                    cpu_manager.index_register += 1;

                }
            }

            0x00a => { // wait to get key 
                
                cpu_manager.set_wait_control(true);

                for i in 0..16
                {
                    if Inputs::is_key_pressed(event, i)
                    {
                        cpu_manager.set_wait_control(false);
                    }
                }


            }

           
            _ => println!("[!] exception sub-opcode (io_manager.rs):FX:  {:x}", bytes)
        }

   
    }

    pub fn ex_skip_key(&self, bytes: u16, cpu_manager: &mut CPUOpcodes, event: &mut EventPump)
    {
        let x = ((bytes & 0xf00) >> 8) as u8;

        let vx = cpu_manager.variables_register[x as usize];

        match bytes & 0xff {

            0x9e => {
                // if vx key pressed 
                if Inputs::is_key_pressed(event, vx as usize)
                {
                    cpu_manager.inc_pc();
                }
            }

            0xa1 => {
               
                // if vx key not pressed 
                if !Inputs::is_key_pressed(event, vx as usize)
                {
                    cpu_manager.inc_pc();
                }
            }
            
            _ => println!("[!] exception sub-opcode (io_manager.rs):EX:  {:x}", bytes)
        }

    }

}