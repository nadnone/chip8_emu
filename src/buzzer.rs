
use sdl2::{audio::{AudioCallback, AudioSpecDesired, AudioDevice}, AudioSubsystem};


// https://docs.rs/sdl2/latest/sdl2/audio/index.html

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {

        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Buzzer {
    device: AudioDevice<SquareWave>,
}

impl Buzzer 
{
    pub fn init(audio_subsystem: &AudioSubsystem) -> Buzzer
    {
        let desired_spec = AudioSpecDesired {
            freq: Some(4000),
            channels: Some(1),  
            samples: None   
        };
        
        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        
            SquareWave {
                phase_inc: 400. / spec.freq as f32,
                phase: 0.,
                volume: 1.
            }
        }).unwrap();


        return Buzzer { 
            device: device,
        }
    }

    pub fn play(&mut self)
    {
        self.device.resume();
    }

    pub fn stop(&mut self)
    {
        self.device.pause();
    }
  
    pub fn change_freq(&mut self, freq: i32, pitch: f32)
    {

        let desired_spec = AudioSpecDesired {
            freq: Some(freq),
            channels: Some(1),  
            samples: None   
        };

        self.device = self.device.subsystem().open_playback(None, &desired_spec, |spec| {
        
            SquareWave {
                phase_inc: pitch / spec.freq as f32,
                phase: 0.,
                volume: 1.
            }
        }).unwrap();

    }

    
}


