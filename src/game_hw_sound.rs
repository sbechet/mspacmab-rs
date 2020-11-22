use crate::game_counter::{ Counter60Hz, CurrentTime };

/****************************************************************************/
// TODO: Historical: use more abstraction here...
#[derive(Copy, Clone)]
pub struct Wave {
    pub num: u8,
    pub cur_bit: u8,
    pub sel: u8,
    pub next_byte: usize,   // pointer
    pub type_w: u8,
    pub duration: u8,
    pub dir: u8,
    pub base_freq: u8,
    pub vol: u8,
}

impl Wave {
    pub fn new() -> Wave {
        Wave {
            num: 0,
            cur_bit: 0,
            sel: 0,
            next_byte: 0,   // pointer
            type_w: 0,
            duration: 0,
            dir: 0,
            base_freq: 0,
            vol: 0,
        }
    }
}

/****************************************************************************/
// TODO: Historical: use more abstraction here...
#[derive(Copy, Clone)]
pub struct Effect {
    pub num: u8,
    pub cur_bit: u8,
    pub table: [u8; 8],
    pub type_e: u8,
    pub duration: u8,
    pub dir: u8,
    pub base_freq: u8,
    pub vol: u8,
}

impl Effect {
    pub fn new() -> Effect {
        Effect {
            num: 0,
            cur_bit: 0,
            table: [0; 8],
            type_e: 0,
            duration: 0,
            dir: 0,
            base_freq: 0,
            vol: 0,
        }
    }
}

/****************************************************************************/
#[derive(Copy, Clone)]
pub struct SoundChannel {
    wave: u8,       // src:5045, src:504a, src:504f,
    freq: u32,      // src:5050, src:5056, src:505b,
    vol: u8,        // src:5055, src:505a, src:505f,
}

impl SoundChannel {

    pub fn new() -> Self {
        SoundChannel {
            wave: 0,
            freq: 0,
            vol: 0,
        }
    }

    pub fn set_wave(&mut self, wave: u8) {
        self.wave = wave;
    }

    pub fn set_freq(&mut self, freq: u32) {
        self.freq = freq;
    }

    pub fn set_vol(&mut self, vol: u8) {
        self.vol = vol;
    }
}


/****************************************************************************/
pub struct SoundChannels {
    pub channel: [SoundChannel; 3], // src:4e8c, src:4e92, src:4e97
    pub effect: [Effect; 3],    // src:4e9c, src:4eac, src:4ebc
    pub wave: [Wave; 3],        // src:4ecc, src:4edc, src:4eec
    pub accumulator: [u32; 3],  // src:5041, src:5046, src:504b
}


impl SoundChannels {
    pub fn new() -> Self {
        SoundChannels {
            channel: [SoundChannel::new(); 3],
            effect: [Effect::new(); 3],
            wave: [Wave::new(); 3],
            accumulator: [0; 3],
        }
    }

}

/****************************************************************************/
