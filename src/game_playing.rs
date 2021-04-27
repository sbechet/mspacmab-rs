use embedded_graphics::geometry::Point;
use num_traits::FromPrimitive;


use crate::hardware::{ HardwareInput, HardwareOutput };
use crate::game_hw_video::{ GameHwVideo, SpriteElement };
use crate::game_hw_sound::{ SoundChannels, Wave };

use crate::tile::TileId;

use crate::game::MainStateE;
use crate::game_task::TaskCoreE;
use crate::text::TextId;
use crate::game_task::GameTask;
use crate::game_task_timed::{ GameTaskTimed, TaskTimedNameE };
use crate::game_counter::CurrentTime;
use crate::mspacmab_data_maze::{ MAZE, PELLET_TO_EAT, POWER_PILL };
use crate::palette::ColorE;
use crate::sprite::SpriteId;
use crate::state_player::StatePlayer;
use crate::ghost_difficulty::{ GhostDifficulty, ManSpeedBitPatterns, GhostSpeedBitPatterns };

enum ManSpeedBitPattern {
    NormalPillState = 0,
    BigPillState = 1,
    SecondDifficultyFlag = 2,
    FirstDifficultyFlag = 3,
}

#[derive(PartialEq, Copy, Clone)]
pub enum KillingGhostState {
    Nothing=0,
    KillRed=1,
    KillPink=2,
    KillBlue=3,
    KillOrange=4,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Right=0,
    Down=1,
    Left=2,
    Up=3,
}

// src:32ff
// use Direction enum as index
// TODO: check right and left
pub const DIRECTION_TILE_MOVEMENTS: [Point; 4] = [
    Point::new(-1,  0),   // right
    Point::new( 0,  1),   // down
    Point::new( 1,  0),   // left
    Point::new( 0, -1)    // up
];

#[derive(PartialEq, Copy, Clone)]
pub enum GhostState {
    Alive=0,
    Dead=1,
    EnteringHomeAfterBeingKilled=2,
    GoLeftAfterEnteringHomeAfterDead=3,
    GoRightAfterEnteringHomeAfterDead=4,
}

#[derive(PartialEq, Copy, Clone)]
pub enum GhostSubState {
    AtHome=0,
    GoingForMan=1,
    CrossingTheDoor=2,
    GoingToTheDoor=3,
}

#[derive(Copy, Clone)]
pub enum SpriteName {
    Red=0,
    Pink=1,
    Blue=2,
    Orange=3,
    Man=4,
    Fruit=5,
}

pub struct GamePlaying {
    // src:4c00 src:4d00 for first five
    // src:4dd2 for last one (0 = no fruit. TODO: Use Option Here?)
    // src:4c00
    pub sprite: [SpriteElement; 6],

    // src:4d00 (for first five) src:4dd2 for last one
    // (0 = no fruit. TODO: Use Option Here?)
    sprites_coord_xy: [Point; 6],
    // src:4d0a (next tile?)
    sprites_coord_middle_of_tile: [Point; 6],
    // src:4d14
    sprites_move_xy_direction: [Point; 6],
    // src:4d1e
    sprites_face_xy_direction: [Point; 6],
    // src:4d28
    sprites_ghosts_previous_orientation: [Direction; 4],
    // src:4d2c
    sprites_ghosts_face_enum_direction: [Direction; 4],
    // src:4d30
    man_orientation: Direction,
    // src:4d31 wanted
    /* 28x36

    (128, 100) <=> ( 46, 44)
    (112, 124) <=> ( 44, 47)
    (128, 124) <=> ( 46, 47)
    (144, 124) <=> ( 48, 47)
    (  0, 148) <=> ( 30, 50)
    (  8, 148) <=> ( 31, 50)
    (128, 196) <=> ( 46, 56)

    0x1d == wraparound -> 0x3d
    0x3e == wraparound -> 0x1e.
    x : 0x1e..0x3d 30..61 (d=31) left-right = decrease <-> 3d=left=0, 1e=right ?
    y : 0x22..0x3e 34..62 (d=28) bottom-top = decrease <-> 3e=top=0, 22=bottom
    */
    sprites_current_tile_xy: [Point; 6],

    /********************************************** path finding algorithm **/

    // src:4d3b
    ghost_best_orientation_found: Direction,
    // src:4d3c
    wanted_man_orientation: Direction,
    // src:4d3d
    opposite_orientation: Direction,
    // src:4d3e
    ghost_current_tile_position: Point,
    // src:4d40
    ghost_destination_tile_position: Point,
    // src:4d42
    ghost_temp_resulting_position: Point,
    // src:4d44
    minimum_distance_square_found: Point,

    // src:4d46
    // movement bit patterns (difficulty dependant)
    pub man_movement: ManSpeedBitPatterns,
    // src:4d56
    pub red_ghost_movement: GhostSpeedBitPatterns,
    // src:4d62
    pub pink_ghost_movement: GhostSpeedBitPatterns,
    // src:4d6e
    pub blue_ghost_movement: GhostSpeedBitPatterns,
    // src:4d7a
    pub orange_ghost_movement: GhostSpeedBitPatterns,

    /*
        Difficulty related table. Each entry is 2 bytes, and
        contains a counter value.  when the counter at 4DC2
        reaches each entry value, the ghosts changes their
        orientation and 4DC1 increments it's value to point to
        the next entry
    */
    // src:4d86
    pub ghost_counter_for_orientation_change: [u16; 7],
    // src:4d94
    pub counter_related_to_ghost_movement_inside_home: u8,
    // src:4d95
    pub number_of_units_before_ghost_leaves_home: u16,
    // src:4d97
    inactivity_counter_for_units_of_the_above: u16,

    /* These values are normally 0, but are changed to 1 when a ghost has entered a tunnel slowdown area */
    // src:4d99
    delay_ghost_movement: [u8; 4],
    // src:4d9d
    // eating pills : 1, eating big pills: 6, else -1
    delay_man_movement: i8,

    /******************************************** ghost substates if alive **/
    // src:4da0, src:4da1, src:4da2, src:4da3
    pub ghost_substate_if_alive: [GhostSubState; 4],

    // src:4da4
    pub number_of_ghost_killed_but_no_collision_for_yet: KillingGhostState,
    // src:4da5
    pub man_dead_animation_state: u8,
    // src:4da6
    power_pill_effect: bool,
    // src:4da7
    ghost_blue_flag: [bool; 4],

    // src:4dab
    killing_ghost_state: KillingGhostState,
    // src:4dac, src:4dad, src:4dae, src:4daf
    ghost_state: [GhostState; 4],

    // src:4db0
    // unused

    // src:4db1
    change_orientation_flag: [bool; 5],

