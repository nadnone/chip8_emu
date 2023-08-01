use crate::{cpu_opcodes::CPUOpcodes, io_manager::IOManager};



pub fn inst_f002(cpu_manager: &CPUOpcodes, io_manager: &mut IOManager)
{
    let i = cpu_manager.get_index_register() as usize;

    let first = (io_manager.ram[i + 0] as u16) << 8; 
    let secnd = io_manager.ram[i + 1] as u16;

    io_manager.audio_pattern_buffer = first | secnd;

}

pub fn inst_fn3a(io_manager: &mut IOManager, cpu_manager: &CPUOpcodes, x: u8)
{

    let vx = cpu_manager.variables_register[x as usize] as u16;

    let playback_rate = 4000 * 2 ^ (( vx - 64 ) / 48 );

    io_manager.beeper.change_freq(playback_rate as i32, vx as f32); 

}