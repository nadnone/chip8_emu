use sdl2::EventPump;
use crate::{sdl2::keyboard::Scancode, constants::KEYS};

pub struct Inputs;


impl Inputs {

    pub fn check(event: &mut EventPump)
    {
        if event.poll_event().is_none() // if no event, pass
        {
            return;
        }

        let key_state = event.keyboard_state();

        if key_state.is_scancode_pressed(Scancode::Escape)
        {
            std::process::exit(0);
        }

    }

    pub fn is_key_pressed(event: &mut EventPump, key: usize) -> bool
    {
        for (keys, state) in event.keyboard_state().scancodes()
        {
            if KEYS[key] == keys && state == true
            {
                return true;
            }
        }

        return false;

    }

}