    /*
        DIFFICULTY SETTINGS

        0: red ghost goes to upper right corner on scatter
        1: red ghost goes for pacman on scatter
        1: red ghost goes faster
        1 if 224 dots have been eaten and all 4 ghosts are free
    */
    // src:4db6
    red_ghost_first_difficulty_flag: bool,

    /*
        when set, red uses a faster bit speed pattern
        0: not set
        1: faster movement
    */
    // src:4db7
    red_ghost_second_difficulty_flag: bool,

    // src:4db8
    pub pink_ghost_counter_to_go_out_of_home_limit: u8,
    // src:4db9
    pub blue_ghost_counter_to_go_out_of_home_limit: u8,
    // src:4dba
    pub orange_ghost_counter_to_go_out_of_home_limit: u8,
    // src:4dbb
    pub red_ghost_remainder_of_pills_when_first_difficulty_flag_is_set: u8,
    // src:4dbc
    pub red_ghost_remainder_of_pills_when_second_difficulty_flag_is_set: u8,
    // src:4dbd
    pub time_the_ghosts_stay_blue_when_pacman_eats_a_big_pill: u16,
    // src:4dbf
    pacman_about_to_enter_a_tunnel: bool,

    /* COUNTERS */
    
    // used for ghost animations
    // src:4dc0
    counter__change_every_8_frames: u8,

    /*
        [0..7]. See ghost_counter_for_orientation_change
        0: random ghost movement, 1: normal movement (?)

        ghost reversal status (altered by timer at $4DC2/3
    */
    // src:4dc1
    counter__orientation_changes_index: u8,

    // chase frames since board/pac start (paused during powerpill)
    // src:4dc2
    counter__related_to_ghost_orientation_changes: u16,

    // counter 0..8 to handle things once every 8 times
    // src:4dc4
    counter__to_handle_things_once_every_8_times: u8,

    // src:4dc5
    man_dead_animation_counter: u16,

    // src:4dc7
    current_orientation_we_are_trying: Direction,
    // src:4dc8
    counter_used_to_change_ghost_colors_under_big_pill_effects: u8,
    // src:4dc9
    pointer_to_pick_a_random_value_from_the_ROM: u16,
    // src:4dcb
    counter_while_ghosts_are_blue: u16,

    // src:4dcd
    // unused

    // src:4dce
    // see credits.rs

    // src:4dcf
    counter_to_handle_power_pill_flashes: u8,

    // src:4dd0
    // how many ghosts eaten this powerpill? 0..4?
    counter_current_number_of_killed_ghosts: usize,

    // src:4dd1 FRUITP  fruit position
    pub killed_ghost_animation_state: i8,

    // src:4dd2 
    // see sprites_coord_xy

    // src:4dd4
    pub fruit_points: i8,

    // src:4dd5
    // unused

    // src:[4dd7..=4dff] empty bytes


    // src:4e04
    pub subroutine_playing_state: u8,   // 0x0E = end of level

    // src:4e09
    pub current_player: usize,
    // src:4e0a, src:4e38
    pub state_player: [StatePlayer; 2],

    // src:4e72
    pub cocktail: bool,
    // src:4e73
    pub p_ghost_difficulty: &'static [usize; 21],

}

impl GamePlaying {

    // src:260f
    pub fn new_is_end(&mut self) {
        self.sprites_coord_xy[SpriteName::Red as usize] = Point::new(0,148);
        self.sprites_coord_xy[SpriteName::Pink as usize] = Point::new(0,148);
        self.sprites_coord_xy[SpriteName::Blue as usize] = Point::new(0,148);
        self.sprites_coord_xy[SpriteName::Orange as usize] = Point::new(0,148);
        self.sprites_coord_xy[SpriteName::Man as usize] = Point::new(128,196);

        self.sprites_coord_middle_of_tile[SpriteName::Red as usize] = Point::new(30,50);
        self.sprites_coord_middle_of_tile[SpriteName::Pink as usize] = Point::new(30,50);
        self.sprites_coord_middle_of_tile[SpriteName::Blue as usize] = Point::new(30,50);
        self.sprites_coord_middle_of_tile[SpriteName::Orange as usize] = Point::new(30,50);

        self.sprites_current_tile_xy[SpriteName::Red as usize] = Point::new(30,50);
        self.sprites_current_tile_xy[SpriteName::Pink as usize] = Point::new(30,50);
        self.sprites_current_tile_xy[SpriteName::Blue as usize] = Point::new(30,50);
        self.sprites_current_tile_xy[SpriteName::Orange as usize] = Point::new(30,50);

        self.sprites_move_xy_direction[SpriteName::Red as usize] = Point::new(1, 0);
        self.sprites_move_xy_direction[SpriteName::Pink as usize] = Point::new(1, 0);
        self.sprites_move_xy_direction[SpriteName::Blue as usize] = Point::new(1, 0);
        self.sprites_move_xy_direction[SpriteName::Orange as usize] = Point::new(1, 0);

        self.sprites_face_xy_direction[SpriteName::Red as usize] = Point::new(1, 0);
        self.sprites_face_xy_direction[SpriteName::Pink as usize] = Point::new(1, 0);
        self.sprites_face_xy_direction[SpriteName::Blue as usize] = Point::new(1, 0);
        self.sprites_face_xy_direction[SpriteName::Orange as usize] = Point::new(1, 0);

        self.sprites_move_xy_direction[SpriteName::Man as usize] = Point::new(1,0);
        self.sprites_face_xy_direction[SpriteName::Orange as usize] = Point::new(1, 0);

        self.sprites_ghosts_previous_orientation[SpriteName::Red as usize] = Direction::Left;
        self.sprites_ghosts_previous_orientation[SpriteName::Pink as usize] = Direction::Left;
        self.sprites_ghosts_previous_orientation[SpriteName::Blue as usize] = Direction::Left;
        self.sprites_ghosts_previous_orientation[SpriteName::Orange as usize] = Direction::Left;
        self.sprites_ghosts_face_enum_direction[SpriteName::Red as usize] = Direction::Left;
        self.sprites_ghosts_face_enum_direction[SpriteName::Pink as usize] = Direction::Left;
        self.sprites_ghosts_face_enum_direction[SpriteName::Blue as usize] = Direction::Left;
        self.sprites_ghosts_face_enum_direction[SpriteName::Orange as usize] = Direction::Left;
        self.man_orientation = Direction::Left;

        self.wanted_man_orientation = Direction::Left;
        self.sprites_coord_xy[SpriteName::Man as usize] = Point::new(8, 148);

        self.sprites_coord_middle_of_tile[SpriteName::Man as usize] = Point::new(31, 50);
        self.sprites_current_tile_xy[SpriteName::Man as usize] = Point::new(31, 50);
    }

