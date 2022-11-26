use std::{fs::File, io::{Read, Seek}, time::Duration, process::exit};

use sdl2::{render::Canvas, video::Window};
use crate::{io_manager::IOManager, constants::{TIMER_MAX_HZ, PROGRAM_FILE}};

pub fn fetch(iomanager: &mut IOManager) -> [u8; 2]
{

    // on ouvre le fichier
    let mut f = File::open(PROGRAM_FILE).unwrap();
    let mut byte_buffer = [0; 2];


    // on se déplace à l'endroit du curseur
    if f.seek( 
        std::io::SeekFrom::Current(
            iomanager.get_pc() as i64
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


    // on incrémente le program_counter
    iomanager.inc_pc();



    // on retourne l'instruction
    return byte_buffer;

}

pub fn load_program(iomanager: &mut IOManager)
{
        // on ouvre le fichier
        let mut f = File::open(PROGRAM_FILE).unwrap();
        let mut byte_buffer: [u8; 1] = [0; 1];

        for i in 0..(0x200) {
                
            // on se déplace à l'endroit du curseur
            if f.seek(std::io::SeekFrom::Current(i as i64)).is_err()
            {
                println!("error seeking file");
                exit(1);
            }
            
        
            // on lis la data
            if f.read(&mut byte_buffer).is_err() {
            
                println!("Error reading file");
                exit(1);
            }

            
            iomanager.load_into_ram(byte_buffer, i);

        }
    
    
    
   
}

pub fn decode(bytes: [u8; 2], iomanager: &mut IOManager, canvas: &mut Canvas<Window>)
{

    // décoder l'instruction

    let nipple = bytes[0] & 0xf0; // on prend la première moitié d'un byte
    
    if bytes[0] == 0x00 && bytes[1] == 0xe0 
    {
        iomanager.clear_screen(canvas);
        return;
    }

    match nipple {
        

        0x10 => iomanager.jump(bytes),

        0x60 => iomanager.set_value_register_vx(bytes[0] & 0x0f, bytes[1]),

        0x70 => iomanager.add_value_register_vx(bytes[0] & 0x0f, bytes[1]),

        0xa0 => iomanager.set_index_register(bytes),

        0xd0 => iomanager.display(bytes, canvas),

        _ => print!("")
    }

}


pub fn cpu_main(canvas: &mut Canvas<Window>)
{
    let mut iomanager = IOManager::initialize();

    load_program(&mut iomanager);

    loop 
    {
        let word = fetch(&mut iomanager);

        decode(word, &mut iomanager, canvas);

        
        let dur = Duration::from_millis(TIMER_MAX_HZ / 1000);
        std::thread::sleep(dur);
    }

   
}