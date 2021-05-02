use std::collections::VecDeque;
use embedded_graphics::prelude::*;
use num_traits::FromPrimitive;

use crate::hardware::HardwareInput;
use crate::game_hw_sound::SoundChannels;
use crate::game_hw_video::{ GameHwVideo, ScreenPart, WIDTH, HEIGHT };

use crate::credits::Credits;
use crate::score::Score;
use crate::mspacmab_data_fruit::FRUIT;
use crate::mspacmab_data_maze::PELLET;
use crate::sprite::SpriteId;
use crate::text::TextId;
use crate::tile::TileId;
use crate::palette::ColorE;

use crate::game::MainStateE;
use crate::game_attract::GameAttract;
use crate::game_playing::{ GamePlaying, SpriteName, GhostSubState };

use crate::hardware::Bonus;
use crate::ghost_difficulty::{ GhostDifficulty, GHOST_DIFFICULTY };


/*
    Param(u8),
    Text(TextId),
    Color(u8),

*/
pub enum TaskCoreE {
    ClearWholeScreenOrMaze(ScreenPart),     //  0 src:23ed
    SelectMazeColor(u8),                    //  1 src:24d7 (u8 playing_state) Verify:0:off, 1:playing, 2:flashing (xref:3e2, 462, 5f9, 677, 98b, 9ec)
    DrawMaze,                               //  2 src:2419
    DrawPellets,                            //  3 src:2448
    ResetSpritesToDefaultValues(bool),      //  4 src:253d (bool game_start)
    ResetGhostHomeCounter(bool),            //  5 src:268b
    ClearColorRam,                          //  6 src:240d (void)
    SetGameToAttractMode,                   //  7 src:2698 (void)
    RedGhostAi,                             //  8 src:2730
    PinkGhostAi,                            //  9 src:276c
    BlueGhostAi,                            // 10 src:27a9
    OrangeGhostAi,                          // 11 src:27f1
    RedGhostMovementWhenPowerPill,          // 12 src:283b
    PinkGhostMovementWhenPowerPill,         // 13 src:2865
    BlueGhostMovementWhenPowerPill,         // 14 src:288f
    OrangeGhostMovementWhenPowerPill,       // 15 src:28b9
    SetupDifficulty,                        // 16 src:070e
    ClearFullDataGame,                      // 17 src:26a2
    ClearsPillsAndPowerPills,               // 18 src:24c9
    ClearPills,                             // 19 src:2a35
    SetupConfigFromDipSwitches,             // 20 src:26d0 (void)
    UpdateScreenPillConfigToVideoRam,       // 21 src:2487
    IncreaseMainSubroutineNumber,           // 22 src:23e8
    PacmanAiMovementWhenAttract,            // 23 src:28e3
    ResetThenPrintPlayersScore,             // 24 src:2ae0 (void)
    UpdateScoreThenDraw(i8),                // 25 src:2a5a (score_index)
    DrawRemainingLivesBottomLeftScreen,     // 26 src:2b6a
    DrawFruitsBottomRightScreen,            // 27 src:2bea
    DrawTextOrGraphics(TextId, bool),       // 28 src:95e3 (TextId textid, bool clear)
    DrawDrawCreditQty,                      // 29 src:2ba1
    ClearFruitAndPacmanPosition,            // 30 src:2675 (void)
    DrawExtraLifePoints,                    // 31 src:26b2
}

pub struct GameTask {
    tasks: VecDeque<TaskCoreE>,
}

impl GameTask {
    pub fn new() -> GameTask {
        GameTask {
            tasks: VecDeque::new()
        }
    }

    pub fn add(&mut self, task: TaskCoreE) {
        self.tasks.push_back(task);
    }

