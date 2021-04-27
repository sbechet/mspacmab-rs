use crate::tile::TileId;

use crate::ghost_difficulty::GhostDifficulty;

#[derive(Copy, Clone)]
pub struct StatePlayer {
    // src:4e0a, src:4e38
    pub p_difficulty_settings: &'static [usize; 21],
    // src:4e0c, src:4e3a
    pub first_fruit_flag: bool,
    // src:4e0d, src:4e3b
    pub second_fruit_flag: bool,
    // src:4e0e, src:4e3c
    pub dots_eaten: u8,
    // src:4e0f, src:4e3d
    pub can_pink_ghost_leave_home: bool,
    // src:4e10, src:4e3e
    pub can_blue_ghost_leave_home: bool,
    // src:4e11, src:4e3f
    pub can_orange_ghost_leave_home: bool,
    // src:4e12, src:4e40
    pub dying_in_a_level: bool,
    // src:4e13, src:4e41
    pub level: usize,          // 0..
    // src:4e14, src:4e42
    pub real_number_of_lives: u8,
    // src:4e15, src:4e43
    pub number_of_lives_displayed: u8,
    // src:4e16, src:4e44
    pub is_pill_present: [u8; 30],    // maximum of 30*8 pellets, if bit=1 draw pill with TileId::Pill1 (16) - TODO: use bitvec crate?
    // src:4e34, src:4e62
    pub power_pills_data_entries: [TileId; 4],
}

impl StatePlayer {
    // src:26d0
    pub fn new(ghost_difficulty: &'static [usize; 21]) -> Self {
        StatePlayer {
            // src:272c
            p_difficulty_settings: ghost_difficulty,
            first_fruit_flag: false,
            second_fruit_flag: false,
            dots_eaten: 0,
            can_pink_ghost_leave_home: false,
            can_blue_ghost_leave_home: false,
            can_orange_ghost_leave_home: false,
            dying_in_a_level: false,
            level: 0,
            real_number_of_lives: 0,
            number_of_lives_displayed: 0,
            is_pill_present: [255; 30],
            power_pills_data_entries: [TileId::Pill5; 4],
        }
    }

 
}