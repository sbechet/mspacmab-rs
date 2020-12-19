use std::collections::VecDeque;
use num_traits::FromPrimitive;

use embedded_graphics::prelude::*;

use embedded_graphics_simulator::{
    Window, 
};

use crate::hardware::{ HardwareInput, HardwareOutput, Coinage, Bonus };
use crate::game_hw_video::{ GameHwVideo, SpriteElement };
use crate::game_hw_sound::{ SoundChannels, Wave };
// use crate::test_mode::{ test_mode };
use crate::game_counter::{ Counter60Hz, CurrentTime };
use crate::palette::{PALETTE, ColorE};
use crate::tile::{TileId, Tile};
use crate::sprite::{SpriteId, Sprite};
use crate::text::{TextId, Text};
use crate::game_animation::{ GameAnimationT, Instruction, NO_DATA, NO_SPRITES };
use crate::game_attract::GameAttract;
use crate::game_playing::GamePlaying;
use crate::game_task::{GameTask, TaskCoreE, ScreenPart};
use crate::game_task_timed::{GameTaskTimed, TaskTimedNameE, TaskTimedE};
use crate::mspacmab_data_maze::{ MAZE, PELLET, POWER_PILL};
use crate::mspacmab_data_fruit::{ FruitId, FRUIT};


pub enum MainStateE {
    Init=0,
    Attract=1,
    CoinInserted=2,
    Playing=3,
}

pub enum Direction {
    Right=0,
    Down=1,
    Left=2,
    Up=3,
}

pub const WIDTH: usize = 28;
pub const HEIGHT: usize = 36;

pub struct DataPlayer {
    // src:4e13, src:4e41
    pub level: u8,          // 0..
    // src:4e14, src:4e42
    pub real_number_of_lives: u8,
    // src:4e15, src:4e43
    number_of_lives_displayed: u8,
    // src:4e16, src:4e44
    is_pill_present: [u8; 30],    // maximum of 30*8 pellets, if bit=1 draw pill with TileId::Pill1 (16)
    // src:4e34, src:4e62
    power_pills_data_entries: [TileId; 4],
}

pub enum SpriteName {
    Red=0,
    Pink=1,
    Blue=2,
    Orange=3,
    Man=4,
    Fruit=5,
}

pub struct Game {
    pub hwvideo: GameHwVideo,
    pub hwinput: HardwareInput,
    pub hwoutput: HardwareOutput,
    pub counter: Counter60Hz,

    // src:4c00 src:4d00 for first five
    // src:4dd2 for last one (0 = no fruit. TODO: Use Option Here?)
    pub sprite: [SpriteElement; 6],

    pub red_ghost_current_tile: (i8, i8),
    pub pink_ghost_current_tile: (i8, i8),
    pub blue_ghost_current_tile: (i8, i8),
    pub orange_ghost_current_tile: (i8, i8),
    pub man_current_tile: (i8, i8),

    pub red_ghost_next_tile: (i8, i8),
    pub pink_ghost_next_tile: (i8, i8),
    pub blue_ghost_next_tile: (i8, i8),
    pub orange_ghost_next_tile: (i8, i8),
    pub man_next_tile: (i8, i8),

    pub red_ghost_move_direction: (i8, i8),
    pub red_ghost_face_direction: (i8, i8),

    pub pink_ghost_move_direction: (i8, i8),
    pub pink_ghost_face_direction: (i8, i8),

    pub blue_ghost_move_direction: (i8, i8),
    pub blue_ghost_face_direction: (i8, i8),

    pub orange_ghost_move_direction: (i8, i8),
    pub orange_ghost_face_direction: (i8, i8),

    pub man_move_direction: (i8, i8),
    pub man_wanted_direction: (i8, i8),

    pub red_ghost_dir: Direction,
    pub red_ghost_dir_face: Direction,

    pub pink_ghost_dir: Direction,
    pub pink_ghost_dir_face: Direction,

    pub blue_ghost_dir: Direction,
    pub blue_ghost_dir_face: Direction,

    pub orange_ghost_dir: Direction,
    pub orange_ghost_dir_face: Direction,

    pub man_orientation: Direction,
    pub wanted_man_orientation: Direction,

    /* src:4cc0 src:4c80 src:4c82 */
    pub tasks: VecDeque<TaskCoreE>,

    // src:4c90
    pub timed_tasks: VecDeque<TaskTimedE>,

