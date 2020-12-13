#[derive(PartialEq,PartialOrd,Copy,Clone, Debug)]
pub enum CurrentTime {
    None = 0,
    LessTenth = 1,
    Tenth = 2,
    Second = 3,
    TenSeconds = 4,
    Minute = 5,
    TenMinutes = 6,
    Hour = 7,
    TenHours = 8,
    Hundred = 9,
}

pub struct Counter60Hz {

    // src:4c84
    inc_counter: u8,    // incremented each VBLANK
    // src:4c85
    dec_counter: u8,    // decremented each VBLANK

    // src:4c86 src:4c87 src:4c88 src:4c89
    // 0: low quartet: vblank (60Hz) counter (when 6 : 6/60 s == 1/10s) - high quartet: number of 1/10s
    // 1: low quartet: seconds counters - high quartet: number of 10 sec
    // 2: high quartet: number of 10 min - low quartet: 6 minutes counters
    counter:[u8; 4],    // idx=0 for 1/10s, idx=1 for seconds, idx=2 for minutes, idx=3 for hours


    // 0x01    0          <= t < 1/10 s
    // 0x02    1/10       <= t < 1 second
    // 0x03    1 second   <= t < 10 seconds
    // 0x04    10 seconds <= t < 1 minute
    // 0x05    1 minute   <= t < 10 minutes
    // 0x06    10 minutes <= t < 1 hour
    // 0x07    1 hour     <= t < 10 hours
    // 0x08    10 hours   <= t < 100 hours
    // 0x09    100 hours  <= t
    // src:4c8a
    step_state: u8,

    // src:4c8b
    rand1: u8,
    // src:4c8c
    rand2: u8,
}

const LIMIT_60_HZ:[[u8; 2]; 4] = [ [ 6, 0xA0],  // tenth: low_quartet: 6*1/60s = 1/10s. high_quartet: 10 => 1 second
                                   [10, 0x60],  // seconds: low_quartet: 10*1s = 10s. high_quartet: 6 => 1 minute
                                   [10, 0x60],  // minutes: low_quartet: 10*1m = 10m. high_quartet: 6 => 1 hour
                                   [10, 0xA0]]; // hours:  low_quartet: 10*1h = 10h. high_quartet: 10 => 100 hours


impl Counter60Hz {

    pub fn new() -> Self {
        Counter60Hz {
            inc_counter: 0,
            dec_counter: 0,
            counter: [0; 4],
            step_state: 0,
            rand1: 0,
            rand2: 0,
        }
    }

    // src:01dc
    // update_timers_and_random_number
    pub fn update(&mut self) {

        self.inc_counter = self.inc_counter.wrapping_add(1);
        self.dec_counter = self.dec_counter.wrapping_sub(1);

        let mut c = 1;

        // counters: tenth, seconds, minutes, hours
        for i in 0..4 {
            self.counter[i] +=1;

            if (self.counter[i] & 0xf) == LIMIT_60_HZ[i][0] {
                c += 1;
                self.counter[i] += 0x10;    // increment high quartet (1/10s, secs, mins, hours)
                self.counter[i] &= 0xf0;    // reset low quartet counter
                if self.counter[i] == LIMIT_60_HZ[i][1] {
                    c += 1;
                    self.counter[i] = 0;
                }
            }
        }

        self.step_state = c;

        self.rand1 = self.rand1.wrapping_mul(5).wrapping_add(1);
        self.rand2 = self.rand2.wrapping_mul(13).wrapping_add(1);
    }

    pub fn get(&self) -> CurrentTime {
        match self.step_state {
            1 => CurrentTime::LessTenth,
            2 => CurrentTime::Tenth,
            3 => CurrentTime::Second,
            4 => CurrentTime::TenSeconds,
            5 => CurrentTime::Minute,
            6 => CurrentTime::TenMinutes,
            7 => CurrentTime::Hour,
            8 => CurrentTime::TenHours,
            9 => CurrentTime::Hundred,
            _ => CurrentTime::None,
        }
    }

    pub fn get_counter(&self) -> u8 {
        self.inc_counter
    }

    pub fn get_rand1(&self) -> u8 {
        self.rand1
    }

    pub fn get_rand2(&self) -> u8 {
        self.rand2
    }

}