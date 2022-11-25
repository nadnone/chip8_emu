use std::{fs::{read, File}, io::{Read, Seek}, time::Duration, process::exit};

use sdl2::{render::Canvas, video::Window};
use crate::{io_manager::{IOManager, self}, constants::TIMER_MAX_HZ};

pub fn fetch(iomanager: &mut IOManager) -> [u8; 2]
{

    // on ouvre le fichier
    let mut f = File::open("./test_opcode.ch8").unwrap();
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
    

    // on lit l'instruction
    if f.read(&mut byte_buffer).is_err() {
     
        println!("Error reading file");
        exit(1);
    }


    // on incrémente le program_cursor
    iomanager.inc_pc();


    // on retourne l'instruction
    return byte_buffer;

}
pub fn decode(bytes: [u8; 2], iomanager: &mut IOManager, canvas: &mut Canvas<Window>)
{
    // décoder l'instruction

    let nipple = bytes[0] & 0xf0; // on prend la première moitié d'un byte
    
    if bytes[0] == 0x00 && bytes[1] == 0xe0 
    {
        iomanager.clear_screen(canvas);
    }

    println!("{:x}", nipple);

    match nipple {
        

        0x10 => iomanager.jump(((bytes[0] & 0x0f) + bytes[1]) as u16),

        0x60 => iomanager.set_value_registe_vx(bytes[0] & 0x0f, bytes[1]),

        0x70 => iomanager.add_value_register(bytes[0] & 0x0f, bytes[1]),

        0xa0 => iomanager.set_index_register(((bytes[0] & 0x0f) + bytes[1]) as u16),

        0xd0 => iomanager.display(bytes[0] & 0x0f, bytes[1] & 0xf0, bytes[1] & 0x0f, canvas),

        _ => print!("")
    }

}


pub fn cpu_main(canvas: &mut Canvas<Window>)
{
    let mut iomanager = IOManager::initialize();

    loop 
    {
        let word = fetch(&mut iomanager);

        decode(word, &mut iomanager, canvas);

        
        let dur = Duration::from_millis(TIMER_MAX_HZ / 1000);
        std::thread::sleep(dur);
    }

   
}