    // src:4da4
    pub number_of_ghost_killed_but_no_collision_for_yet: u8,
    // src:4da5
    pub ghost_eat_ability: bool,

    // src:4dd1 FRUITP  fruit position
    pub killed_ghost_animation_state: i8,
    // src:4dd4
    pub fruit_points: i8,
    // src:4dd5 emtpy
    // src:4dd6
    led_state: bool,
    // src:[4dd7..=4dff] empty bytes
    // src:4e00
    pub mode: MainStateE,
    // src:4e01
    pub subroutine_init_state: u8,
    // src:4e02
    pub subroutine_attract_state: u8,  // 0.. 16
    // src:4e03
    pub subroutine_coin_inserted_state: u8,
    // src:4e04
    pub subroutine_playing_state: u8,   // 0x0E = end of level
    // src:4e05
    // emtpy byte
    // src:4e06
    pub state_in_first_cutscene: u8,
    // src:4e07
    pub state_in_second_cutscene: u8,
    // src:4e08
    pub state_in_third_cutscene: u8,

    // src:4e0a
    pub current_player: DataPlayer,
    // src:4e38
    pub backup_player: DataPlayer,

    // src:4e6b
    pub number_of_coins_per_credit: u8,
    // src:4e6c
    number_of_coins_inserted: u8,
    // src:4e6d
    pub number_of_credits_per_coin: Coinage,
    // src:4e6e
    pub number_of_credits: u8,
    // src:4e6f
    pub number_of_lives: u8,
    // src:4e70
    pub number_of_players: u8,      // 0=1 player, 1=2 players
    // src:4e71
    pub bonus: u8,             // (10, 15, 20, 255) * 1000
    // src:4e72
    pub cocktail_mode: bool,
    // src:4e73
    pub p_difficulty_settings: Vec<u8>,
    // src:4e75
    pub ghost_names_mode: bool,
    // src:4e80
    pub score_p1: u32,
    // src:4e83
    pub p1_got_bonus_life: bool,
    // src:4e84
    pub score_p2: u32,
    // src:4e87
    pub p2_got_bonus_life: bool,
    // src:4e88
    high_score: u32,

    // src:4ecc, src:4edc, src:4eec
    wave: [Wave; 3],

    // src:4f00
    // pub animation: GameAnimationT,
    pub animation_enable: bool,

    // src:4f01
    pub flashing_bulbs_counter: u8,

