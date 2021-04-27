use crate::game_playing::GamePlaying;

// src:0068
pub const DIFFICULTY_EASY: [usize;21] = [  1,  3,4,  6,7,8,9,10,11,12,13,14,15,16,17,      20,20,20,20,20,20]; // RUST HACK: last five 20 for array equality
// src:007d
pub const DIFFICULTY_HARD: [usize;21] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];


pub struct GhostDifficulty {
        // 0: (0..6) speed bit patterns and orientation changes (table at #330F)
        pub speed_and_orientation: &'static GhostSpeedAndOrientation,
        // 2: (0..3) ghost counter table to exit home (table at #0843)
        pub out_counter: &'static [u8; 3],
        // 3: (0..7) remaining number of pills to set difficulty flags (table at #084F)
        pub pill_counter: &'static [u8; 2],
        // 4: (0..8) ghost time to stay blue when pacman eats the big pill (table at #0861)
        pub blue_time: &'static u16,
        // 5: (0..2) number of units before a ghost goes out of home (table at #0873)
        pub leaves_home_time: &'static u16,
}

impl GhostDifficulty {
    // src:272c
    pub fn get_difficulty_settings(hard: bool) -> &'static [usize; 21] {
        if hard {
            &DIFFICULTY_HARD
        } else {
            &DIFFICULTY_EASY
        }
    }
}

// src:0796
pub const GHOST_DIFFICULTY:[GhostDifficulty; 21] = [
    GhostDifficulty {
        speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[3], 
        out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[1], 
        pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[0], 
        blue_time: &GHOST_DIFFICULTY_BLUE_TIME[2], 
        leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[0],
    },
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[4], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[2], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[1], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[3], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[0],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[4], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[2], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[4], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[1],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[4], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[2], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[5], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[1],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[2], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[6], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[3], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[3], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[3], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[6], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[3], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[6], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[4], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[7], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[4], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[3], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[4], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[6], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[5], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[7], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[5], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[7], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[5], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[5], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[6], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[7], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[6], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[7], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[6], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[8], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[6], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[7], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[7], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[8], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[5], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[7], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[8], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
    GhostDifficulty {speed_and_orientation: &GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[6], out_counter: &GHOST_DIFFICULTY_OUT_COUNTER[3], pill_counter: &GHOST_DIFFICULTY_PILL_COUNTER[7], blue_time: &GHOST_DIFFICULTY_BLUE_TIME[8], leaves_home_time: &GHOST_DIFFICULTY_LEAVES_HOME_TIME[2],},
];

#[derive(Copy, Clone)]
pub struct GhostSpeedAndOrientation 
{
    speed_bit_pattern: ManSpeedBitPatterns,
    ghost_data_movement_bit_pattern: GhostSpeedBitPatterns,
    ghost_counter_for_orientation_change: [u16; 7],
}

impl GhostSpeedAndOrientation
{
    // src:0814
    pub fn copy_difficulty_movement_bit_pattern(self, playing: &mut GamePlaying) {
        playing.man_movement = self.speed_bit_pattern;
        playing.red_ghost_movement = self.ghost_data_movement_bit_pattern;
        playing.pink_ghost_movement = self.ghost_data_movement_bit_pattern;
        playing.blue_ghost_movement = self.ghost_data_movement_bit_pattern;
        playing.orange_ghost_movement = self.ghost_data_movement_bit_pattern;
        playing.ghost_counter_for_orientation_change = self.ghost_counter_for_orientation_change;
    }
}

#[derive(Copy, Clone)]
pub struct ManSpeedBitPatterns {
    pub normal_state: u32,
    pub big_pill_state: u32,
    pub second_difficulty_flag: u32,
    pub first_difficulty_flag: u32,
}

impl ManSpeedBitPatterns {
    pub fn new(id: usize) -> ManSpeedBitPatterns {
        GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[id&7].speed_bit_pattern
    }
}

#[derive(Copy, Clone)]
pub struct GhostSpeedBitPatterns {
    pub normal_state: u32,
    pub speed_state: u32, // eg. blue ghost
    pub tunnel_state: u32, // eg. tunnel areas
}

impl GhostSpeedBitPatterns {
    pub fn new(id: usize) -> GhostSpeedBitPatterns {
        GHOST_DIFFICULTY_SPEED_AND_ORIENTATION[id&7].ghost_data_movement_bit_pattern
    }
}

