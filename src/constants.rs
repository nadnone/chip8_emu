use sdl2::keyboard::Scancode;

pub const RAM_SIZE_BYTES: usize = 4096; // 4 kBytes
pub const DISPLAY: (usize, usize) = (64, 32);
pub const REGISTER_SIZE: usize = 16;

pub const SPRITES_DATA: [u8; 80] = [


    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80  //F

];

pub const TIMER_FREQ: f32 = 1. / 60.;
pub const CPU_FREQ: f32 = 1. / 2000.; // fréquence d'horloge

pub const PROGRAM_COUNTER_START_ADDR: u16 = 512;


pub const KEYS: [Scancode; 16] = [
    Scancode::Kp0,
    Scancode::Kp1,
    Scancode::Kp2,
    Scancode::Kp3,
    Scancode::Kp4,
    Scancode::Kp5,
    Scancode::Kp6,
    Scancode::Kp7,
    Scancode::Kp8,
    Scancode::Kp9,
    Scancode::A,
    Scancode::B,
    Scancode::C,
    Scancode::D,
    Scancode::E,
    Scancode::F
];