    // src:4f02
    pub animation_current: [ (&'static [Instruction], usize) ; 6],
    // src:4f0f
    pub animation_cmd_table_sprite_index: [ u8; 8],
    // src:4f17
    pub animation_cmd_table_delay: [ u8; 8],
    // src:4f1f
    pub animation_cmd_table_stop:  [bool; 6],
    // src:4f2e
    pub animation_cmd_table_f0_loop: [ (i8,i8); 6],
    // src:4f3e
    pub animation_cmd_table_sprite: [ &'static [SpriteId]; 8],

    // src:4e8c, src:4e92, src:4e97
    // src:4e9c, src:4eac, src:4ebc
    // src:4ecc, src:4edc, src:4eec
    // src:5041, src:5046, src:504b
    pub hwsound: SoundChannels,
}

impl Game {

    pub fn new() -> Self {
        Game {
            hwvideo: GameHwVideo::new(),
            hwsound: SoundChannels::new(),
            hwinput: HardwareInput::new(),
            hwoutput: HardwareOutput::new(),
            counter: Counter60Hz::new(),
            led_state: false,
            mode: MainStateE::Init,                        // src:4e00
            subroutine_init_state: 0,                      // src:4e01

            subroutine_attract_state: 0,
            subroutine_coin_inserted_state: 0,
            subroutine_playing_state: 0,
            
            number_of_coins_per_credit: 0,
            number_of_coins_inserted: 0,
            number_of_credits_per_coin: Coinage::FreePlay,
            number_of_credits: 0,
            number_of_lives: 0,                            // src:4e6f
            number_of_players: 0,

            bonus: 10,
            ghost_names_mode: false,
            p_difficulty_settings: vec![0; 20],
            cocktail_mode: false,

            score_p1: 0,
            p1_got_bonus_life: false,
            score_p2: 0,
            p2_got_bonus_life: false,
            high_score: 0,

            sprite: [
                SpriteElement::new_red_ghost(),
                SpriteElement::new_pink_ghost(),
                SpriteElement::new_blue_ghost(),
                SpriteElement::new_orange_ghost(),
                SpriteElement::new_man(),
                SpriteElement::new_fruit(),
            ],

            red_ghost_current_tile: (0,0),
            pink_ghost_current_tile: (0,0),
            blue_ghost_current_tile: (0,0),
            orange_ghost_current_tile: (0,0),
            man_current_tile: (0,0),
        
            red_ghost_next_tile: (0,0),
            pink_ghost_next_tile: (0,0),
            blue_ghost_next_tile: (0,0),
            orange_ghost_next_tile: (0,0),
            man_next_tile: (0,0),
        
            red_ghost_move_direction: (0,0),
            red_ghost_face_direction: (0,0),
        
            pink_ghost_move_direction: (0,0),
            pink_ghost_face_direction: (0,0),
        
            blue_ghost_move_direction: (0,0),
            blue_ghost_face_direction: (0,0),
        
            orange_ghost_move_direction: (0,0),
            orange_ghost_face_direction: (0,0),
        
            man_move_direction: (0,0),
            man_wanted_direction: (0,0),
        
            red_ghost_dir: Direction::Right,
            red_ghost_dir_face: Direction::Right,
        
            pink_ghost_dir: Direction::Right,
            pink_ghost_dir_face: Direction::Right,
        
            blue_ghost_dir: Direction::Right,
            blue_ghost_dir_face: Direction::Right,
        
            orange_ghost_dir: Direction::Right,
            orange_ghost_dir_face: Direction::Right,
        
            man_orientation: Direction::Right,
            wanted_man_orientation: Direction::Right,
        
            tasks: VecDeque::new(),
            timed_tasks: Self::timed_task_new(),

            number_of_ghost_killed_but_no_collision_for_yet: 0,
            ghost_eat_ability: false,

            killed_ghost_animation_state: 0,

            fruit_points: 0,

            state_in_first_cutscene: 0,
            state_in_second_cutscene: 0,
            state_in_third_cutscene: 0,

            current_player: DataPlayer {
                level: 0,
                real_number_of_lives: 0,
                number_of_lives_displayed: 0,
                is_pill_present: [255; 30],
                power_pills_data_entries: [TileId::Pill5; 4],
            },

            backup_player: DataPlayer {
                level: 0,
                real_number_of_lives: 0,
                number_of_lives_displayed: 0,
                is_pill_present: [255; 30],
                power_pills_data_entries: [TileId::Pill5; 4],
            },

            wave: [Wave::new(); 3],

            // Is set to true during intermissions and parts of the attract mode, otherwise false
            animation_enable: false,

            flashing_bulbs_counter: 0,

            animation_current: [ (&NO_DATA,0); 6],
            animation_cmd_table_sprite_index: [ 0; 8],
            animation_cmd_table_delay: [ 0; 8],
            animation_cmd_table_stop:  [false; 6],
            animation_cmd_table_f0_loop: [ (0,0); 6],
            animation_cmd_table_sprite: [ &NO_SPRITES; 8],

        }
    }

    /// push on "real" hardware
    pub fn update(&mut self) {
        for i in 0..6 {
            self.hwvideo.put_sprite(i+1, self.sprite[i].p, self.sprite[i].s, self.sprite[i].c);
        }
        self.hwvideo.update();
    }

    /* MEMORY_MAP: program_rom1 **********************************************/

    /* src:03c8 */
    fn change_mode(&mut self) {
        match self.mode {
            MainStateE::Init => {
                if self.subroutine_init_state == 0 {
                    /* src:03dc */
                    println!("change_mode/Init");
                    self.tasks.push_back(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::All));
                    self.tasks.push_back(TaskCoreE::ClearColorRam);
                    self.tasks.push_back(TaskCoreE::SelectMazeColor(0));
                    self.tasks.push_back(TaskCoreE::SetupConfigFromDipSwitches);
                    self.tasks.push_back(TaskCoreE::ResetThenPrintPlayersScore);
                    self.tasks.push_back(TaskCoreE::ResetSpritesToDefaultValues(true));
                    self.tasks.push_back(TaskCoreE::ClearFruitAndPacmanPosition);
                    self.tasks.push_back(TaskCoreE::SetGameToAttractMode);
                    self.subroutine_init_state = 1;
                    self.hwoutput.sound_enabled = true;
                }
            },
            MainStateE::Attract => {
                /* src:03fe */
                println!("change_mode/Attract");
                self.t1d_draw_credit_qty();
                if self.number_of_credits != 0 {
                    self.mode = MainStateE::CoinInserted;   // +=1
                    self.subroutine_attract_state = 0;
                    self.subroutine_playing_state = 0;
                } else {
                    self.execute_attract_task_state_patch();
                }
            },
            MainStateE::CoinInserted => {
                /* src:05e5 */
                println!("change_mode/CoinInserted/{}", self.subroutine_coin_inserted_state);
                match self.subroutine_coin_inserted_state {
                    0 => {
                        // src:05f3
                        self.t1d_draw_credit_qty();
                        self.tasks.push_back(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                        self.tasks.push_back(TaskCoreE::SelectMazeColor(0));
                        self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::PushStartButton, false));
                        // Midway logo and copyright text
                        self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::AdditionalAt000Pts, false));
                        self.tasks.push_back(TaskCoreE::ClearFruitAndPacmanPosition);
                        self.subroutine_coin_inserted_state += 1;
                        self.led_state = true;
                        if self.bonus != 255 {
                            self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::TileMsPacMan, false));
                            self.tasks.push_back(TaskCoreE::DrawExtraLifePoints);
                        }
                    },
                    1 => {
                        // src:061b
                        // display credits qty
                        self.t1d_draw_credit_qty();
                        // display number of players using credits qty
                        let player_tile = if self.number_of_credits == 1 {
                            TextId::OnePlayerOnly
                        } else {
                            TextId::OneOrTwoPlayers
                        };
                        self.hwvideo.put_text(player_tile);

                        // player want to play?

                        if self.number_of_credits != 1 && self.hwinput.player2_start_button {
                            self.number_of_players = 1;
                        } else if self.hwinput.player1_start_button {
                            self.number_of_players = 0;
                        } else {
                            // no start button has been pressed
                            return;
                        }

                        // Check FreePlay
                        if self.number_of_coins_per_credit != 0 {
                            // two players
                            if self.number_of_players == 1 {
                                self.number_of_credits -= 1;
                            }
                            self.number_of_credits -= 1;
                            self.t1d_draw_credit_qty();
                        }
                
                        self.subroutine_coin_inserted_state += 1;
                        self.led_state = false;
                        self.wave[0].num = 1;
                        self.wave[1].num = 1;
                    },
                    2 => {
                        // src:0674
                        self.tasks.push_back(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                        self.tasks.push_back(TaskCoreE::SelectMazeColor(1));
                        self.tasks.push_back(TaskCoreE::DrawMaze);
                        self.tasks.push_back(TaskCoreE::ClearsPillsAndPowerPills);
                        self.tasks.push_back(TaskCoreE::DrawPellets);
                        self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::PlayerOne, false));
                        self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Ready, false));
                        self.tasks.push_back(TaskCoreE::ResetThenPrintPlayersScore);
                        self.tasks.push_back(TaskCoreE::DrawFruitsBottomRightScreen);
                        self.current_player.level = 0;
                        self.current_player.real_number_of_lives = self.number_of_lives;
                        self.current_player.number_of_lives_displayed = self.number_of_lives;
                        self.tasks.push_back(TaskCoreE::DrawRemainingLivesBottomLeftScreen);
                        self.timed_task_add(CurrentTime::LessTenth, 23, TaskTimedNameE::IncreaseSubroutineCoinInsertedState);
                    },
                    3 => {
                        // println!("MY CURRENT_HUMAN_REVERSE_POINTER / PC :)");
                        // PC
                        // src:000c
                        // RET
                    },
                    4 => {
                        // src:06a8
                        self.current_player.number_of_lives_displayed -= 1;
                        self.t1a_draw_remaining_lives_bottom_screen();
                        self.mode = MainStateE::Playing;
                        self.subroutine_attract_state = 0;
                        self.subroutine_coin_inserted_state = 0;
                        self.subroutine_playing_state = 0;
                    },
                    _ => {},
                }
            },
            MainStateE::Playing => {
                /* src:06be */
                println!("change_mode/Playing");
                self.execute_playing_task_state();
            },
        }
    }

    /*
        1. SOUND
            copy freqs to hardware (channel1, channel2, channel3)
            for each channel :
                hardware wave_t.sel configuration
                if channel_X_wave.num == 0:
                    wave_select_X = channel_X_effect.table[0]
        2. SPRITES PREPARE FOR HARDWARE
            // 4C02..4C20 TO 4C22..4C40
                all sprites data to hardware_prepare (*8):
                    sprite_flip_id_color_t[7] and sprites_coord_t[8]
            // 4C22..4C2C :
                red, pink, blue, orange, man, fruit : rotate to put xy on lower part?

        3. KILLED GHOST ANIMATION
            if (killed_ghost_animation_state == 1) {
                ... action on 4C22..4C2C = BUFF PREPARE FOR HARDWARE
            }
        4. POWER PILL ANIMATION
            if (power_pill_effect) {
                ... action on 4C22..4C2C = BUFF PREPARE FOR HARDWARE
            }
        
        5. SPRITES PREPARED TO HARDWARE
            // COPY 4C22..4C2E TO 4FF2..5000 (flipx, flipy, spriteid, palette)
            // COPY 4C32..4C40 TO 5060..5070 (x, y)

    */
    // src:
    pub fn timed_60_hz(&mut self) {
        // SoundChannels::channel[0].set_wave 

        // 6. VARIABLE THINGS
        self.counter.update();
        self.timed_task_execute();
        self.change_mode();
        match self.mode {
            MainStateE::Init => {
                // channel_2_effect.num = 0;
                // channel_3_effect.num = 0;
            },
            _ => {
                // check_for_double_size_pacman();
                // no_cocktail_mode_update_sprites();
                // cocktail_mode_update_sprites();
                // rack_input__add_credits();
                // debounce_coin_input__add_credits();
                // blink_coin_lights();
            }
        }

        // 7. SOUND GAME
        // process_effect_all_voices();
        // process_waves_all_voices();

        // 8. MSPACMAN intermission
        //     see backup_sprites__then__check_cocktail_animation_end();
        //     if animation_enable {
        //         copy sprites informations to intermisson buffer
        //     }
        //     ..


    }


    // src:23ed
    pub fn clear_whole_screen_or_maze(&mut self, part:ScreenPart) {
        match part {
            // src:23f3
            ScreenPart::All => {
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        self.hwvideo.put_screen_tile(Point::new(x as i32,y as i32), TileId::Space);
                    }
                }
            },
            // src:2400
            // clean:4040->423f - history: (27, 2) -> (12, 33)
            // calculated: (12, 2) -> (27, 33)
            ScreenPart::Maze => {
                // for x in 12..=27 {   // BUG? Only 1/2 screen... not sure about original ROM code
                for x in 0..=27 {
                    for y in 2..=33 {
                        self.hwvideo.put_screen_tile(Point::new(x as i32,y as i32), TileId::Space);
                    }
                }
            }
        }
    }

    // src:240d
    pub fn clear_color_ram(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.hwvideo.put_screen_color(Point::new(x as i32,y as i32), ColorE::Black);
            }
        }
    }

    // src:2453
    pub fn t03_pellets_draw(&mut self, pellets_coord: &[ (u8,u8); 240 ]) {
        let mut pellet_index = 0;
        for byte_index in 0..30 {
            let mut current_pills = self.current_player.is_pill_present[byte_index];
            for _bit_index in 0..8 {
                let p = pellets_coord[pellet_index];
                if current_pills >> 7 == 1 {
                    self.hwvideo.put_screen_tile(Point::new(p.0 as i32,p.1 as i32), TileId::Pill1);
                }
                current_pills <<= 1;
                pellet_index += 1;
            }
        }
        self.t03_power_pills_draw();
    }

    // src:24c9
    pub fn clears_all_pills(&mut self) {
        self.current_player.is_pill_present = [255; 30];    // 1 = true
        self.current_player.power_pills_data_entries = [TileId::Pill5; 4];
    }

    // src:2675 (void)
    pub fn clear_fruit_and_pacman_position(&mut self) {
        self.sprite[SpriteName::Fruit as usize].p = Point::new(0,0);
        self.sprite[SpriteName::Man as usize].p = Point::new(0,0);
        self.clear_all_ghosts_from_screen();
    }

    // src: 267e
    fn clear_all_ghosts_from_screen(&mut self) {
        self.sprite[SpriteName::Red as usize].p = Point::new(0,0);
        self.sprite[SpriteName::Pink as usize].p = Point::new(0,0);
        self.sprite[SpriteName::Blue as usize].p = Point::new(0,0);
        self.sprite[SpriteName::Orange as usize].p = Point::new(0,0);
    }

    // src:2abe
    pub fn draw_score_to_screen(&mut self, score:u32, p:(u8, u8) ) {
        // Printed text format must be '00' not '0' for 0
        let text = format!("{:5}{:1}", score / 10, score % 10);
        let textb = text.as_bytes();
        let mut x = p.0 as i32;
        let y = p.1 as i32;

        for c in textb {
            let p = Point::new(x,y);
            x += 1;
            let tileid = match c {
                b' ' => TileId::Space,
                _ => TileId::from_u8(*c).unwrap(),
            };
            self.hwvideo.put_screen_tile(p, tileid);
        }

    }

    // src:2b6a
    pub fn t1a_draw_remaining_lives_bottom_screen(&mut self) {
        if let MainStateE::Attract = self.mode {
            return;   // do not update screen
        }
        self.set_bottom_left_background_to_yellow();
        //
        let mut places = 5;
        let number_of_lives_displayed = self.current_player.number_of_lives_displayed;

        let mut x = 3;

        // first draw lives
        if number_of_lives_displayed != 0 && number_of_lives_displayed < 6 {
            for _i in 0..number_of_lives_displayed {
                // draw 16x16 Ms Pacman 
                self.draw_fruit_tile(TileId::MspacBigUpperRight, (x,34));
                x += 2;
                places -= 1;
            }
        }
        // clean last ones
        for _i in 0..places {
            self.draw_fruit_tile_blank( (x,34) );
            x += 2;
        }

    }

    // src:2b7e
    fn draw_fruit_tile_blank(&mut self, p: (i32, i32) ) {
        let t = TileId::Space;

        let point = Point::new(p.0, p.1);
        self.hwvideo.put_screen_tile(point, t);

        let point = Point::new(p.0 - 1, p.1);
        self.hwvideo.put_screen_tile(point, t);

        let point = Point::new(p.0, p.1 + 1);
        self.hwvideo.put_screen_tile(point, t);

        let point = Point::new(p.0 - 1, p.1 + 1);
        self.hwvideo.put_screen_tile(point, t);
    }

    // src:2b80
    fn draw_fruit_color(&mut self, c: ColorE, p: (i32, i32) ) {
        let point = Point::new(p.0, p.1);
        self.hwvideo.put_screen_color(point, c);

        let point = Point::new(p.0 - 1, p.1);
        self.hwvideo.put_screen_color(point, c);

        let point = Point::new(p.0, p.1 + 1);
        self.hwvideo.put_screen_color(point, c);

        let point = Point::new(p.0 - 1, p.1 + 1);
        self.hwvideo.put_screen_color(point, c);
    }
    
    // src:2b8f
    fn draw_fruit_tile(&mut self, t: TileId, p: (i32, i32) ) {
        let tnum = t as u8;

        let point = Point::new(p.0, p.1);
        self.hwvideo.put_screen_tile(point, t);

        let point = Point::new(p.0 - 1, p.1);
        let tnext = TileId::from_u8(tnum + 1).unwrap();
        self.hwvideo.put_screen_tile(point, tnext);

        let point = Point::new(p.0, p.1 + 1);
        let tnext = TileId::from_u8(tnum + 2).unwrap();
        self.hwvideo.put_screen_tile(point, tnext);

        let point = Point::new(p.0 - 1, p.1 + 1);
        let tnext = TileId::from_u8(tnum + 3).unwrap();
        self.hwvideo.put_screen_tile(point, tnext);
    }

    // src:2ba1
    fn t1d_draw_credit_qty(&mut self) {
        /* display number of credits */
        if self.number_of_credits == 255 {
            self.hwvideo.put_text(TextId::FreePlay);
        } else {
            self.hwvideo.put_text(TextId::Credit);
            let text = format!("{}", self.number_of_credits);
            let textb = text.as_bytes();
            let mut x = 9;
            let y = 35;
            for c in textb {
                let p = Point::new(x,y);
                x += 1;
                match num::FromPrimitive::from_u8(*c) {
                    Some(tile) => self.hwvideo.put_screen_tile(p, tile),
                    None => {},
                }
            }
        }
    }

    // src:2bcd
    fn set_bottom_left_background_to_yellow(&mut self) {
        for y in 0..2 {
            for x in 1..=11 {
                let p = Point::new(x, 34+y);
                self.hwvideo.put_screen_color(p, ColorE::Yellow);
            }
        }
    }

    // src:2bea
    pub fn draw_fruits_bottom_right_screen(&mut self) -> bool {
        if let MainStateE::Attract = self.mode {
            return false;   // do not update screen
        }
        let level:usize = if self.current_player.level > 7 {
            7
        } else {
            self.current_player.level as usize
        };

        let mut x=25;
        for i in 0..level {
            let f = FRUIT[i];
            self.draw_fruit_tile( f.0, (x, 34) );
            self.draw_fruit_color( f.1, (x, 34) );
            x -= 2;
        }
        for _i in level..7 {
            self.draw_fruit_tile_blank( (x, 34) );
            x -= 2;
        }

        return true;
    }

    /* MEMORY_MAP: /program_rom1 *********************************************/


    /* MEMORY_MAP: program_rom2 **********************************************/

    // src:946a
    pub fn get_current_maze_table(&mut self) -> &[ [u8; 28]; 36 ] {
        self.get_data_from_current_level(&MAZE)
    }

    // src:949c
    fn t03_power_pills_draw(&mut self) {
        let power_pill = self.get_data_from_current_level(&POWER_PILL);
        for i in 0..4 {
            let p = power_pill[i];
            let t = self.current_player.power_pills_data_entries[i];
            self.hwvideo.put_screen_tile(Point::new(p.0 as i32,p.1 as i32), t);
        }
    }
 
    /// map order table.  order that boards are played
    /// - 1st & 2nd boards use maze 1
    /// - 3rd, 4th, 5th boards use maze 2
    /// - boards 6 through 9 use maze 3
    /// - boards 10 through 13 use maze 4
    /// 
    // src:94bd
    pub fn get_data_from_current_level<'b, T>(&self, data_lookup_table: &'b [ &T; 4 ]) -> &'b T {
        const MAP_ORDER_TABLE:[u8; 13] = [0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];

        let mut n = self.current_player.level as usize;
        while n > 12 {
            n = n - (13-5);     // 5 <= n <= 12
        }

        let index = MAP_ORDER_TABLE[n];

        data_lookup_table[index as usize]
    }

    /// HACK: original version use colorram bit6 to configure tile like a slowdown tile for ghost
    /// see src:95c3
    fn is_tunnel_slowdown(&self, x: u8, y: u8) -> bool {
        match self.current_player.level {
            0 | 1 => {
                // slow down for maze on level 1 and 2 :
                // src: 8b3d
                let tunnel_slow_down_1_2 = [
                    ( 0,11), ( 1,11), ( 2,11),  // upper left
                    (24,11), (25,11), (26,11),  // upper right
                    ( 0,20), ( 1,20), ( 2,20),  // lower left
                    (24,20), (25,20), (26,20),  // lower right
                    ( 3, 2), (10, 2), (17, 2), (24, 2),  // TODO:original garbage?
                ];
                for e in &tunnel_slow_down_1_2 {
                    if *e == (x, y) {
                        return true;
                    }
                }
            },
            2 => {
                // slow down for maze on level 3
                // src: 8e28
                let tunnel_slow_down_3 = [
                    ( 0, 4), ( 1, 4), ( 2, 4), ( 3, 4), ( 4, 4), ( 5, 4), ( 6, 4),  // upper left
                    ( 0, 4), ( 1, 4), ( 2, 4), ( 3, 4), ( 4, 4), ( 5, 4), ( 6, 4),  // upper right
                    ( 0,26), ( 1,26), ( 2,26),  // lower left
                    (25,26), (26,26), (27,26),  // lower right
                    (14,2), (7,2)               // TODO:original garbage?
                ];
                for e in &tunnel_slow_down_3 {
                    if *e == (x, y) {
                        return true;
                    }
                }
            },
            _ => {
                // nop
                return false;
            }
        };

        return false;
    }

    /* MEMORY_MAP: /program_rom2 *********************************************/

    

    // pub fn hw_set_orientation(&self, left:bool, right: bool, up: bool, down: bool) {

    // }

    // pub fn hw_sound(&self, enable:bool) {

    // }


}