    // src:2576
    pub fn new_is_not_end(&mut self) {
        self.sprites_coord_xy[SpriteName::Red as usize] = Point::new(128,100);
        self.sprites_coord_xy[SpriteName::Pink as usize] = Point::new(128,124);
        self.sprites_coord_xy[SpriteName::Blue as usize] = Point::new(144,124);
        self.sprites_coord_xy[SpriteName::Orange as usize] = Point::new(112,124);
        self.sprites_coord_xy[SpriteName::Man as usize] = Point::new(128,196);

        self.sprites_coord_middle_of_tile[SpriteName::Red as usize] = Point::new(46,44);
        self.sprites_current_tile_xy[SpriteName::Red as usize] = Point::new(46,44);
        self.sprites_coord_middle_of_tile[SpriteName::Pink as usize] = Point::new(46,47);
        self.sprites_current_tile_xy[SpriteName::Pink as usize] = Point::new(46,47);
        self.sprites_coord_middle_of_tile[SpriteName::Blue as usize] = Point::new(48,47);
        self.sprites_current_tile_xy[SpriteName::Blue as usize] = Point::new(48,47);
        self.sprites_coord_middle_of_tile[SpriteName::Orange as usize] = Point::new(44,47);
        self.sprites_current_tile_xy[SpriteName::Orange as usize] = Point::new(44,47);
        self.sprites_coord_middle_of_tile[SpriteName::Man as usize] = Point::new(46,56);
        self.sprites_current_tile_xy[SpriteName::Man as usize] = Point::new(46,56);

        self.sprites_move_xy_direction[SpriteName::Red as usize] = Point::new(1, 0);
        self.sprites_face_xy_direction[SpriteName::Red as usize] = Point::new(1, 0);
        self.sprites_move_xy_direction[SpriteName::Pink as usize] = Point::new(0, 1);
        self.sprites_face_xy_direction[SpriteName::Pink as usize] = Point::new(0, 1);
        self.sprites_move_xy_direction[SpriteName::Blue as usize] = Point::new(0, -1);
        self.sprites_face_xy_direction[SpriteName::Blue as usize] = Point::new(0, -1);
        self.sprites_move_xy_direction[SpriteName::Orange as usize] = Point::new(0, -1);
        self.sprites_face_xy_direction[SpriteName::Orange as usize] = Point::new(0, -1);
        self.sprites_move_xy_direction[SpriteName::Man as usize] = Point::new(1, 0);
        self.sprites_face_xy_direction[SpriteName::Man as usize] = Point::new(1, 0);

        self.sprites_ghosts_previous_orientation[SpriteName::Red as usize] = Direction::Down;
        self.sprites_ghosts_previous_orientation[SpriteName::Pink as usize] = Direction::Left;
        self.sprites_ghosts_face_enum_direction[SpriteName::Red as usize] = Direction::Down;
        self.sprites_ghosts_face_enum_direction[SpriteName::Pink as usize] = Direction::Left;
        self.sprites_ghosts_previous_orientation[SpriteName::Blue as usize] = Direction::Up;
        self.sprites_ghosts_previous_orientation[SpriteName::Orange as usize] = Direction::Up;
        self.sprites_ghosts_face_enum_direction[SpriteName::Blue as usize] = Direction::Up;
        self.sprites_ghosts_face_enum_direction[SpriteName::Orange as usize] = Direction::Up;

        self.man_orientation = Direction::Left;
        self.wanted_man_orientation = Direction::Left;
        self.sprites_coord_xy[SpriteName::Fruit as usize] = Point::new(0,0);

    }
    // src:26a2
    pub fn new() -> GamePlaying {
        GamePlaying {
            sprite: [
                SpriteElement::new_red_ghost(),
                SpriteElement::new_pink_ghost(),
                SpriteElement::new_blue_ghost(),
                SpriteElement::new_orange_ghost(),
                SpriteElement::new_man(),
                SpriteElement::new_fruit(),
            ],
            sprites_coord_xy: [Point::new(0,0); 6],
            sprites_coord_middle_of_tile: [Point::new(0,0); 6],
            sprites_move_xy_direction: [Point::new(0,0); 6],
            sprites_face_xy_direction: [Point::new(0,0); 6],
            sprites_ghosts_previous_orientation: [Direction::Right; 4],
            sprites_ghosts_face_enum_direction: [Direction::Right; 4],
            man_orientation: Direction::Right,
            sprites_current_tile_xy: [Point::new(0,0); 6],
            ghost_best_orientation_found: Direction::Right,
            wanted_man_orientation: Direction::Right,
            opposite_orientation: Direction::Left,  // HACK: manual add for new()

            ghost_current_tile_position: Point::new(0,0),
            ghost_destination_tile_position: Point::new(0,0),
            ghost_temp_resulting_position: Point::new(0,0),
            minimum_distance_square_found: Point::new(0,0),
            man_movement: ManSpeedBitPatterns::new(0),
            red_ghost_movement: GhostSpeedBitPatterns::new(0),
            pink_ghost_movement: GhostSpeedBitPatterns::new(0),
            blue_ghost_movement: GhostSpeedBitPatterns::new(0),
            orange_ghost_movement: GhostSpeedBitPatterns::new(0),
            ghost_counter_for_orientation_change: [0; 7],
            counter_related_to_ghost_movement_inside_home: 0,
            number_of_units_before_ghost_leaves_home: 0,
            inactivity_counter_for_units_of_the_above: 0,
            delay_ghost_movement: [0; 4],

            delay_man_movement: 0,  // TODO: can be directly -1?
            ghost_substate_if_alive: [GhostSubState::AtHome; 4],
            number_of_ghost_killed_but_no_collision_for_yet: KillingGhostState::Nothing,
            man_dead_animation_state: 0,
            power_pill_effect: false,
            ghost_blue_flag: [false; 4],
            killing_ghost_state: KillingGhostState::Nothing,
            ghost_state: [GhostState::Alive; 4],
            change_orientation_flag: [false; 5],


            red_ghost_first_difficulty_flag: false, // HACK: manual add for new()
            red_ghost_second_difficulty_flag: false,    // HACK: manual add for new()
            pink_ghost_counter_to_go_out_of_home_limit: 0,  // HACK: manual add for new()
            blue_ghost_counter_to_go_out_of_home_limit: 0,  // HACK: manual add for new()
            orange_ghost_counter_to_go_out_of_home_limit: 0,    // HACK: manual add for new()
            red_ghost_remainder_of_pills_when_first_difficulty_flag_is_set: 0,  // HACK: manual add for new()
            red_ghost_remainder_of_pills_when_second_difficulty_flag_is_set: 0, // HACK: manual add for new()
            time_the_ghosts_stay_blue_when_pacman_eats_a_big_pill: 0,   // HACK: manual add for new()
            pacman_about_to_enter_a_tunnel: false,  // HACK: manual add for new()
            counter__change_every_8_frames: 0,  // HACK: manual add for new()
            counter__orientation_changes_index: 0,  // HACK: manual add for new()
            counter__related_to_ghost_orientation_changes: 0,   // HACK: manual add for new()
            counter__to_handle_things_once_every_8_times: 0,    // HACK: manual add for new()

            man_dead_animation_counter: 0,
            
            current_orientation_we_are_trying: Direction::Right,
            counter_used_to_change_ghost_colors_under_big_pill_effects: 0,
            pointer_to_pick_a_random_value_from_the_ROM: 0,
            counter_while_ghosts_are_blue: 0,


            counter_to_handle_power_pill_flashes: 0,
            counter_current_number_of_killed_ghosts: 0,
            killed_ghost_animation_state: 0,
            fruit_points: 0,

            subroutine_playing_state: 0,

            current_player: 0,
            state_player: [StatePlayer::new(GhostDifficulty::get_difficulty_settings(false)); 2],

            cocktail: false,
            p_ghost_difficulty: GhostDifficulty::get_difficulty_settings(false),
        }
    }

