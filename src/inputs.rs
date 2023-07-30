use sdl2::EventPump;

pub struct Inputs;

impl Inputs {

    pub fn check(event: &mut EventPump)
    {
        if event.poll_event().is_none() // if no event, pass
        {
            return;
        }

        if event.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::Escape)
        {
            std::process::exit(0);
        }

    }
    
}