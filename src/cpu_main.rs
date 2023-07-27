use std::{fs::File, io::{Read, Seek}, time::Duration, process::exit};

use sdl2::{render::Canvas, video::Window};
use crate::{io_manager::IOManager, misc_cpu::MiscCPU, constants::{TIMER_MAX_HZ, PROGRAM_FILE}};

fn load_program(io_manager: &mut IOManager)
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


        io_manager.put_into_ram(byte_buffer, i);



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

fn decode(bytes: [u8; 2], io_manger: &mut IOManager, cpu_manager: &mut MiscCPU, canvas: &mut Canvas<Window>)
{

    // décoder l'instruction

    let half_byte = (bytes[0] & 0xf0) >> 4; // on prend la première moitié d'un byte

    if bytes[0] == 0x00 && bytes[1] == 0xe0 
    {
        io_manger.clear_screen(canvas);
        return;
    }

    println!("half:{:x} => {:x}:{:x}", half_byte, bytes[0], bytes[1]);

    // switch des instructions
    match half_byte {

        0x1 => cpu_manager.jump(bytes), // check

        0x2 => cpu_manager.inst_2nnn(bytes),

        0x3 => cpu_manager.inst_3xnn(bytes[0] & 0x0f, bytes[1]),

        0x4 => cpu_manager.inst_4xnn(bytes[0] & 0x0f, bytes[1]),

        0x5 => cpu_manager.inst_5xy0(bytes),

        0x6 => cpu_manager.set_value_register_vx_6xnn(bytes[0] & 0x0f, bytes[1]), // check

        0x7 => cpu_manager.add_value_register_vx_7xnn(bytes[0] & 0x0f, bytes[1]), // check

        0x8 => cpu_manager.inst_8xy_x(bytes),

        0x9 => cpu_manager.inst_9xy0(bytes),

        0xa => cpu_manager.set_index_register_annn(bytes), // check

        0xd => io_manger.display(bytes, canvas, cpu_manager), // check

        0xf => io_manger.inst_fx_xx(cpu_manager, bytes), 

        0x0 => {

            match bytes[1] {
                
                0x0 => check_error(bytes),
                
                0xee => cpu_manager.inst_00ee(),

                _ => {}
            }
            
        }

        _ => println!("[!] exception opcode: {:x}", half_byte)
    }

}

fn fetch(io_manager: &mut IOManager, cpu_manager: &mut MiscCPU) -> [u8; 2]
{
    let word = io_manager.get_from_ram(cpu_manager.get_pc());
    cpu_manager.inc_pc();

    return  word;
}

pub fn cpu_main(canvas: &mut Canvas<Window>)
{
    let mut cpu_manager = MiscCPU::initialize();
    let mut io_manager = IOManager::initialize();

    load_program(&mut io_manager);

    loop 
    {
        let word = fetch(&mut io_manager, &mut cpu_manager);

        decode(word, &mut io_manager, &mut cpu_manager, canvas);


        
        canvas.present();



        let dur = Duration::from_millis(TIMER_MAX_HZ / 10);
        std::thread::sleep(dur);

    }

   
}