    // src:06be
    pub fn execute_playing_task_state(&mut self,
                                    timed_task: &mut GameTaskTimed,
                                    tasks: &mut GameTask, 
                                    hwvideo: &mut GameHwVideo, 
                                    hwsound: &mut SoundChannels,
                                    hwinput: &HardwareInput,
                                    hwoutput: &mut HardwareOutput,
                                    main_state: &MainStateE, 
                                    subroutine_attract_state: u8) {
        println!("playing_state={}", self.subroutine_playing_state);
        match self.subroutine_playing_state {
             0 => self.p00_reset_game_data(), // set up game initialization
             1 => self.p01_init_screen_or_p09(timed_task, tasks, hwoutput, main_state), // set up tasks for beginning of game
// println!("MY CURRENT_HUMAN_REVERSE_POINTER / PC :)");
// PC
             3 => self.p03_check_rack_test(timed_task, tasks, hwvideo, hwsound, hwinput, subroutine_attract_state), // demo mode or player is playing
             4 => self.p04_player_is_died_game_over(), // when player has collided with hostile ghost (died)
             6 => self.p06_switch_player(), // check for game over, do things if true
             8 => self.p08_end_of_demo(), // end of demo mode when ms pac dies in demo.  clears a bunch of memories.
             9 => self.p09_prepare_screen_level(), // sets a bunch of tasks and displays "ready" or "game over"
            11 => self.p0b_loop_state_p03(), // begin start of maze demo after marquee
            12 => self.p0c_end_of_level_clear_sound(), // clears sounds and sets a small delay.  run at end of each level
            14 => self.p0e_flash_screen_on(), // flash screen
            16 => self.p10_flash_screen_off(), // flash screen
            18 => self.p0e_flash_screen_on(), // flash screen
            20 => self.p10_flash_screen_off(), // flash screen
            22 => self.p0e_flash_screen_on(), // flash screen
            24 => self.p10_flash_screen_off(), // flash screen
            26 => self.p0e_flash_screen_on(), // flash screen
            28 => self.p10_flash_screen_off(), // flash screen
            30 => self.p1e_end_of_level_after_flash_screen(), // set a bunch of tasks
            32 => self.p20_end_of_level_clear_sounds_and_run_intermissions(), // clears all sounds 
            34 => self.p22_prepare_next_level(), // clears sounds, increases level, increases difficulty if needed, resets pill maps
            35 => self.p09_prepare_screen_level(), // get game ready to play and set this sub back to #03
            37 => self.p0b_loop_state_p03(), // sets sub # back to #03
            _ => {},
        }
    }

    // src:0879
    /* main routine #3.  arrive here at the start of the game when a new game is started */
    fn p00_reset_game_data(&mut self) {
        self.state_player = [StatePlayer::new(self.p_ghost_difficulty); 2];
        self.subroutine_playing_state += 1;
    }

