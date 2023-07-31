use crate::constants::*;

pub struct CPUOpcodes {
    stack: Vec<u16>,
    program_counter: u16,
    index_register: u16,
    variables_register: [u8; REGISTER_SIZE],
    wait: bool,
}

impl CPUOpcodes {

    pub fn initialize() -> CPUOpcodes
    {

        // on charges les FONTS SPRITES 
        let mut ram_tmp = [0; RAM_SIZE_BYTES];
        for i in 0..80 
        {
            ram_tmp[i] = SPRITES_DATA[i];
        }


        return CPUOpcodes {
            program_counter: PROGRAM_COUNTER_START_ADDR,
            index_register: 0,
            variables_register: [0; REGISTER_SIZE],
            stack: vec![],
            wait: false,
        }

    }

    pub fn _get_register_range_array(&self, x: usize, y: usize)
    {
        let mut range = vec![];
        for i in x..y+1 {
            range.push(self.variables_register[i]);
        }
        
        println!("Register range data: {:?}", range);
    }

    pub fn set_wait_control(&mut self, state: bool)
    {
        self.wait = state;
    }

    pub fn get_wait_control(&self) -> bool
    {
        return self.wait;
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

    pub fn inst_1nnn(&mut self, bytes: u16)
    {
        // jump to nnn
        self.set_pc(bytes & 0xfff);

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

    pub fn inst_2nnn(&mut self, bytes: u16)
    {

        // on push to stack
        self.stack.push(self.get_pc());

        //on change le pc à nnn
        self.set_pc(bytes & 0xfff);

    }

    pub fn inst_3xnn(&mut self, bytes: u16)
    {

        let x = ((bytes & 0xf00) >> 8) as u8;
        let nn = (bytes & 0xff) as u8;

        if self.variables_register[x as usize] == nn // if VX == NN
        {  
            // skip instruction
            self.inc_pc();
        }
    }

    pub fn inst_00ee(&mut self)
    {

        if self.stack.is_empty() 
        {
            return;
        }

        // popping from stack
        let subroutine = self.stack.pop().unwrap();

        self.set_pc(subroutine);

    }

    pub fn inst_4xnn(&mut self, bytes: u16)
    {

        let x = ((bytes & 0xf00) >> 8) as u8;
        let nn = (bytes & 0xff) as u8;

        if self.variables_register[x as usize] != nn // if VX != NN
        {  
            // skip instruction
            self.inc_pc();
        }
    }

    pub fn inst_5xy0(&mut self, bytes: u16)
    {
        let x = ((bytes & 0xf00) >> 8) as u8;
        let y = ((bytes & 0xf0) >> 4) as u8;

        if self.variables_register[x as usize] == self.variables_register[y as usize] // VX == VY
        {
            self.inc_pc(); // skip
        }
    }

    pub fn inst_6xnn(&mut self, bytes: u16)
    {
        let x = ((bytes & 0xf00) >> 8) as u8;
        let nn = (bytes & 0xff) as u8;

        self.variables_register[x as usize] = nn;
    }

    pub fn inst_7xnn(&mut self, bytes: u16)
    {
        let x = ((bytes & 0xf00) >> 8) as u8;
        let nn = (bytes & 0xff) as u8;

        let vx = self.variables_register[x as usize];

        self.variables_register[x as usize] = vx.wrapping_add(nn);
     
    }

    pub fn inst_8xy_x(&mut self, bytes: u16)
    {
        let x = ((bytes & 0xf00) >> 8) as u8;
        let y = ((bytes & 0xf0) >> 4) as u8;
        let last = bytes & 0xf;

        match last {
            
            0x0 => self.variables_register[x as usize] = self.variables_register[y as usize],

            0x1 => self.variables_register[x as usize] |= self.variables_register[y as usize], // OR bitwise

            0x2 => self.variables_register[x as usize] &= self.variables_register[y as usize], // AND bitwise
            
            0x3 => self.variables_register[x as usize] ^= self.variables_register[y as usize], // XOR bitwise
            
            0x4 => { // addition

                let vx = self.variables_register[x as usize];
                let vy = self.variables_register[y as usize];
                
                self.variables_register[x as usize] = vx.wrapping_add(vy);
               
                if vx as u16 + vy as u16 > 0xff
                {
                    // si overflow flag à 1
                    self.variables_register[0xf] = 1;
                }
                else {
                    self.variables_register[0xf] = 0; 
                }

            }

            0x5 => { // soustraction VX - VY
                
                
                let vx = self.variables_register[x as usize]; 
                let vy = self.variables_register[y as usize];
                
                self.variables_register[x as usize] = vx.wrapping_sub(vy);

                if vx > vy
                {
                    // si underflow flag à 1
                    self.variables_register[0xf] = 1;
                }
                else {
                    self.variables_register[0xf] = 0;
                }

            }

            0x6 => {

                self.variables_register[x as usize] = self.variables_register[y as usize]; // set VX value VY
                
                let flag = self.variables_register[x as usize] & 0x1; // last bit before shift
                
                self.variables_register[x as usize] >>= 1; // right shift
                
                self.variables_register[0xf] = flag; // set flag at end, VX could be VF
            }

            0x7 => { // soustraction VY - VX
                
                let vx = self.variables_register[x as usize]; 
                let vy = self.variables_register[y as usize];

                self.variables_register[x as usize] = vy.wrapping_sub(vx);

                if vy > vx
                {
                    // si underflow flag à 1
                    self.variables_register[0xf] = 1;
                }
                else {
                    self.variables_register[0xf] = 0;
                }

            }

            0xe => {
                self.variables_register[x as usize] = self.variables_register[y as usize]; // set VX value VY
                
                let flag = self.variables_register[x as usize] & 0x80; // first bit before shift

                self.variables_register[x as usize] <<= 1; // left shift
                
                self.variables_register[0xf] = flag; // set flag at end, VX could be VF
            
            }


            _ => println!("[!] exception sub-opcode (misc_cpu.rs): {:x}", last)
        }

   
    }
    
    pub fn inst_9xy0(&mut self, bytes: u16)
    {
        let x = ((bytes & 0xf00) >> 8) as u8;
        let y = (bytes & 0xf0) >> 4 as u8;

        if self.variables_register[x as usize] != self.variables_register[y as usize] // VX != VY
        {
            self.inc_pc(); // skip
        }
    }

    pub fn inst_annn(&mut self, bytes: u16)
    {
        let nnn = bytes & 0xfff;

        self.index_register = nnn;
    }

  
    pub fn inst_cxnn(&mut self, bytes: u16)
    {
        let x = ((bytes & 0xf00) >> 8) as u8;
        
        let random = rand::random::<u8>();

        self.variables_register[x as usize] = random & (bytes & 0xff) as u8;
    }

   

}