    // src:238d
    pub fn idle(&mut self, hwinput: &HardwareInput, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, credits: &mut Credits, score: &mut Score, game_attract: &mut GameAttract, playing: &mut GamePlaying, main_state: &mut MainStateE, main_state_init_done: &mut bool) -> bool {
        // println!("idle");
        match self.tasks.pop_front() {
            Some(action) => {
                match action {
                    // 0
                    TaskCoreE::ClearWholeScreenOrMaze(part) => {
                        println!("TaskCoreE::ClearWholeScreenOrMaze");
                        hwvideo.clear_whole_screen_or_maze(part);
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
                            _ => match game_attract.subroutine_attract_state {
                                0 | 16 => {
                                    let mut n = playing.state_player[playing.current_player].level as usize;
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
                            for x in 0..WIDTH {
                                hwvideo.put_screen_color(Point::new(x as i32, y), color);
                            }
                        }

                        // ColorFruit color for first two lines
                        for y in 0..=1 {
                            for x in 0..WIDTH {
                                hwvideo.put_screen_color(Point::new(x as i32, y), ColorE::ColorFruit);
                            }
                        }

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
                            hwvideo.put_screen_color(Point::new(14, 15), ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor);
                            // store into ghost house door (left side)
                            hwvideo.put_screen_color(Point::new(13, 15), ColorE::ColorMazeLevel14_15_16_17AndGhostsDoor);
                        }

                    },
                    //  2 src:2419
                    TaskCoreE::DrawMaze => {
                        println!("TaskCoreE::DrawMaze");
                        // Hack original data was too near hardware
                        let maze = playing.get_current_maze_table();
                        for x in 0..WIDTH {
                            for y in 0..HEIGHT {
                                let tile_id = maze[y as usize][x as usize];
                                let tile = TileId::from_u8(tile_id).unwrap();
                                hwvideo.put_screen_tile(Point::new(x as i32,y as i32), tile);
                            }
                        }

                    },
                    //  3 src:2448
                    TaskCoreE::DrawPellets => {
                        let pellet: &[ (u8,u8); 240 ] = playing.get_data_from_current_level(&PELLET);
                        self.t03_pellets_draw(hwvideo, playing, pellet);
                    },
                    //  4 src:253d
                    TaskCoreE::ResetSpritesToDefaultValues(is_end) => {
                        println!("TaskCoreE::ResetSpritesToDefaultValues");
                        playing.sprite[SpriteName::Red as usize].s = SpriteId::GhostRight1;
                        playing.sprite[SpriteName::Pink as usize].s = SpriteId::GhostRight1;
                        playing.sprite[SpriteName::Blue as usize].s = SpriteId::GhostRight1;
                        playing.sprite[SpriteName::Orange as usize].s = SpriteId::GhostRight1;
                        playing.sprite[SpriteName::Man as usize].s = SpriteId::Stork0;
                        playing.sprite[SpriteName::Fruit as usize].s = SpriteId::FruitStart;

                        playing.sprite[SpriteName::Red as usize].c = ColorE::Red;
                        playing.sprite[SpriteName::Pink as usize].c = ColorE::Pink;
                        playing.sprite[SpriteName::Blue as usize].c = ColorE::Blue;
                        playing.sprite[SpriteName::Orange as usize].c = ColorE::Orange;
                        playing.sprite[SpriteName::Man as usize].c = ColorE::Yellow;
                        playing.sprite[SpriteName::Fruit as usize].c = ColorE::Black;

                        if ! is_end {
                            playing.new_is_not_end();
                        } else {
                            playing.new_is_end();
                        }

                    },
                    // 5 src:268b
                    TaskCoreE::ResetGhostHomeCounter(b) => {
                        playing.counter_related_to_ghost_movement_inside_home = 0b0101_0101;
                        if b != true {
                            playing.ghost_substate_if_alive[SpriteName::Red as usize] = GhostSubState::GoingForMan;
                        }
                    },
                    // 6 src:240d
                    TaskCoreE::ClearColorRam => {
                        println!("TaskCoreE::ClearColorRam");
                        hwvideo.clear_color_ram();
                    },
                    // 7 src:2698
                    TaskCoreE::SetGameToAttractMode => {
                        println!("TaskCoreE::SetGameToAttractMode");
                        *main_state = MainStateE::Attract;
                        *main_state_init_done = false;
                    },
                    // 8 src:2730
                    TaskCoreE::RedGhostAi => {
// TODO
                    },
                    //  9 src:276c
                    TaskCoreE::PinkGhostAi => {
// TODO
                    },
                    // 10 src:27a9
                    TaskCoreE::BlueGhostAi => {
// TODO
                    },
                    // 11 src:27f1
                    TaskCoreE::OrangeGhostAi => {
// TODO
                    },
                    // 12 src:283b
                    TaskCoreE::RedGhostMovementWhenPowerPill => {
// TODO
                    },
                    // 13 src:2865
                    TaskCoreE::PinkGhostMovementWhenPowerPill => {
// TODO
                    },
                    // 14 src:288f
                    TaskCoreE::BlueGhostMovementWhenPowerPill => {
// TODO
                    },
                    // 15 src:28b9
                    TaskCoreE::OrangeGhostMovementWhenPowerPill => {
// TODO
                    },
                    // 16 src:070e
                    TaskCoreE::SetupDifficulty => {
                        let difficulty = playing.state_player[playing.current_player].p_difficulty_settings[0];
                        let ghost_speed_and_orientation = GHOST_DIFFICULTY[difficulty].speed_and_orientation;
                        ghost_speed_and_orientation.copy_difficulty_movement_bit_pattern(playing);

                        let out_counter = GHOST_DIFFICULTY[difficulty].out_counter;
                        // src:083a
                        playing.pink_ghost_counter_to_go_out_of_home_limit = out_counter[0];
                        playing.blue_ghost_counter_to_go_out_of_home_limit = out_counter[1];
                        playing.orange_ghost_counter_to_go_out_of_home_limit = out_counter[2];

                        let pill_counter = GHOST_DIFFICULTY[difficulty].pill_counter;
                        playing.red_ghost_remainder_of_pills_when_first_difficulty_flag_is_set = pill_counter[0];
                        playing.red_ghost_remainder_of_pills_when_second_difficulty_flag_is_set = pill_counter[1];

                        let blue_time =  GHOST_DIFFICULTY[difficulty].blue_time;
                        playing.time_the_ghosts_stay_blue_when_pacman_eats_a_big_pill = *blue_time;

                        let leaves_home_time = GHOST_DIFFICULTY[difficulty].leaves_home_time;
                        playing.number_of_units_before_ghost_leaves_home = *leaves_home_time;

                        return self.draw_fruits_bottom_right_screen(hwvideo, playing, *main_state);
                    },
                    // 17 src:26a2
                    TaskCoreE::ClearFullDataGame => {
                        *playing = GamePlaying::new();
                        credits.counter_blink_for_lights_coin_and_players = 0;
                        credits.can_led_blink = false;
                    },
                    // 18 src:24c9
                    TaskCoreE::ClearsPillsAndPowerPills => {
                        println!("TaskCoreE::ClearsPillsAndPowerPills");
                        playing.clears_all_pills();
                    }
                    // 19 src:2a35
                    TaskCoreE::ClearPills => {
                        for y in 2..=33 {
                            for x in 0..WIDTH {
                                let p = Point::new(x as i32,y as i32);
                                let tile = hwvideo.get_screen(p).0;
                                match tile {
                                    TileId::Pill1 | TileId::Pill3 | TileId::Pill5 => hwvideo.put_screen_tile(p, TileId::Space),
                                    _ => {},
                                }
                            }

                        }
                    },
                    // 20 src:26d0 (void)
                    TaskCoreE::SetupConfigFromDipSwitches => {
                        println!("TaskCoreE::SetupConfigFromDipSwitches");
                        credits.setup_config_from_dip_switches(hwinput, playing);
                    },
                    // 24 src:2ae0 (void)
                    TaskCoreE::ResetThenPrintPlayersScore => {
                        println!("TaskCoreE::ResetThenPrintPlayersScore");
                        hwvideo.put_text(TextId::HighScore);
                        score.reset_score_players();
                        score.draw_score_to_screen(hwvideo, score.score[0].0, ( 5, 1) );
                        if credits.number_of_players != 0 {
                            score.draw_score_to_screen(hwvideo, score.score[1].0, (24, 1) );
                        }
                    },
                    // 25 src:2a5a
                    // score = # of ghosts eaten +1 (2-5)
                    TaskCoreE::UpdateScoreThenDraw(score_index) => {
                        println!("TaskCoreE::UpdateScoreThenDraw");
                        score.t19_update_score_then_draw(hwvideo, hwsound, playing, credits, *main_state, score_index);
                    },
                    // 26 src:2b6a
                    TaskCoreE::DrawRemainingLivesBottomLeftScreen => {
                        println!("TaskCoreE::DrawRemainingLivesBottomLeftScreen");
                        credits.t1a_draw_remaining_lives_bottom_screen(hwvideo, playing, *main_state);
                    },
                    // 27 src:2bea
                    TaskCoreE::DrawFruitsBottomRightScreen => {
                        println!("TaskCoreE::DrawFruitsBottomRightScreen");
                        return self.draw_fruits_bottom_right_screen(hwvideo, playing, *main_state);
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
                                hwvideo.put_screen( Point::new(13,23), TileId::MspacBigUpperLeft,  ColorE::Yellow);
                                hwvideo.put_screen( Point::new(14,23), TileId::MspacBigUpperRight, ColorE::Yellow);
                                hwvideo.put_screen( Point::new(14,24), TileId::MspacBigLowerRight, ColorE::Yellow);
                                hwvideo.put_screen( Point::new(13,24), TileId::MspacBigLowerLeft,  ColorE::Yellow);
                            },
                            TextId::AdditionalAt000Pts => {
                                // src:95f6
                                game_attract.draw_the_midway_logo_and_copyright(self, hwvideo);
                                // src:95fd
                                if let Bonus::None = hwinput.bonus {
                                    final_textid = TextId::Space4;
                                }
                            },
                            TextId::Ready => {
                                // 963c
                                // clear the intermission indicator
                                game_attract.animation_enable = false;
                            },
                            _ => {},
                        }

                        if clear {
                            hwvideo.del_text(final_textid);
                        } else {
                            hwvideo.put_text(final_textid);
                        }

                    },
                    // 30 src:2675 (void)
                    TaskCoreE::ClearFruitAndPacmanPosition => {
                        println!("TaskCoreE::ClearFruitAndPacmanPosition");
                        playing.clear_fruit_and_pacman_position();
                    },
                    // 31 src:26b2
                    TaskCoreE::DrawExtraLifePoints => {
                        println!("TaskCoreE::DrawExtraLifePoints");
                        let t0 = TileId::from_u8(b'0' + credits.bonus / 10).unwrap();
                        let t1 = TileId::from_u8(b'0' + credits.bonus % 10).unwrap();
                        hwvideo.put_screen_tile(Point::new(19,24), t0);
                        hwvideo.put_screen_tile(Point::new(20,24), t1);
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

    // src:2453
    fn t03_pellets_draw(&mut self, hwvideo: &mut GameHwVideo, playing: &mut GamePlaying, pellets_coord: &[ (u8,u8); 240 ]) {
        let mut pellet_index = 0;
        for byte_index in 0..30 {
            let mut current_pills = playing.state_player[playing.current_player].is_pill_present[byte_index];
            for _bit_index in 0..8 {
                let p = pellets_coord[pellet_index];
                if current_pills >> 7 == 1 {
                    hwvideo.put_screen_tile(Point::new(p.0 as i32,p.1 as i32), TileId::Pill1);
                }
                current_pills <<= 1;
                pellet_index += 1;
            }
        }
        playing.t03_power_pills_draw(hwvideo);
    }


    // src:2b80
    fn draw_fruit_color(&mut self, hwvideo: &mut GameHwVideo, c: ColorE, p: (i32, i32) ) {
        let point = Point::new(p.0, p.1);
        hwvideo.put_screen_color(point, c);

        let point = Point::new(p.0 - 1, p.1);
        hwvideo.put_screen_color(point, c);

        let point = Point::new(p.0, p.1 + 1);
        hwvideo.put_screen_color(point, c);

        let point = Point::new(p.0 - 1, p.1 + 1);
        hwvideo.put_screen_color(point, c);
    }

    // src:2bea
    fn draw_fruits_bottom_right_screen(&mut self, hwvideo: &mut GameHwVideo, playing: &mut GamePlaying, main_state: MainStateE) -> bool {
        if let MainStateE::Attract = main_state {
            return false;   // do not update screen
        }
        let level:usize = if playing.state_player[playing.current_player].level > 7 {
            7
        } else {
            playing.state_player[playing.current_player].level as usize
        };

        let mut x=25;
        for i in 0..level {
            let f = FRUIT[i];
            hwvideo.draw_big_tile( f.0, (x, 34) );
            self.draw_fruit_color( hwvideo, f.1, (x, 34) );
            x -= 2;
        }
        for _i in level..7 {
            hwvideo.draw_big_tile_blank( (x, 34) );
            x -= 2;
        }
        return true;
    }

}