use std::{fs::File, io::{Read, Seek, self}, time::Duration, process::exit};

use sdl2::{render::Canvas, video::Window};
use crate::{io_manager::IOManager, constants::{TIMER_MAX_HZ, PROGRAM_FILE}};

fn load_program(iomanager: &mut IOManager)
{

    for i in (0x200..0xfff).step_by(2) {
        
        
        // on ouvre le fichier
        let mut f = File::open(PROGRAM_FILE).unwrap();
        let mut byte_buffer = [0; 2];


        // on se déplace à l'endroit du curseur
        if f.seek( 
            std::io::SeekFrom::Current(
                i as i64 - 0x200
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


        iomanager.put_into_ram(byte_buffer, i);



    }

    


}
fn check_error(bytes: [u8; 2])
{
    if bytes[0] == 0x00 && bytes[1] == 0x00
    {
        println!("[!] Erreur de lecture d'oppcode: cpu_main.rs");
        exit(1);
    }
}

fn decode(bytes: [u8; 2], iomanager: &mut IOManager, canvas: &mut Canvas<Window>)
{

    // décoder l'instruction

    let half_byte = (bytes[0] & 0xf0) >> 4; // on prend la première moitié d'un byte

    if bytes[0] == 0x00 && bytes[1] == 0xe0 
    {
        iomanager.clear_screen(canvas);
        return;
    }

    println!("half:{:x} => {:x}:{:x}", half_byte, bytes[0], bytes[1]);

    // switch des instructions
    match half_byte {

        0x1 => iomanager.jump(bytes), // check

        0x2 => iomanager.inst_2nnn(bytes),

        0x3 => iomanager.inst_3xnn(bytes[0] & 0x0f, bytes[1]),

        0x4 => iomanager.inst_4xnn(bytes[0] & 0x0f, bytes[1]),

        0x5 => iomanager.inst_5xy0(bytes),

        0x6 => iomanager.set_value_register_vx(bytes[0] & 0x0f, bytes[1]), // check

        0x7 => iomanager.add_value_register_vx_7xnn(bytes[0] & 0x0f, bytes[1]), // check

        0x8 => iomanager.inst_8xy_x(bytes),

        0x9 => iomanager.inst_9xy0(bytes),

        0xa => iomanager.set_index_register_annn(bytes), // check

        0xd => iomanager.display(bytes, canvas), // check

        0xf => iomanager.inst_fx_xx(bytes), 

        0x0 => {

            match bytes[1] {
                
                0x0 => check_error(bytes),
                
                0xee => iomanager.inst_00ee(),

                _ => {}
            }
            
        }

        _ => println!("[!] exception opcode: {:x}", half_byte)
    }

}

fn fetch(iomanager: &mut IOManager) -> [u8; 2]
{
    let word = iomanager.get_from_ram(iomanager.get_pc());
    iomanager.inc_pc();

    return  word;
}

pub fn cpu_main(canvas: &mut Canvas<Window>)
{
    let mut iomanager = IOManager::initialize();
    load_program(&mut iomanager);

    loop 
    {
        let word = fetch(&mut iomanager);

        decode(word, &mut iomanager, canvas);


        
        canvas.present();



        let dur = Duration::from_millis(TIMER_MAX_HZ / 10);
        std::thread::sleep(dur);

    }

   
}