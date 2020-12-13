use std::collections::VecDeque;
use embedded_graphics::prelude::*;
use num_traits::FromPrimitive;

use crate::mspacmab_data_maze::PELLET;
use crate::sprite::SpriteId;
use crate::text::TextId;
use crate::tile::TileId;
use crate::palette::ColorE;

use crate::game::Game;
use crate::game::Direction;
use crate::game::MainStateE;
use crate::game_attract::GameAttract;

use crate::hardware::Bonus;
use crate::hardware::Coinage;

pub enum ScreenPart {
    All=0,
    Maze=1,
}

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
    ResetGhostHomeCounter,                  //  5 src:268b
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
    ClearsSprites,                          // 19 src:2a35
    SetupConfigFromDipSwitches,             // 20 src:26d0 (void)
    UpdateScreenPillConfigToVideoRam,       // 21 src:2487
    IncreaseMainSubroutineNumber,           // 22 src:23e8
    PacmanAiMovementWhenAttract,            // 23 src:28e3
    ResetThenPrintPlayersScore,             // 24 src:2ae0 (void)
    UpdateScoreThenDraw,                    // 25 src:2a5a
    DrawRemainingLivesBottomLeftScreen,     // 26 src:2b6a
    DrawFruitsBottomRightScreen,            // 27 src:2bea
    DrawTextOrGraphics(TextId, bool),       // 28 src:95e3 (TextId textid, bool clear)
    DrawDrawCreditQty,                      // 29 src:2ba1
    ClearFruitAndPacmanPosition,            // 30 src:2675 (void)
    DrawExtraLifePoints,                    // 31 src:26b2
}

pub trait GameTask {
    fn task_new() -> VecDeque<TaskCoreE>;
    fn idle(&mut self) -> bool;
    fn setup_config_from_dip_switches(&mut self);
}

impl GameTask for Game {
    fn task_new() -> VecDeque<TaskCoreE> {
        VecDeque::new()
    }

    // src:238d
    fn idle(&mut self) -> bool {
        // println!("idle");
        match self.tasks.pop_front() {
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
                            _ => match self.subroutine_attract_state {
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

                            // pacman_tile_pos_in_attract_and_cut_scenes
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
                    TaskCoreE::SetGameToAttractMode => {
                        println!("TaskCoreE::SetGameToAttractMode");
                        self.mode = MainStateE::Attract;
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


}