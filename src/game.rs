use std::collections::VecDeque;
use num_traits::FromPrimitive;

use embedded_graphics::{
    prelude::*,
};

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
use crate::game_demo::GameDemo;
use crate::game_playing::GamePlaying;
use crate::game_task::{GameTask, TaskCoreE, ScreenPart};
use crate::game_task_timed::{GameTaskTimed, TaskTimedNameE, TaskTimedE};
use crate::mspacmab_data_maze::{ MAZE, PELLET, POWER_PILL};
use crate::mspacmab_data_fruit::{ FruitId, FRUIT};

pub enum MainStateE {
    Init=0,
    Demo=1,
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
    level: u8,          // 0..
    // src:4e14, src:4e42
    pub real_number_of_lives: u8,
    // src:4e15, src:4e43
    number_of_lives_displayed: u8,
    // src:4e16, src:4e44
    is_pill_present: [u8; 30],    // maximum of 30*8 pellets, if bit=1 draw pill with TileId::Pill1 (16)
    // src:4e34, src:4e62
    power_pills_data_entries: [TileId; 4],
}

pub struct Game {
    pub hwvideo: GameHwVideo,
    pub hwinput: HardwareInput,
    pub hwoutput: HardwareOutput,
    pub counter: Counter60Hz,

    // src:4c00
    // unknown: SpriteElement,
    red_ghost: SpriteElement,
    pink_ghost: SpriteElement,
    blue_ghost: SpriteElement,
    orange_ghost: SpriteElement,
    man: SpriteElement,
    fruit: SpriteElement,
    // unknown2: SpriteElement,

    red_ghost_current_tile: (i8, i8),
    pink_ghost_current_tile: (i8, i8),
    blue_ghost_current_tile: (i8, i8),
    orange_ghost_current_tile: (i8, i8),
    man_current_tile: (i8, i8),

    red_ghost_next_tile: (i8, i8),
    pink_ghost_next_tile: (i8, i8),
    blue_ghost_next_tile: (i8, i8),
    orange_ghost_next_tile: (i8, i8),
    man_next_tile: (i8, i8),

    red_ghost_move_direction: (i8, i8),
    red_ghost_face_direction: (i8, i8),

    pink_ghost_move_direction: (i8, i8),
    pink_ghost_face_direction: (i8, i8),

    blue_ghost_move_direction: (i8, i8),
    blue_ghost_face_direction: (i8, i8),

    orange_ghost_move_direction: (i8, i8),
    orange_ghost_face_direction: (i8, i8),

    man_move_direction: (i8, i8),
    man_wanted_direction: (i8, i8),

    red_ghost_dir: Direction,
    red_ghost_dir_face: Direction,

    pink_ghost_dir: Direction,
    pink_ghost_dir_face: Direction,

    blue_ghost_dir: Direction,
    blue_ghost_dir_face: Direction,

    orange_ghost_dir: Direction,
    orange_ghost_dir_face: Direction,

    man_orientation: Direction,
    wanted_man_orientation: Direction,

    // src:4c90
    pub timed_tasks: VecDeque<TaskTimedE>,

    // src:4dd1 FRUITP  fruit position
    pub killed_ghost_animation_state: i8,
    // src:4dd2 FVALUE  value of the current fruit (0=no fruit)
    pub fruit_coord: (i8, i8),
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
    pub subroutine_demo_state: u8,  // 0.. 16
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
    number_of_coins_per_credit: u8,
    // src:4e6c
    number_of_coins_inserted: u8,
    // src:4e6d
    number_of_credits_per_coin: Coinage,
    // src:4e6e
    number_of_credits: u8,
    // src:4e6f
    number_of_lives: u8,
    // src:4e70
    number_of_players: u8,      // 0=1 player, 1=2 players
    // src:4e71
    bonus: u8,             // (10, 15, 20, 255) * 1000
    // src:4e72
    cocktail_mode: bool,
    // src:4e73
    p_difficulty_settings: Vec<u8>,
    // src:4e75
    ghost_names_mode: bool,
    // src:4e80
    score_p1: u32,
    // src:4e83
    p1_got_bonus_life: bool,
    // src:4e84
    score_p2: u32,
    // src:4e87
    p2_got_bonus_life: bool,
    // src:4e88
    high_score: u32,