    // src:0899
    fn p01_init_screen_or_p09(&mut self,
                            timed_task: &mut GameTaskTimed,
                            tasks: &mut GameTask,
                            hwoutput: &mut HardwareOutput,
                            main_state: &MainStateE){
        if *main_state == MainStateE::Attract {
            self.subroutine_playing_state = 9;
        } else {
            tasks.add(TaskCoreE::ClearFullDataGame);
            tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::PlayerOne,false));
            tasks.add(TaskCoreE::ResetSpritesToDefaultValues(false));
            tasks.add(TaskCoreE::ResetGhostHomeCounter(false));
            tasks.add(TaskCoreE::SetupDifficulty);
            tasks.add(TaskCoreE::DrawRemainingLivesBottomLeftScreen);
            timed_task.add(CurrentTime::LessTenth, 20, TaskTimedNameE::IncreaseSubroutinePlayingState);
            timed_task.add(CurrentTime::LessTenth, 20, TaskTimedNameE::ClearReadyMessage);
            hwoutput.flip_screen = self.current_player == 1 && self.cocktail;
            self.subroutine_playing_state += 1;
        }
    }

    // src:08cd
    fn p03_check_rack_test(&mut self, timed_task: &mut GameTaskTimed, tasks: &mut GameTask, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, hwinput: &HardwareInput, subroutine_attract_state: u8) {
        if hwinput.rack_test {
            self.subroutine_playing_state = 14;
            tasks.add(TaskCoreE::ClearPills);
        } else {
            self.check_if_board_is_cleared(timed_task, tasks, hwvideo, hwsound, subroutine_attract_state);
        }
    }

    // src:090d
    fn p04_player_is_died_game_over(&mut self){
        println!("p04");
    }

    // src:0940
    fn p06_switch_player(&mut self){
        println!("p06");
    }

    // src:0972
    fn p08_end_of_demo(&mut self){
        println!("p08");
    }

    // src:0988
    fn p09_prepare_screen_level(&mut self){
        println!("p09");
    }

    // src:09d2
    fn p0b_loop_state_p03(&mut self){
        println!("p0b");
    }

    // src:09d8
    fn p0c_end_of_level_clear_sound(&mut self){
        println!("p0c");
    }

    // src:09e8
    fn p0e_flash_screen_on(&mut self){
        println!("p0e");
    }

    // src:09fe
    fn p10_flash_screen_off(&mut self){
        println!("p10");
    }

    // src:0a0e
    fn p1e_end_of_level_after_flash_screen(&mut self){
        println!("p1e");
    }

    // src:0a2c
    fn p20_end_of_level_clear_sounds_and_run_intermissions(&mut self){
        println!("p20");
    }

    // src:0a7c
    fn p22_prepare_next_level(&mut self){
        println!("p22");
    }

    // src:94a1
    // routine to determine the number of pellets which must be eaten
    fn check_if_board_is_cleared(&mut self, timed_task: &mut GameTaskTimed, tasks: &mut GameTask, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, subroutine_attract_state: u8) {
        let pellet_to_eat = self.get_data_from_current_level(&PELLET_TO_EAT);
        if self.state_player[self.current_player].dots_eaten == *pellet_to_eat {
            self.subroutine_playing_state = 12;
        } else {
            // src:08eb
            self.update_ghost_and_pacman_state(timed_task, tasks, hwvideo, hwsound, subroutine_attract_state);
            // self.update_ghost_and_pacman_state();
            // self.is_time_to_leave_house();
            // self.adjust_ghost_movement(param_1);
            // self.is_time_to_change_ghost_animation();
            // self.is_time_to_reverse_ghost_direction();
            // self.handle_ghost_flashing_and_colors_when_power_pills_are_eaten();
            // self.pacman_only_set_color_for_dead_ghost();
            // self.handle_power_pill_flashes();
            // self.change_background();
            // self.check_for_fruit_to_come_out();
        }
    }

    // src:1017
    fn update_ghost_and_pacman_state(&mut self, timed_task: &mut GameTaskTimed, tasks: &mut GameTask, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, subroutine_attract_state: u8) {
        self.man_dead_animation(hwsound);
        if self.man_dead_animation_state != 0 {
            return;
        }
        self.check_for_ghosts_being_eaten_and_set_ghost_states_accordingly();
        self.red_ghost_movement_update(tasks, hwvideo, hwsound, subroutine_attract_state);
        self.pink_ghost_movement_update(tasks, hwvideo, hwsound, subroutine_attract_state);
        self.blue_ghost_movement_update(tasks, hwvideo, hwsound, subroutine_attract_state);
        self.orange_ghost_movement_update(tasks, hwvideo, hwsound, subroutine_attract_state);
        if self.number_of_ghost_killed_but_no_collision_for_yet != KillingGhostState::Nothing {
            self.ghost_bonus_anim_after_eaten(timed_task, hwsound);
        } else {
            self.check_for_collision_with_regular_ghost(tasks, hwsound);
            self.check_for_collision_with_blue_ghost(tasks, hwsound);
            if self.number_of_ghost_killed_but_no_collision_for_yet == KillingGhostState::Nothing {
                self.handles_pacman_movement();
            //     control_movement_red_ghost();
            //     control_movement_pink_ghost();
            //     control_movement_blue_ghost();
            //     control_movement_orange_ghost();
            //     if (subroutine_PLAYING_state == GHOST_MOVE) {
            //         control_blue_ghost_timer();
            //         leave_house_check_pink_ghost();
            //         leave_house_check_blue_ghost();
            //         leave_house_check_orange_ghost();
            //     }

            }

        }
    }

    // src:1066
    fn check_for_ghosts_being_eaten_and_set_ghost_states_accordingly(&mut self) {
        match self.killing_ghost_state {
            KillingGhostState::KillRed => {
                self.killing_ghost_state = KillingGhostState::Nothing;
                self.ghost_state[SpriteName::Red as usize] = GhostState::Dead;
            },
            KillingGhostState::KillPink => {
                self.killing_ghost_state = KillingGhostState::Nothing;
                self.ghost_state[SpriteName::Pink as usize] = GhostState::Dead;
            },
            KillingGhostState::KillBlue => {
                self.killing_ghost_state = KillingGhostState::Nothing;
                self.ghost_state[SpriteName::Blue as usize] = GhostState::Dead;
            },
            KillingGhostState::KillOrange => {
                self.killing_ghost_state = KillingGhostState::Nothing;
                self.ghost_state[SpriteName::Orange as usize] = GhostState::Dead;
            },
            _ => {},
        }
    }
    
    // src:1094
    fn red_ghost_movement_update(&mut self, tasks: &mut GameTask, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, subroutine_attract_state: u8) {
        match self.ghost_state[SpriteName::Red as usize] {
            GhostState::Dead => {
                self.handles_ghost_movement(tasks, hwvideo, SpriteName::Red, subroutine_attract_state);
                // right above the ghost house ?
                if self.sprite[SpriteName::Red as usize].p == Point::new(128, 100) {
                    self.ghost_state[SpriteName::Red as usize] = GhostState::EnteringHomeAfterBeingKilled;
                }
            },
            GhostState::EnteringHomeAfterBeingKilled => {
                self.state_ghost_eyes_above_red_pink(SpriteName::Red, hwsound);
            },
            _ => {},
        }
    }
    // src:109e
    fn pink_ghost_movement_update(&mut self, tasks: &mut GameTask, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, subroutine_attract_state: u8) {
        match self.ghost_state[SpriteName::Pink as usize] {
            GhostState::Dead => {
                self.handles_ghost_movement(tasks, hwvideo, SpriteName::Pink, subroutine_attract_state);
                // right above the ghost house ?
                if self.sprite[SpriteName::Pink as usize].p == Point::new(128, 100) {
                    self.ghost_state[SpriteName::Pink as usize] = GhostState::EnteringHomeAfterBeingKilled;
                }
            },
            GhostState::EnteringHomeAfterBeingKilled => {
                self.state_ghost_eyes_above_red_pink(SpriteName::Pink, hwsound);
            },
            _ => {},
        }
    }
    // src:10a8
    fn blue_ghost_movement_update(&mut self, tasks: &mut GameTask, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, subroutine_attract_state: u8) {
        let sni = SpriteName::Blue as usize;

        match self.ghost_state[sni] {
            GhostState::Dead => {
                self.handles_ghost_movement(tasks, hwvideo, SpriteName::Blue, subroutine_attract_state);
                // right above the ghost house ?
                if self.sprite[sni].p == Point::new(128, 100) {
                    self.ghost_state[sni] = GhostState::EnteringHomeAfterBeingKilled;
                }
            },
            GhostState::EnteringHomeAfterBeingKilled => {
                self.state_ghost_eyes_above_blue_orange(SpriteName::Blue);
            },
            GhostState::GoLeftAfterEnteringHomeAfterDead => {
                self.state_ghost_eyes_in_house_blue(hwsound, SpriteName::Blue);
            },
            _ => {},
        }
    }
    // src:10b4
    fn orange_ghost_movement_update(&mut self, tasks: &mut GameTask, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, subroutine_attract_state: u8) {
        let sni = SpriteName::Orange as usize;
        match self.ghost_state[sni] {
            GhostState::Dead => {
                self.handles_ghost_movement(tasks, hwvideo, SpriteName::Orange, subroutine_attract_state);
                // right above the ghost house ?
                if self.sprite[sni].p == Point::new(128, 100) {
                    self.ghost_state[sni] = GhostState::EnteringHomeAfterBeingKilled;
                }
            },
            GhostState::EnteringHomeAfterBeingKilled => {
                self.state_ghost_eyes_above_blue_orange(SpriteName::Orange);
            },
            GhostState::GoLeftAfterEnteringHomeAfterDead => {
                self.state_ghost_eyes_in_house_orange(hwsound, SpriteName::Orange);
            },
            _ => {},
        }
    }


    // src:10d2, src:112a
    // arrive here when ghost eyes are above and entering the ghost house when returning home
    fn state_ghost_eyes_above_red_pink(&mut self, sn: SpriteName, hwsound: &mut SoundChannels) {
        let sni = sn as usize;

        let down = DIRECTION_TILE_MOVEMENTS[Direction::Down as usize];
        self.sprite[sni].p+=down;

        if self.sprite[sni].p.y != 128 {
            self.sprites_ghosts_previous_orientation[sni] = Direction::Down;
            self.sprites_ghosts_face_enum_direction[sni] = Direction::Down;
        } else {
            self.sprites_coord_middle_of_tile[sni] = Point::new(46,47);
            self.sprites_current_tile_xy[sni] = Point::new(46,47);

            self.ghost_substate_if_alive[sni] = GhostSubState::AtHome;
            self.ghost_state[sni] = GhostState::Alive;
            self.ghost_blue_flag[sni] = false;
            self.check_one_ghost_dead(hwsound);
        }
    }
    // src:1101
    fn check_one_ghost_dead(&mut self, hwsound: &mut SoundChannels) {
        if self.ghost_state[SpriteName::Red as usize] != GhostState::Alive ||
        self.ghost_state[SpriteName::Pink as usize] != GhostState::Alive ||
        self.ghost_state[SpriteName::Blue as usize] != GhostState::Alive ||
        self.ghost_state[SpriteName::Orange as usize] != GhostState::Alive {
            hwsound.effect[1].num &= 0b1011_1111;  // TODO What?
        }
    }

    // src:116e, src:11db
    // arrive here when ghost eyes are above and entering the ghost house when returning home
    fn state_ghost_eyes_above_blue_orange(&mut self, sn: SpriteName) {
        let sni = sn as usize;

        let down = DIRECTION_TILE_MOVEMENTS[Direction::Down as usize];
        self.sprite[sni].p+=down;

        self.sprites_ghosts_previous_orientation[sni] = Direction::Down;
        self.sprites_ghosts_face_enum_direction[sni] = Direction::Down;

        if self.sprite[sni].p.y == 128 {
            self.ghost_state[sni] = GhostState::GoLeftAfterEnteringHomeAfterDead;
        }
    }

    // src:118f
    fn state_ghost_eyes_in_house_blue(&mut self, hwsound: &mut SoundChannels, sn: SpriteName) {
        let sni = sn as usize;

        let left = DIRECTION_TILE_MOVEMENTS[Direction::Left as usize];
        self.sprite[sni].p+=left;

        if self.sprite[sni].p.y != 144 {
            self.sprites_ghosts_previous_orientation[sni] = Direction::Left;
            self.sprites_ghosts_face_enum_direction[sni] = Direction::Left;
        } else {
            self.sprites_coord_middle_of_tile[sni] = Point::new(48,47);
            self.sprites_current_tile_xy[sni] = Point::new(48,47);

            self.sprites_ghosts_previous_orientation[sni] = Direction::Down;
            self.sprites_ghosts_face_enum_direction[sni] = Direction::Down;

            self.ghost_substate_if_alive[sni] = GhostSubState::AtHome;
            self.ghost_state[sni] = GhostState::Alive;
            self.ghost_blue_flag[sni] = false;
            self.check_one_ghost_dead(hwsound);
        }
    }
    // src:11fc
    fn state_ghost_eyes_in_house_orange(&mut self, hwsound: &mut SoundChannels, sn: SpriteName) {
        let sni = sn as usize;

        let right = DIRECTION_TILE_MOVEMENTS[Direction::Right as usize];
        self.sprite[sni].p+=right;

        if self.sprite[sni].p.y != 112 {
            self.sprites_ghosts_previous_orientation[sni] = Direction::Left;
            self.sprites_ghosts_face_enum_direction[sni] = Direction::Left;
        } else {
            self.sprites_coord_middle_of_tile[sni] = Point::new(44,47);
            self.sprites_current_tile_xy[sni] = Point::new(44,47);

            self.sprites_ghosts_previous_orientation[sni] = Direction::Down;
            self.sprites_ghosts_face_enum_direction[sni] = Direction::Down;

            self.ghost_substate_if_alive[sni] = GhostSubState::AtHome;
            self.ghost_state[sni] = GhostState::Alive;
            self.ghost_blue_flag[sni] = false;
            self.check_one_ghost_dead(hwsound);
        }
    }

    // src:1235
    fn ghost_bonus_anim_after_eaten(&mut self, timed_task: &mut GameTaskTimed, hwsound: &mut SoundChannels) {
        match self.killed_ghost_animation_state {
            0 | 2 => {
                // src:123f
                let mut current_ghost = &mut self.sprite[self.number_of_ghost_killed_but_no_collision_for_yet as usize];

                if self.killed_ghost_animation_state == 0 {
                    let mut bonus = match self.counter_current_number_of_killed_ghosts {
                        1 => SpriteId::T200,
                        2 => SpriteId::T400,
                        3 => SpriteId::T800,
                        4 => SpriteId::T1600,
                        _ => SpriteId::T1600,
                    };

                    /* flip x and y in cocktail mode */
                    if self.cocktail && self.current_player != 0 {
                        bonus = bonus.flip_xy();
                    }

                    current_ghost.s = bonus;
                    current_ghost.c = ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor;
                    self.sprite[SpriteName::Man as usize].c = ColorE::Black;
                    timed_task.add(CurrentTime::LessTenth, 10, TaskTimedNameE::IncreaseKilledGhostAnimationState);
                } else {
                    current_ghost.s = SpriteId::GhostRight1;
                    self.killing_ghost_state = self.number_of_ghost_killed_but_no_collision_for_yet;
                    self.sprite[SpriteName::Man as usize].c = ColorE::Yellow;
                    self.number_of_ghost_killed_but_no_collision_for_yet = KillingGhostState::Nothing;
                    self.killed_ghost_animation_state = 0;
                    hwsound.effect[1].num |= 0b0100_0000;
                }

            }
            _ => {},
        }
    }

    // src:1291
    fn man_dead_animation(&mut self, hwsound: &mut SoundChannels) {
        match self.man_dead_animation_state {
            1..=4 => {
                self.man_dead_animation_counter += 1;
                if self.man_dead_animation_counter == 120 {
                    self.man_dead_animation_state = 5;
                }
            },
            5 => {
                self.clear_all_ghosts_from_screen();
                self.animate_dead_mspac(SpriteId::MsDown1, 180);
            },
            6 => {
                hwsound.effect[2].num |= 0x10;
                self.animate_dead_mspac(SpriteId::MsLeft1, 195);
            },
            7 => self.animate_dead_mspac(SpriteId::MsUp1, 210),
            8 => self.animate_dead_mspac(SpriteId::MsRight1, 225),
            9 => self.animate_dead_mspac(SpriteId::MsDown1Bis, 240),
            10 => self.animate_dead_mspac(SpriteId::MsLeft1Bis, 255),
            11 => self.animate_dead_mspac(SpriteId::MsUp1Bis,270),
            12 => self.animate_dead_mspac(SpriteId::MsRight1Bis,285),
            13 => self.animate_dead_mspac(SpriteId::MsDown1Ter,300),
            14 => self.animate_dead_mspac(SpriteId::MsLeft1Ter,315),
            15 => {
                hwsound.effect[2].num = 0;
                self.animate_dead_mspac(SpriteId::MsUp1Ter,345);
            },
            16 => {
                self.sprite[SpriteName::Man as usize].s = SpriteId::FruitStart;
                self.man_dead_animation_counter += 1;
                if self.man_dead_animation_counter == 440 {
                    self.state_player[self.current_player].real_number_of_lives -= 1;
                    self.state_player[self.current_player].number_of_lives_displayed -= 1;
                    self.clear_fruit_and_pacman_position();
                    self.subroutine_playing_state += 1;
                }
            },
            _ => {},
        }
    }

    // src:12d6
    fn animate_dead_mspac(&mut self, sprite: SpriteId, counter: u16) {
        let sprite_ok = if self.current_player == 1 && self.cocktail {
            sprite.flip_xy()
        } else {
            sprite
        };
        self.sprite[SpriteName::Man as usize].s = sprite_ok;

        self.man_dead_animation_counter += 1;
        if self.man_dead_animation_counter == counter {
            self.man_dead_animation_state += 1;
        }
    }


    // src:171d
    fn check_for_collision_with_regular_ghost(&mut self, tasks: &mut GameTask, hwsound: &mut SoundChannels)
    {
        //  normal ghost collision detect
        let man_p = self.sprites_current_tile_xy[SpriteName::Man as usize];
        let orange_collision:bool = self.ghost_state[SpriteName::Orange as usize] == GhostState::Alive && self.sprites_current_tile_xy[SpriteName::Orange as usize] == man_p;
        let blue_collision:bool = self.ghost_state[SpriteName::Blue as usize] == GhostState::Alive && self.sprites_current_tile_xy[SpriteName::Blue as usize] == man_p;
        let pink_collision:bool = self.ghost_state[SpriteName::Pink as usize] == GhostState::Alive && self.sprites_current_tile_xy[SpriteName::Pink as usize] == man_p;
        let red_collision:bool = self.ghost_state[SpriteName::Red as usize] == GhostState::Alive && self.sprites_current_tile_xy[SpriteName::Red as usize] == man_p;
        self.check_for_collision(tasks, hwsound, red_collision, pink_collision, blue_collision, orange_collision);
    }

    // src:1763
    fn check_for_collision(&mut self, tasks: &mut GameTask, hwsound: &mut SoundChannels, red_collision:bool, pink_collision:bool, blue_collision:bool, orange_collision: bool) {
        if orange_collision {
            self.number_of_ghost_killed_but_no_collision_for_yet = KillingGhostState::KillOrange;
        } else if blue_collision {
            self.number_of_ghost_killed_but_no_collision_for_yet = KillingGhostState::KillBlue;
        } else if pink_collision {
            self.number_of_ghost_killed_but_no_collision_for_yet = KillingGhostState::KillPink;
        } else if red_collision {
            self.number_of_ghost_killed_but_no_collision_for_yet = KillingGhostState::KillRed;
        } else {
            self.number_of_ghost_killed_but_no_collision_for_yet = KillingGhostState::Nothing;
        }

        let ghost_num = self.number_of_ghost_killed_but_no_collision_for_yet as u8;
        if self.number_of_ghost_killed_but_no_collision_for_yet == KillingGhostState::Nothing {
            self.man_dead_animation_state = 0;
        } else if self.ghost_blue_flag[ghost_num as usize - 1] == false {
            self.man_dead_animation_state = ghost_num;
        } else {
            // arrive here when eating a blue ghost
            self.man_dead_animation_state = 0;
            let score_index = 2 + self.counter_current_number_of_killed_ghosts;  // index 2 for first ghost
            self.counter_current_number_of_killed_ghosts += 1;
            // Rust HACK, here we use tasks, because we don't want attribut propagation fn for T19...
            //self.t19_update_score_then_draw(score_index);
            tasks.add(TaskCoreE::UpdateScoreThenDraw(score_index));
            hwsound.effect[2].num |= 0b0000_1000;
        }
    }

    // src:1789
    fn check_for_collision_with_blue_ghost(&mut self, tasks: &mut GameTask, hwsound: &mut SoundChannels) {
        // end normal ghost collision detect blue (edible) ghost collision detect
        if self.number_of_ghost_killed_but_no_collision_for_yet != KillingGhostState::Nothing {
            return;
        }
        if self.power_pill_effect == false {
            return;
        }

        let man_p:Point = self.sprites_current_tile_xy[SpriteName::Man as usize];
        let orange_collision:bool = self.ghost_state[SpriteName::Orange as usize] == GhostState::Alive && ((self.sprites_current_tile_xy[SpriteName::Orange as usize].x - man_p.x) <= 4) && ((self.sprites_current_tile_xy[SpriteName::Orange as usize].y - man_p.y) <= 4);
        let blue_collision:bool = self.ghost_state[SpriteName::Blue as usize] == GhostState::Alive && ((self.sprites_current_tile_xy[SpriteName::Blue as usize].x - man_p.x) <= 4) && ((self.sprites_current_tile_xy[SpriteName::Blue as usize].y - man_p.y) <= 4);
        let pink_collision:bool = self.ghost_state[SpriteName::Pink as usize] == GhostState::Alive && ((self.sprites_current_tile_xy[SpriteName::Blue as usize].x - man_p.x) <= 4) && ((self.sprites_current_tile_xy[SpriteName::Pink as usize].y - man_p.y) <= 4);
        let red_collision:bool = self.ghost_state[SpriteName::Red as usize] == GhostState::Alive && ((self.sprites_current_tile_xy[SpriteName::Blue as usize].x - man_p.x) <= 4) && ((self.sprites_current_tile_xy[SpriteName::Red as usize].y - man_p.y) <= 4);
        self.check_for_collision(tasks, hwsound, red_collision, pink_collision, blue_collision, orange_collision);
    }
    // src:1806 TODO
    fn handles_pacman_movement(&mut self) {

        /* delay man movement */
        if self.delay_man_movement != -1 {
            self.delay_man_movement -= 1;
            return;
        }

        /* movement when power pill is active or not */
        if self.power_pill_effect {
            // movement when power pill active
            self.man_movement.big_pill_state *= 2;
        } else {
            // movement when power pill not active
            self.man_movement.normal_state *= 2;
        }

        /* all pacman movement */

        // TODO
    }

    // src:1bd8, src:1caf
    fn handles_ghost_movement(&mut self, tasks: &mut GameTask, hwvideo: &mut GameHwVideo, sn: SpriteName, subroutine_attract_state: u8) {
        let sni = sn as usize;

        if self.sprites_move_xy_direction[sni] != Point::new(0, 0) {
            self.sprite[sni].p.x = self.sprite[sni].p.y;
        }
        // middle of tile?
        if self.sprites_coord_xy[sni].x & 7 == 4 {
            if ! self.is_sprite_using_tunnel(sn) {
                if self.ghost_blue_flag[sni] == false {
                    let (t,c) = hwvideo.get_screen(self.sprites_coord_middle_of_tile[sni]);
                    if c != ColorE::ColorPacmanAndGhostInitialMapPositions {
                        match sn {
                            SpriteName::Red => tasks.add(TaskCoreE::RedGhostAi),
                            SpriteName::Pink => tasks.add(TaskCoreE::PinkGhostAi),
                            SpriteName::Blue => tasks.add(TaskCoreE::BlueGhostAi),
                            SpriteName::Orange => tasks.add(TaskCoreE::OrangeGhostAi),
                            _ => {},
                        }
                        
                    }
                } else {
                    match sn {
                        SpriteName::Red => tasks.add(TaskCoreE::RedGhostMovementWhenPowerPill),
                        SpriteName::Pink => tasks.add(TaskCoreE::PinkGhostMovementWhenPowerPill),
                        SpriteName::Blue => tasks.add(TaskCoreE::BlueGhostMovementWhenPowerPill),
                        SpriteName::Orange => tasks.add(TaskCoreE::OrangeGhostMovementWhenPowerPill),
                        _ => {},
                    }
                    
                }
            }
            self.is_reverse_ghost_direction_time(sn, subroutine_attract_state);
            self.sprites_coord_middle_of_tile[sni]+=self.sprites_face_xy_direction[sni];
            self.sprites_move_xy_direction[sni]=self.sprites_face_xy_direction[sni];
            self.sprites_ghosts_previous_orientation[sni]=self.sprites_ghosts_face_enum_direction[sni];
        }

        self.sprites_coord_xy[sni] += self.sprites_move_xy_direction[sni];
        self.sprites_current_tile_xy[sni] = Self::convert_sprite_position_to_tile_position(self.sprites_coord_xy[sni]);
    }

    // src:1ed0
    fn is_sprite_using_tunnel(&mut self, sname: SpriteName) -> bool {
        let mut tile_coord = self.sprites_coord_middle_of_tile[sname as usize];
        match tile_coord.x {
            29 => tile_coord.x = 61,
            62 => tile_coord.x = 30,
            _ => {
                if tile_coord.x > 32 && tile_coord.x < 59 {
                    return false;
                }
            }
        }
        return true;
    }

    // src:1efe, src:1f25, src:1f4c, src:1f73
    fn is_reverse_ghost_direction_time(&mut self, sn: SpriteName, subroutine_attract_state: u8) {
        let sni = sn as usize;

        if self.change_orientation_flag[sni] {
            self.change_orientation_flag[sni] = false;

            let dir = self.sprites_ghosts_previous_orientation[sni];
            self.sprites_ghosts_face_enum_direction[sni] = match dir {
                Direction::Right => Direction::Left,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Up => Direction::Down,
            };

            let p = DIRECTION_TILE_MOVEMENTS[dir as usize];
            self.sprites_face_xy_direction[sni] = p;
            if subroutine_attract_state == 34 {
                self.sprites_move_xy_direction[sni] = p;
                self.sprites_ghosts_previous_orientation[sni] = dir;
            }
        }
    }

    // src:2018
    fn convert_sprite_position_to_tile_position(p: Point) -> Point {
        Point::new(30 + p.x/8, 32 + p.y/8)
    }

    // src:2052
    // (x, y) -> screen => no operation...

    // src:24c9
    pub fn clears_all_pills(&mut self) {
        self.state_player[self.current_player].is_pill_present = [255; 30];    // 1 = true
        self.state_player[self.current_player].power_pills_data_entries = [TileId::Pill5; 4];
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

    /* MEMORY_MAP: program_rom2 **********************************************/

    // src:946a
    pub fn get_current_maze_table(&mut self) -> &[ [u8; 28]; 36 ] {
        self.get_data_from_current_level(&MAZE)
    }

    // src:949c
    pub fn t03_power_pills_draw(&mut self, hwvideo: &mut GameHwVideo) {
        let power_pill = self.get_data_from_current_level(&POWER_PILL);
        for i in 0..4 {
            let p = power_pill[i];
            let t = self.state_player[self.current_player].power_pills_data_entries[i];
            hwvideo.put_screen_tile(Point::new(p.0 as i32,p.1 as i32), t);
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

        let mut n = self.state_player[self.current_player].level;
        while n > 12 {
            n = n - (13-5);     // 5 <= n <= 12
        }

        let index = MAP_ORDER_TABLE[n];

        data_lookup_table[index as usize]
    }

    /// HACK: original version use colorram bit6 to configure tile like a slowdown tile for ghost
    /// see src:95c3
    fn is_tunnel_slowdown(&self, x: u8, y: u8) -> bool {
        match self.state_player[self.current_player].level {
            0 | 1 => {
                // slow down for maze on level 1 and 2 :
                // src: 8b3d
                const tunnel_slow_down_1_2:[(u8,u8); 16] = [
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


}
