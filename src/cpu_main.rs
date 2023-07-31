use std::{fs::File, io::{Read, Seek}, process::exit};

use sdl2::{render::Canvas, video::Window, EventPump};
use crate::{io_manager::IOManager, cpu_opcodes::CPUOpcodes, constants::DURATION_CPU, inputs::Inputs};

fn load_program(io_manager: &mut IOManager, filename: &str)
{

    // à déplacer dans IO_manager.rs

    for i in (0x200..0xfff).step_by(2) {
        
        
        // on ouvre le fichier
        let mut f = File::open(filename).unwrap();
        let mut byte_buffer = [0; 2];


        // on se déplace à l'endroit du curseur
        if f.seek( 
            std::io::SeekFrom::Current(
                i as i64 - 512
            ) 
        ).is_err()
        {
            println!("Reading position error");
            exit(1);
        }
        

        // on lis l'instruction
        if f.read(&mut byte_buffer).is_err() {
        
            println!("Error reading file");
            exit(1);
        }


        io_manager.put_into_ram(byte_buffer, i);



    }

    


}
fn check_error(bytes: u16)
{
    if bytes == 0x0000
    {
        println!("[!] Erreur de lecture d'oppcode: cpu_main.rs");
        exit(1);
    }
}

fn decode(bytes: u16, io_manager: &mut IOManager, cpu_manager: &mut CPUOpcodes, canvas: &mut Canvas<Window>, event_pump: &mut EventPump)
{

    // décoder l'instruction

    let half_byte = (bytes & 0xf000) >> 12; // on prend la première moitié d'un byte


    //println!("half:{:x} => {:x}", half_byte, bytes);

    // switch des instructions
    match half_byte {

        0x1 => cpu_manager.inst_1nnn(bytes), 

        0x2 => cpu_manager.inst_2nnn(bytes), 

        0x3 => cpu_manager.inst_3xnn(bytes), 

        0x4 => cpu_manager.inst_4xnn(bytes), 

        0x5 => cpu_manager.inst_5xy0(bytes), 

        0x6 => cpu_manager.inst_6xnn(bytes), 

        0x7 => cpu_manager.inst_7xnn(bytes), 

        0x8 => cpu_manager.inst_8xy_x(bytes),

        0x9 => cpu_manager.inst_9xy0(bytes),

        0xa => cpu_manager.inst_annn(bytes), 

        0xd => io_manager.display(bytes, canvas, cpu_manager),

        0xe => io_manager.ex_skip_key(bytes, cpu_manager, event_pump),

        0xf => io_manager.inst_fx_xx(cpu_manager, bytes, event_pump),

        0x0 => {

            match bytes & 0xff {
                
                0x0 => check_error(bytes),
                
                0xe0 => io_manager.clear_screen(canvas),

                0xee => cpu_manager.inst_00ee(),

                _ => println!("[!] exception sub-opcode (cpu.main.rs): {:x}", bytes)
            }
            
        }

        _ => println!("[!] exception opcode: {:x}", half_byte)
    }


    //cpu_manager._get_register_range_array(0xf, 0xf);


}

fn fetch(io_manager: &mut IOManager, cpu_manager: &mut CPUOpcodes) -> u16
{
    let bytes = io_manager.get_from_ram(cpu_manager.get_pc());

    let word = ((bytes[0] as u16) << 8) | bytes[1] as u16;

    cpu_manager.inc_pc();

    return word;
}

pub fn cpu_main(canvas: &mut Canvas<Window>, argv: Vec<String>, event_pump: &mut EventPump)
{
    let mut cpu_manager = CPUOpcodes::initialize();
    let mut io_manager = IOManager::initialize();

    if argv.len() < 1
    {
        println!("[!] ./chip8_emu.exe file_rom.ch8");
        return;
    }

    load_program(&mut io_manager, argv[1].as_str());

    let mut word: u16 = 0;

    loop 
    {

        Inputs::check(event_pump);

        if !cpu_manager.get_wait_control() // if it is on pause
        {
            word = fetch(&mut io_manager, &mut cpu_manager);
        }
        else 
        {
            io_manager.dec_delay_timer();
        }

        decode(word, &mut io_manager, &mut cpu_manager, canvas, event_pump);
        
        canvas.present();
    


        std::thread::sleep(DURATION_CPU);

    }

   
}