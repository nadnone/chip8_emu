use crate::constants::*;

pub struct MiscCPU {
    stack: Vec<u16>,
    stack_pointer: u16,
    program_counter: u16,
    index_register: u16,
    variables_register: [u8; REGISTER_SIZE],
}

impl MiscCPU {

    pub fn initialize() -> MiscCPU
    {

        // on charges les FONTS SPRITES 
        let mut ram_tmp = [0; RAM_SIZE_BYTES];
        for i in 0..80 
        {
            ram_tmp[i] = SPRITES_DATA[i];
        }


        return MiscCPU {
            program_counter: PROGRAM_COUNTER_START_ADDR,
            index_register: 0,
            variables_register: [0; REGISTER_SIZE],
            stack: vec![],
            stack_pointer: 0,
        }

    }

    pub fn set_values_register(&mut self, x: usize, var: u8)
    {
        self.variables_register[x] = var;
    }

    pub fn get_values_register(&self, x: usize) -> u8
    {
        return self.variables_register[x];
    }

    pub fn set_index_register(&mut self, var: u16)
    {
        self.index_register = var;
    }

    pub fn get_index_register(&self) -> u16
    {
        return self.index_register;
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

    pub fn set_value_register_vx_6xnn(&mut self, x: u8, nn: u8)
    {
        self.variables_register[x as usize] = nn;
    }

    pub fn add_value_register_vx_7xnn(&mut self, x: u8, nn: u8)
    {
        let var = self.variables_register[x as usize];

        // on bloque au max d'un 8bit
        self.variables_register[x as usize] = (var as u16 + nn as u16) as u8 % 255;
     
    }

    pub fn inst_8xy_x(&mut self, bytes: [u8; 2])
    {
        let x = bytes[0] & 0x0f;
        let y = (bytes[1] & 0xf0) >> 4;
        let last = bytes[1] & 0x0f;

        match last {
            
            0x0 => self.variables_register[x as usize] = self.variables_register[y as usize],

            0x1 => self.variables_register[x as usize] |= self.variables_register[y as usize], // OR bitwise

            0x2 => self.variables_register[x as usize] &= self.variables_register[y as usize], // AND bitwise
            
            0x3 => self.variables_register[x as usize] ^= self.variables_register[y as usize], // XOR bitwise
            
            0x4 => { // addition

                let add = self.variables_register[x as usize] as u16 + self.variables_register[y as usize] as u16;
                

                self.variables_register[0xf] = 0;

                if add > 0xff
                {
                    // si overflow flag à 1
                    self.variables_register[0xf] = 1;
                }
               
                self.variables_register[x as usize] = add as u8;

            }

            0x5 => { // soustraction VX - VY
                
                
                let vx = self.variables_register[x as usize]; 
                let vy = self.variables_register[y as usize];
                

                self.variables_register[0xf] = 0;

                if vx > vy
                {
                    // si overflow flag à 1
                    self.variables_register[0xf] = 1;
                }
               
                let sub = vx as i16 - vy as i16;

                self.variables_register[x as usize] = sub as u8;


            }

            0x6 => self.set_value_register_vx_6xnn(x, self.variables_register[y as usize] >> 1), // VY bitshift right 1

            0x7 => { // soustraction VY - VX
                
                let vx = self.variables_register[x as usize]; 
                let vy = self.variables_register[y as usize];
                

                self.variables_register[0xf] = 0;

                if vx < vy
                {
                    // si overflow flag à 1
                    self.variables_register[0xf] = 1;
                }
               
                let sub = vy as i16 - vx as i16;

                self.variables_register[x as usize] = sub as u8;

            }

            0xe => self.set_value_register_vx_6xnn(x, self.variables_register[y as usize] << 1), // VY bitshift left 1

            


            _ => println!("[!] exception sub-opcode (misc_cpu.rs): {:x}", last)
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

    pub fn set_index_register_annn(&mut self, bytes: [u8; 2])
    {
        let shifted = ((bytes[0] as u16) & 0x0f) << 8;

        self.index_register = shifted | bytes[1] as u16;
    }

  

   

}