// src:330f
const GHOST_DIFFICULTY_SPEED_AND_ORIENTATION: [GhostSpeedAndOrientation; 7] = [
    // 0
    GhostSpeedAndOrientation {
        speed_bit_pattern: ManSpeedBitPatterns { normal_state: 0x552a552a, big_pill_state: 0x55555555, second_difficulty_flag: 0x552a552a, first_difficulty_flag: 0x524aa594 },
        ghost_data_movement_bit_pattern: GhostSpeedBitPatterns { normal_state: 0x25252525, speed_state: 0x22222222, tunnel_state: 0x01010101 },
        ghost_counter_for_orientation_change: [0x0258, 0x0708, 0x0960, 0x0e10, 0x1068, 0x1770, 0x1914],
    },
    // 1
    GhostSpeedAndOrientation {
        speed_bit_pattern: ManSpeedBitPatterns { normal_state: 0x524aa594, big_pill_state: 0xaa2a5555, second_difficulty_flag: 0x552a552a, first_difficulty_flag: 0x524aa594},
        ghost_data_movement_bit_pattern: GhostSpeedBitPatterns { normal_state: 0x92242549, speed_state: 0x48242291, tunnel_state: 0x01010101 },
        ghost_counter_for_orientation_change: [0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000],
    },
    // 2
    GhostSpeedAndOrientation {
        speed_bit_pattern: ManSpeedBitPatterns { normal_state: 0x552a552a, big_pill_state: 0x55555555, second_difficulty_flag: 0xaa2a5555, first_difficulty_flag: 0x552a552a},
        ghost_data_movement_bit_pattern: GhostSpeedBitPatterns { normal_state: 0x524aa594, speed_state: 0x48242291, tunnel_state: 0x21444408 },
        ghost_counter_for_orientation_change: [0x0258, 0x0834, 0x09d8, 0x0fb4, 0x1158, 0x1608, 0x1734],
    },
    // 3
    GhostSpeedAndOrientation {
        speed_bit_pattern: ManSpeedBitPatterns { normal_state: 0x55555555, big_pill_state: 0xd56ad56a, second_difficulty_flag: 0xaa6a55d5, first_difficulty_flag: 0x55555555},
        ghost_data_movement_bit_pattern: GhostSpeedBitPatterns { normal_state: 0xaa2a5555, speed_state: 0x92249224, tunnel_state: 0x22222222 },
        ghost_counter_for_orientation_change: [0x01a4, 0x0654, 0x07f8, 0x0ca8, 0x0dd4, 0x1284, 0x13b0],
    },
    // 4
    GhostSpeedAndOrientation {
        speed_bit_pattern: ManSpeedBitPatterns { normal_state: 0xd56ad56a, big_pill_state: 0xd65aadb5, second_difficulty_flag: 0xd65aadb5, first_difficulty_flag: 0xd56ad56a},
        ghost_data_movement_bit_pattern: GhostSpeedBitPatterns { normal_state: 0xaa6a55d5, speed_state: 0x92242549, tunnel_state: 0x48242291 },
        ghost_counter_for_orientation_change: [0x01a4, 0x0654, 0x07f8, 0x0ca8, 0x0dd4, 0xfffe, 0xffff],
    },
    // 5
    GhostSpeedAndOrientation {
        speed_bit_pattern: ManSpeedBitPatterns { normal_state: 0x6d6d6d6d, big_pill_state: 0x6d6d6d6d, second_difficulty_flag: 0xb66d6ddb, first_difficulty_flag: 0x6d6d6d6d},
        ghost_data_movement_bit_pattern: GhostSpeedBitPatterns { normal_state: 0xd65aadb5, speed_state: 0x25252525, tunnel_state: 0x92249224 },
        ghost_counter_for_orientation_change: [0x012c, 0x05dc, 0x0708, 0x0bb8, 0x0ce4, 0xfffe, 0xffff],
    },
    // 6
    GhostSpeedAndOrientation {
        speed_bit_pattern: ManSpeedBitPatterns { normal_state: 0xd56ad56a, big_pill_state: 0xd56ad56a, second_difficulty_flag: 0xb66d6ddb, first_difficulty_flag: 0x6d6d6d6d},
        ghost_data_movement_bit_pattern: GhostSpeedBitPatterns { normal_state: 0xd65aadb5, speed_state: 0x48242291, tunnel_state: 0x92249224 },
        ghost_counter_for_orientation_change: [0x012c, 0x05dc, 0x0708, 0x0bb8, 0x0ce4, 0xfffe, 0xffff],
    },
];



// src:0843
/*
0: counter for pink
1: counter for blue
2: counter for orange
*/
const GHOST_DIFFICULTY_OUT_COUNTER: [ [u8; 3]; 4] = [
    [0x14, 0x1e, 0x46],
    [0x00, 0x1e, 0x3c],
    [0x00, 0x00, 0x32],
    [0x00, 0x00, 0x00],
];

// src:084f
/*
0: number of pills, difficulty flag #1
1: number of pills, difficulty flag #2
*/
const GHOST_DIFFICULTY_PILL_COUNTER: [ [u8; 2]; 9] = [
    [0x14, 0x0a],
    [0x1e, 0x0f],
    [0x28, 0x14],
    [0x32, 0x19],
    [0x3c, 0x1e],
    [0x50, 0x28],
    [0x64, 0x32],
    [0x78, 0x3c],
    [0x8c, 0x46],
];

// src:0861
// Time the ghosts stay blue when pacman eats a big pill
const GHOST_DIFFICULTY_BLUE_TIME: [u16; 9] = [
    960, // 8 seconds (not used)
    840, // 7 seconds (not used)
    720, // 6 seconds
    600, // 5 seconds
    480, // 4 seconds
    360, // 3 seconds
    240, // 2 seconds
    120, // 1 second
    1, // 0 seconds
];


// src:0873
// number of units before ghosts leaves home
const GHOST_DIFFICULTY_LEAVES_HOME_TIME: [u16; 3] = [
    240, // 2 seconds
    240, // 2 seconds
    180, // 1.5 seconds
];