    // src:4ecc, src:4edc, src:4eec
    wave: [Wave; 3],

    // src:4f00
    pub intermission_mode: bool,

    // src:4f01
    pub flashing_bulbs_counter: u8,

    // src:4e8c, src:4e92, src:4e97
    // src:4e9c, src:4eac, src:4ebc
    // src:4ecc, src:4edc, src:4eec
    // src:5041, src:5046, src:504b
    pub hwsound: SoundChannels,

    pub task: GameTask,
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

            subroutine_demo_state: 0,
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

            red_ghost: SpriteElement::new_red_ghost(),
            pink_ghost: SpriteElement::new_pink_ghost(),
            blue_ghost: SpriteElement::new_blue_ghost(),
            orange_ghost: SpriteElement::new_orange_ghost(),
            man: SpriteElement::new_man(),
            fruit: SpriteElement::new_fruit(),

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
        
            timed_tasks: Self::timed_task_new(),

            killed_ghost_animation_state: 0,

            fruit_coord: (0,0),
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
            intermission_mode: false,

            flashing_bulbs_counter: 0,

            task: GameTask::new(),
        }
    }

    /// push on "real" hardware
    pub fn update(&mut self) {
        self.hwvideo.put_sprite(1,    self.red_ghost.p,    self.red_ghost.s,    self.red_ghost.c);
        self.hwvideo.put_sprite(2,   self.pink_ghost.p,   self.pink_ghost.s,   self.pink_ghost.c);
        self.hwvideo.put_sprite(3,   self.blue_ghost.p,   self.blue_ghost.s,   self.blue_ghost.c);
        self.hwvideo.put_sprite(4, self.orange_ghost.p, self.orange_ghost.s, self.orange_ghost.c);
        self.hwvideo.put_sprite(5,          self.man.p,          self.man.s,          self.man.c);
        self.hwvideo.put_sprite(6,        self.fruit.p,        self.fruit.s,        self.fruit.c);
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
                    self.task.add_to_task_list(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::All));
                    self.task.add_to_task_list(TaskCoreE::ClearColorRam);
                    self.task.add_to_task_list(TaskCoreE::SelectMazeColor(0));
                    self.task.add_to_task_list(TaskCoreE::SetupConfigFromDipSwitches);
                    self.task.add_to_task_list(TaskCoreE::ResetThenPrintPlayersScore);
                    self.task.add_to_task_list(TaskCoreE::ResetSpritesToDefaultValues(true));
                    self.task.add_to_task_list(TaskCoreE::ClearFruitAndPacmanPosition);
                    self.task.add_to_task_list(TaskCoreE::SetGameToDemoMode);
                    self.subroutine_init_state = 1;
                    self.hwoutput.sound_enabled = true;
                }
            },
            MainStateE::Demo => {
                /* src:03fe */
                println!("change_mode/Demo");
                self.t1d_draw_credit_qty();
                if self.number_of_credits != 0 {
                    self.mode = MainStateE::CoinInserted;   // +=1
                    self.subroutine_demo_state = 0;
                    self.subroutine_playing_state = 0;
                } else {
                    self.execute_demo_task_state_patch();
                }
            },
            MainStateE::CoinInserted => {
                /* src:05e5 */
                println!("change_mode/CoinInserted/{}", self.subroutine_coin_inserted_state);
                match self.subroutine_coin_inserted_state {
                    0 => {
                        // src:05f3
                        self.t1d_draw_credit_qty();
                        self.task.add_to_task_list(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                        self.task.add_to_task_list(TaskCoreE::SelectMazeColor(0));
                        self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::PushStartButton, false));
                        // Midway logo and copyright text
                        self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::AdditionalAt000Pts, false));
                        self.task.add_to_task_list(TaskCoreE::ClearFruitAndPacmanPosition);
                        self.subroutine_coin_inserted_state += 1;
                        self.led_state = true;
                        if self.bonus != 255 {
                            self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::TileMsPacMan, false));
                            self.task.add_to_task_list(TaskCoreE::DrawExtraLifePoints);
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
                        self.task.add_to_task_list(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                        self.task.add_to_task_list(TaskCoreE::SelectMazeColor(1));
                        self.task.add_to_task_list(TaskCoreE::DrawMaze);
                        self.task.add_to_task_list(TaskCoreE::ClearsPillsAndPowerPills);
                        self.task.add_to_task_list(TaskCoreE::DrawPellets);
                        self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::PlayerOne, false));
                        self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::Ready, false));
                        self.task.add_to_task_list(TaskCoreE::ResetThenPrintPlayersScore);
                        self.task.add_to_task_list(TaskCoreE::DrawFruitsBottomRightScreen);
                        self.current_player.level = 0;
                        self.current_player.real_number_of_lives = self.number_of_lives;
                        self.current_player.number_of_lives_displayed = self.number_of_lives;
                        self.task.add_to_task_list(TaskCoreE::DrawRemainingLivesBottomLeftScreen);
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
                        self.subroutine_demo_state = 0;
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
        //     if intermission_mode {
        //         copy sprites informations to intermisson buffer
        //     }
        //     ..


    }

    // src:238d
    pub fn idle(&mut self) -> bool {
        // println!("idle");
        match self.task.get_from_task_list() {
            Some(action) => {
                match action {
                    // 0
                    TaskCoreE::ClearWholeScreenOrMaze(part) => {
                        println!("TaskCoreE::ClearWholeScreenOrMaze");
                        self.clear_whole_screen_or_maze(part);
                    },
                    // 1 src:24d7
                    TaskCoreE::SelectMazeColor(playing_state) => {      // 0: default maze color, 1: ?, 2: flashing maze color (White)
                        println!("TaskCoreE::SelectMazeColor");
                        const COLOR_PALETTE_TABLE_FOR_MAZES: [ColorE; 21] = [
                            ColorE::ColorMazeLevel1_2_18_19_20_21,
                            ColorE::ColorMazeLevel1_2_18_19_20_21,
                            ColorE::ColorMazeLevel3_4_5,
                            ColorE::ColorMazeLevel3_4_5,
                            ColorE::ColorMazeLevel3_4_5,
                            ColorE::ColorMazeLevel6_7_8_9,
                            ColorE::ColorMazeLevel6_7_8_9,
                            ColorE::ColorMazeLevel6_7_8_9,
                            ColorE::ColorMazeLevel6_7_8_9,
                            ColorE::Orange,
                            ColorE::Orange,
                            ColorE::Orange,
                            ColorE::Orange,
                            ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor,
                            ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor,
                            ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor,
                            ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor,
                            ColorE::ColorMazeLevel1_2_18_19_20_21,
                            ColorE::ColorMazeLevel1_2_18_19_20_21,
                            ColorE::ColorMazeLevel1_2_18_19_20_21,
                            ColorE::ColorMazeLevel1_2_18_19_20_21,
                        ];

                        /* controls the color of the mazes */
                        let color = match playing_state {
                            2 => ColorE::White, // white color for flashing at end of level
                            _ => match self.subroutine_demo_state {
                                0 | 16 => {
                                    let mut n = self.current_player.level as usize;
                                    while n > 20 {
                                        n = n - (21-5);     // 5 <= n <= 20
                                    }
                                    COLOR_PALETTE_TABLE_FOR_MAZES[n]
                                },
                                _ => ColorE::Red,   // used to properly color the midway logo
                            },
                        };

                        // src:24e1
                        for y in 2..=33 {
                            // for x in 12..=27 {   // BUG? Only 1/2 screen... can't understand original binary code
                            for x in 0..=27 {
                                self.hwvideo.put_screen_color(Point::new(x, y), color);
                            }
                        }

                        // for x in 16..=27 {
                        //     self.hwvideo.put_screen_color(Point::new(x, 0), ColorE::Black); // HACK: Original was = 0x40 (for "tunnel slowdown"?)
                        // }

                        if playing_state==1 {
                            // src:95c3
                            /* 
                                HACK: replaced with is_tunnel_slowdown() fn: original method was too near hardware.

                                original code sets bit 6 in the color grid of certain screen locations on the first three levels.
                                This color bit is ignored when actually coloring the grid, so it is invisible onscreen.
                                When a ghost encounters one of these specially painted areas, he slows down.
                                This is used to slow down the ghosts when they use the tunnels on these levels.

                            */

                            // store into ghost house door (right side)
                            self.hwvideo.put_screen_color(Point::new(14, 15), ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor);
                            // store into ghost house door (left side)
                            self.hwvideo.put_screen_color(Point::new(13, 15), ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor);
                        }

                    },
                    //  2 src:2419
                    TaskCoreE::DrawMaze => {
                        println!("TaskCoreE::DrawMaze");
                        // Hack original data was too near hardware
                        let maze = *self.get_current_maze_table();  // TODO: not sure about my rust here...?
                        for x in 0..28 {
                            for y in 0..36 {
                                let tile_id = maze[y as usize][x as usize].clone();
                                let tile = TileId::from_u8(tile_id).unwrap();
                                self.hwvideo.put_screen_tile(Point::new(x,y), tile);
                            }
                        }

                    },
                    //  3 src:2448
                    TaskCoreE::DrawPellets => {
                        let pellet: &[ (u8,u8); 240 ] = self.get_data_from_current_level(&PELLET);
                        self.t03_pellets_draw(pellet);
                    },
                    //  4 src:253d
                    TaskCoreE::ResetSpritesToDefaultValues(start) => {
                        println!("TaskCoreE::ResetSpritesToDefaultValues");
                        self.red_ghost.s = SpriteId::GhostRight1;
                        self.pink_ghost.s = SpriteId::GhostRight1;
                        self.blue_ghost.s = SpriteId::GhostRight1;
                        self.orange_ghost.s = SpriteId::GhostRight1;
                        self.man.s = SpriteId::ManStart;
                        self.fruit.s = SpriteId::FruitStart;

                        self.red_ghost.c = ColorE::Red;
                        self.pink_ghost.c =  ColorE::Pink;
                        self.blue_ghost.c = ColorE::Blue;
                        self.orange_ghost.c = ColorE::Orange;
                        self.man.c = ColorE::Yellow;
                        self.fruit.c = ColorE::Black;

                        if start {
                            // src:2576
                            self.red_ghost.p = Point::new(128,100);
                            self.pink_ghost.p = Point::new(128,124);
                            self.blue_ghost.p = Point::new(144,124);
                            self.orange_ghost.p = Point::new(112,124);
                            self.man.p = Point::new(128,196);

                            // Next Tile XY Position (middle of tile)
                            // Current Tile XY (these are updated after a move)
                            // 29 == wraparound -> 61. 
                            // 62 == wraparound -> 30.
                            // y = bottom to top = decreases (34..62) len=28
                            // x = left to right = decreases (30..61) len=31
                            self.red_ghost_next_tile = (46,44);
                            self.red_ghost_current_tile = (46,44);

                            self.pink_ghost_next_tile = (46,47);
                            self.pink_ghost_current_tile = (46,47);

                            self.blue_ghost_next_tile = (48,47);
                            self.blue_ghost_current_tile = (48,47);

                            self.orange_ghost_next_tile = (44,47);
                            self.orange_ghost_current_tile = (44,47);

                            self.man_next_tile = (46,56);
                            self.man_current_tile = (46,56);

                            // TODO: Inverses all values here!

                            // left, left_right
                            self.red_ghost_move_direction = (1, 0);
                            self.red_ghost_face_direction = (1, 0);

                            // up_down, down
                            self.pink_ghost_move_direction = (0, 1);
                            self.pink_ghost_face_direction = (0, 1);

                            // up_down, up
                            self.blue_ghost_move_direction = (0, -1);
                            self.blue_ghost_face_direction = (0, -1);

                            // up_down, up
                            self.orange_ghost_move_direction = (0, -1);
                            self.orange_ghost_face_direction = (0, -1);

                            // left, left_right
                            self.man_move_direction = (1, 0);
                            self.man_wanted_direction = (1, 0);

                            self.red_ghost_dir = Direction::Down;
                            self.red_ghost_dir_face = Direction::Down;

                            self.pink_ghost_dir = Direction::Left;
                            self.pink_ghost_dir_face = Direction::Left;

                            self.blue_ghost_dir = Direction::Up;
                            self.blue_ghost_dir_face = Direction::Up;

                            self.orange_ghost_dir = Direction::Up;
                            self.orange_ghost_dir_face = Direction::Up;

                            self.man_orientation = Direction::Left;
                            self.wanted_man_orientation = Direction::Left;

                            self.fruit_coord = (0,0);
                        } else {
                            //  sets up sprites for character introduction screen
                            // src:260f

                            // sprites_coord_yx
                            self.red_ghost.p = Point::new(0,148);
                            self.pink_ghost.p = Point::new(0,148);
                            self.blue_ghost.p = Point::new(0,148);
                            self.orange_ghost.p = Point::new(0,148);

                            // sprites_coord_middle_of_tile
                            self.red_ghost_next_tile = ( 30, 50);
                            self.pink_ghost_next_tile = ( 30, 50);
                            self.blue_ghost_next_tile = ( 30, 50);
                            self.orange_ghost_next_tile = ( 30, 50);

                            // TODO: Inverses all values here!

                            // sprites_current_tile_xy
                            self.red_ghost_current_tile = ( 30, 50);
                            self.pink_ghost_current_tile = ( 30, 50);
                            self.blue_ghost_current_tile = ( 30, 50);
                            self.orange_ghost_current_tile = ( 30, 50);

                            // sprites_move_xy_direction
                            // down, up_down
                            self.red_ghost_move_direction = (1, 0);
                            self.pink_ghost_move_direction = (1, 0);
                            self.blue_ghost_move_direction = (1, 0);
                            self.orange_ghost_move_direction = (1, 0);

                            // sprites_face_xy_direction
                            self.red_ghost_face_direction = (1, 0);
                            self.pink_ghost_face_direction = (1, 0);
                            self.blue_ghost_face_direction = (1, 0);
                            self.orange_ghost_face_direction = (1, 0);

                            self.man_move_direction = (1, 0);
                            self.man_wanted_direction = (1, 0);

                            // src:2661 (9 next lines)
                            // sprites_ghosts_previous_orientation
                            self.red_ghost_dir = Direction::Left;
                            self.pink_ghost_dir = Direction::Left;
                            self.blue_ghost_dir = Direction::Left;
                            self.orange_ghost_dir = Direction::Left;

                            // sprites_ghosts_face_enum_direction
                            self.red_ghost_dir_face = Direction::Left;
                            self.pink_ghost_dir_face = Direction::Left;
                            self.blue_ghost_dir_face = Direction::Left;
                            self.orange_ghost_dir_face = Direction::Left;

                            // man_orientation
                            self.man_orientation = Direction::Left;
                            self.wanted_man_orientation = Direction::Left;

                            // pacman_tile_pos_in_demo_and_cut_scenes
                            self.man_next_tile = (31, 50);

                            // pacman_position_tile_position
                            self.man_current_tile = (31, 50);
                        }

                    },
                    // 6 src:240d
                    TaskCoreE::ClearColorRam => {
                        println!("TaskCoreE::ClearColorRam");
                        self.clear_color_ram();
                    },
                    // 7 src:2698
                    TaskCoreE::SetGameToDemoMode => {
                        println!("TaskCoreE::SetGameToDemoMode");
                        self.mode = MainStateE::Demo;
                        self.subroutine_init_state = 0;
                    },
                    // 18 src:24c9
                    TaskCoreE::ClearsPillsAndPowerPills => {
                        println!("TaskCoreE::ClearsPillsAndPowerPills");
                        self.clears_all_pills();
                    }
                    // 20 src:26d0 (void)
                    TaskCoreE::SetupConfigFromDipSwitches => {
                        println!("TaskCoreE::SetupConfigFromDipSwitches");
                        self.setup_config_from_dip_switches();
                    },
                    // 24 src:2ae0 (void)
                    TaskCoreE::ResetThenPrintPlayersScore => {
                        println!("TaskCoreE::ResetThenPrintPlayersScore");
                        self.hwvideo.put_text(TextId::HighScore);

                        self.score_p1 = 0;
                        self.p1_got_bonus_life = false;

                        self.score_p2 = 0;
                        self.p2_got_bonus_life = false;

                        self.draw_score_to_screen(self.score_p1, ( 1, 1) );
                        if self.number_of_players != 0 {
                            self.draw_score_to_screen(self.score_p2, (20, 1) );
                        }
                    },
                    // 26 src:2b6a
                    TaskCoreE::DrawRemainingLivesBottomLeftScreen => {
                        println!("TaskCoreE::DrawRemainingLivesBottomLeftScreen");
                        self.t1a_draw_remaining_lives_bottom_screen();
                    },
                    // 27 src:2bea
                    TaskCoreE::DrawFruitsBottomRightScreen => {
                        println!("TaskCoreE::DrawFruitsBottomRightScreen");
                        return self.draw_fruits_bottom_right_screen();
                    },
                    // 28 src:95e3 (TextId textid, bool clear)
                    TaskCoreE::DrawTextOrGraphics(textid, clear) => {
                        println!("TaskCoreE::DrawTextOrGraphics");
                        let mut final_textid = textid;

                        match final_textid {
                            TextId::TileMsPacMan => {
                                // src:960B
                                // Yes, draw the MS PAC MAN graphic which appears between "ADDITIONAL" and "AT 10,000 pts"
                                // src:9627 src:9616 (mspac_graph)
                                self.hwvideo.put_screen( Point::new(13,23), TileId::MspacBigUpperLeft,  ColorE::Yellow);
                                self.hwvideo.put_screen( Point::new(14,23), TileId::MspacBigUpperRight, ColorE::Yellow);
                                self.hwvideo.put_screen( Point::new(14,24), TileId::MspacBigLowerRight, ColorE::Yellow);
                                self.hwvideo.put_screen( Point::new(13,24), TileId::MspacBigLowerLeft,  ColorE::Yellow);
                            },
                            TextId::AdditionalAt000Pts => {
                                // src:95f6
                                self.draw_the_midway_logo_and_copyright();
                                // src:95fd
                                if let Bonus::None = self.hwinput.bonus {
                                    final_textid = TextId::Space4;
                                }
                            },
                            TextId::Ready => {
                                // 963c
                                // clear the intermission indicator
                                self.intermission_mode = false;
                            },
                            _ => {},
                        }

                        if clear {
                            self.hwvideo.del_text(final_textid);
                        } else {
                            self.hwvideo.put_text(final_textid);
                        }

                    },
                    // 30 src:2675 (void)
                    TaskCoreE::ClearFruitAndPacmanPosition => {
                        println!("TaskCoreE::ClearFruitAndPacmanPosition");
                        self.clear_fruit_and_pacman_position();
                    },
                    // 31 src:26b2
                    TaskCoreE::DrawExtraLifePoints => {
                        println!("TaskCoreE::DrawExtraLifePoints");
                        let t0 = TileId::from_u8(b'0' + self.bonus / 10).unwrap();
                        let t1 = TileId::from_u8(b'0' + self.bonus % 10).unwrap();
                        self.hwvideo.put_screen_tile(Point::new(19,24), t0);
                        self.hwvideo.put_screen_tile(Point::new(20,24), t1);
                    }
                    _ => {
                    },
                }
            },
            None    => {
                // println!("NOP");
                return false;
            },
        }
        // update screen
        true
    }

    // src:23ed
    fn clear_whole_screen_or_maze(&mut self, part:ScreenPart) {
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
    fn clear_color_ram(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.hwvideo.put_screen_color(Point::new(x as i32,y as i32), ColorE::Black);
            }
        }
    }

    // src:2453
    fn t03_pellets_draw(&mut self, pellets_coord: &[ (u8,u8); 240 ]) {
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
    fn clears_all_pills(&mut self) {
        self.current_player.is_pill_present = [255; 30];    // 1 = true
        self.current_player.power_pills_data_entries = [TileId::Pill5; 4];
    }

    // src:2675 (void)
    fn clear_fruit_and_pacman_position(&mut self) {
        self.fruit_coord = (0, 0);
        // pacman_coord
        self.man.p = Point::new(0,0);
        self.clear_all_ghosts_from_screen();
    }

    // src: 267e
    fn clear_all_ghosts_from_screen(&mut self) {
        // sprites_coord_yx
        self.red_ghost.p = Point::new(0,0);
        self.pink_ghost.p = Point::new(0,0);
        self.blue_ghost.p = Point::new(0,0);
        self.orange_ghost.p = Point::new(0,0);
    }

    // src:26d0
    fn setup_config_from_dip_switches(&mut self) {
        self.number_of_credits_per_coin = self.hwinput.coinage;
        if let Coinage::FreePlay = self.number_of_credits_per_coin {
            self.number_of_credits = 255;
        }
        /*
        number_of_coins_per_credit => number_of_credits_per_coin
                                    0 => 0
                                    1 => 1
                                    1 => 2
                                    2 => 1
            Historical : 
                number_of_coins_per_credit = (number_of_credits_per_coin >> 1) + (number_of_credits_per_coin & 1);
                number_of_credits_per_coin = number_of_coins_per_credit & 2 ^ number_of_credits_per_coin;
        */
        self.number_of_coins_per_credit = match self.number_of_credits_per_coin {
            Coinage::FreePlay => 0,
            Coinage::For1coin1credit => 1,
            Coinage::For1coin2credits => 2,
            Coinage::For2coins1credit => 1,
        };

        /* check dip switches 2 and 3.  number of starting lives per game */
        self.number_of_lives = self.hwinput.live as u8 + 1;
        if self.number_of_lives == 4 {
            self.number_of_lives += 2;
        }

        /* check dip switches 4 and 5.  points for bonus pac man */
        /* CONFIG for extra life
            10 for 10.000 pts
            15 for 15.0000 pts
            20 for 20_0000 pts
            FF for no extra life
        */
        self.bonus = match self.hwinput.bonus {
            Bonus::Pts10000 => 10,
            Bonus::Pts15000 => 15,
            Bonus::Pts20000 => 20,
            Bonus::None     => 255,
        };

        /* check dip switch 7 for ghost names during attract mode */
        self.ghost_names_mode = self.hwinput.change_ghost_names;

        /* check dip switch 6 for difficulty */
        self.p_difficulty_settings = if self.hwinput.hard_game {
            // hard
            vec![  1,  3,4,  6,7,8,9,10,11,12,13,14,15,16,17,      20]
        } else {
            // normal
            vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]
        };

        /* check bit 7 on IN1 for upright / cocktail */
        self.cocktail_mode = self.hwinput.cocktail_cabinet;
    }


    // src:2abe
    fn draw_score_to_screen(&mut self, score:u32, p:(u8, u8) ) {
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
    fn t1a_draw_remaining_lives_bottom_screen(&mut self) {
        if let MainStateE::Demo = self.mode {
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
    fn draw_fruits_bottom_right_screen(&mut self) -> bool {
        if let MainStateE::Demo = self.mode {
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
    fn get_current_maze_table(&mut self) -> &[ [u8; 28]; 36 ] {
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
    fn get_data_from_current_level<'b, T>(&self, data_lookup_table: &'b [ &T; 4 ]) -> &'b T {
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