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
fn check_error(i: u8)
{
    if i > 3 
    {
        println!("[!] Erreur de lecture d'oppcode: cpu_main.rs");
        exit(1);
    }
}

pub fn decode(bytes: [u8; 2], iomanager: &mut IOManager, canvas: &mut Canvas<Window>, i: u8)
{

    // décoder l'instruction

    let half_byte = bytes[0] & 0xf0; // on prend la première moitié d'un byte

    if bytes[0] == 0x00 && bytes[1] == 0xe0 
    {
        iomanager.clear_screen(canvas);
        return;
    }


    // switch des instructions
    match half_byte {

        0x10 => iomanager.jump(bytes),

        0x60 => iomanager.set_value_register_vx(bytes[0] & 0x0f, bytes[1]),

        0x70 => iomanager.add_value_register_vx(bytes[0] & 0x0f, bytes[1]),

        0xa0 => iomanager.set_index_register(bytes),

        0xd0 => iomanager.display(bytes, canvas),

        0x00 => check_error(i),

        _ => print!("")
    }

}


pub fn cpu_main(canvas: &mut Canvas<Window>)
{
    let mut iomanager = IOManager::initialize();

    let mut i = 0;
    loop 
    {
        let word = fetch(&mut iomanager);

        decode(word, &mut iomanager, canvas, i);

        

        
        canvas.present();



        let dur = Duration::from_millis(TIMER_MAX_HZ / 1000);
        std::thread::sleep(dur);

        i += 1;
    }

   
}