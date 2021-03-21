use embedded_graphics::prelude::*;

use crate::palette::ColorE;
use crate::text::TextId;
use crate::tile::TileId;

use crate::game::Game;
use crate::game_animation::{ AnimationE, GameAnimation };
// use crate::game_task::GameTask;
use crate::game_playing::GamePlaying;
use crate::game_task::{ TaskCoreE, ScreenPart };
use crate::game_counter::CurrentTime;
use crate::game_task_timed::{ TaskTimedNameE, GameTaskTimed };

pub trait GameAttract {
    fn tt02_increase_subroutine_attract_state(&mut self);
    fn execute_attract_task_state_patch(&mut self);
    fn flashing_bulbs_around_the_marquee(&mut self);
    fn draw_the_midway_logo_and_copyright(&mut self);
}

impl GameAttract for Game {

    // src:058e
    fn tt02_increase_subroutine_attract_state(&mut self) {
        self.subroutine_attract_state += 1;
    }

    // src:3e5c
    fn execute_attract_task_state_patch(&mut self) {

        if self.subroutine_attract_state != 16 {
            // PATCH: must remove 0 attract_state else maze cleared but 
            // flashing bulbs fn started on recent hardware! 
            if self.subroutine_attract_state != 0 {    
                self.flashing_bulbs_around_the_marquee();
            }
        }

        match self.subroutine_attract_state {
            0 => {
                // src:045f
                // attract_mode_prepare_screen
                self.tasks.push_back(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                self.tasks.push_back(TaskCoreE::SelectMazeColor(0));
                self.tasks.push_back(TaskCoreE::ResetSpritesToDefaultValues(false));
                self.tasks.push_back(TaskCoreE::ClearFruitAndPacmanPosition);
                // src:0585
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::MsPacman, false));
                self.timed_task_add(CurrentTime::LessTenth, 10, TaskTimedNameE::IncreaseSubroutineAttractState);
                // _then_ src:058e!
                self.tt02_increase_subroutine_attract_state();
            },
            1 => {
                // src:3e96
                // attract_mode_draw_the_midway_logo_and_copyright
                self.draw_the_midway_logo_and_copyright();
                self.tt02_increase_subroutine_attract_state();
            }
            2 => {
                // src:3e8b
                // attract_mode_display_MS_pacman
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::MsPacman, false));
                // PATCH: remove a small bug in original code?
                // self.flashing_bulbs_counter = 96;
                self.tt02_increase_subroutine_attract_state();
            }
            3 => {
                // src:000c
                // RET
                // Here we wait for timer
            },
            4 => {
                // src:3ebd
                // attract_mode_display_with
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::With, false));
                self.tt02_increase_subroutine_attract_state();
            },
            5 => {
                // src:3e9c
                // attract_mode_display_Blinky
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Blinky, false));
                self.tt02_increase_subroutine_attract_state();
            },
            6 => {
                // src:3483
                // attract_mode_move_Blinky_around
                self.animation(AnimationE::Ghost1);
            },
            7 => {
                // src:3ea2
                // attract_mode_clear_with_display_Pinky
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Space, false));
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Pinky, false));
                self.tt02_increase_subroutine_attract_state();
            },
            8 => {
                // src:3488
                // attract_mode_move_Pinky_across
                self.animation(AnimationE::Ghost2);
            },
            9 => {
                // src:3eab
                // attract_mode_display_Inky
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Inky, false));
                self.tt02_increase_subroutine_attract_state();
            },
            10 => {
                // src:348d
                // attract_mode_move_Inky_across
                self.animation(AnimationE::Ghost3);
            },
            11 => {
                // src:3eb1
                // attract_mode_display_Sue
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Sue, false));
                self.tt02_increase_subroutine_attract_state();
            },
            12 => {
                // src:3492
                // attract_mode_move_Sue_across
                self.animation(AnimationE::Ghost4);
            },
            13 => {
                // src:3ec3
                // attract_mode_display_Starring
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Starring, false));
                self.tt02_increase_subroutine_attract_state();
            },
            14 => {
                // src:3eb7
                // attract_mode_display_Ms_pacman
                self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::MsPacMan, false));
                self.tt02_increase_subroutine_attract_state();
            },
            15 => {
                // src:3497
                // attract_mode_move_mspacman_across
                self.animation(AnimationE::Man);
            },
            16 => {
                // src:3ec9
                // attract_mode_start_attract_mode_where_mspacman_plays_herself
                self.current_player.real_number_of_lives = 0;
                // src:057c
                self.execute_playing_task_state();
            },
        _ => {},
        }
    }

    // This sub controls the flashing bulbs around the marquee in the attract screen 
    // src:3ed0
    fn flashing_bulbs_around_the_marquee(&mut self) {
        // src:3f81
        // (7,11) -> (24, 18)
        const FLASHING_BULBS: [ [(u8, u8); 8]; 6] = [
            [(8, 18), (9, 18), (10, 18), (11, 18), (12, 18), (13, 18), (14, 18), (15, 18), ],
            [(16, 18), (17, 18), (18, 18), (19, 18), (20, 18), (21, 18), (22, 18), (23, 18), ],

            [(24, 18), (24, 17), (24, 16), (24, 15), (24, 14), (24, 13), (24, 12), (24, 11), ],

            [(23, 11), (22, 11), (21, 11), (20, 11), (19, 11), (18, 11), (17, 11), (16, 11), ],
            [(15, 11), (14, 11), (13, 11), (12, 11), (11, 11), (10, 11), (9, 11), (8, 11), ],

            [(7, 11), (7, 12), (7, 13), (7, 14), (7, 15), (7, 16), (7, 17), (7, 18), ],
        ];
        const TILE1: [TileId; 6] = [TileId::FlashingBulbsBottomWG, TileId::FlashingBulbsBottomWG, TileId::FlashingBulbsRightGW, TileId::FlashingBulbsUpGW, TileId::FlashingBulbsUpGW, TileId::FlashingBulbsLeftWG];
        const TILE2: [TileId; 6] = [TileId::FlashingBulbsBottomGW, TileId::FlashingBulbsBottomGW, TileId::FlashingBulbsRightWG, TileId::FlashingBulbsUpWG, TileId::FlashingBulbsUpWG, TileId::FlashingBulbsLeftGW];

        self.flashing_bulbs_counter += 1;
        self.flashing_bulbs_counter &= 15;

        // two step for each index using flashing_bulbs_counter lower bit
        let i: usize = self.flashing_bulbs_counter as usize >> 1;
        if self.flashing_bulbs_counter & 1 != 0 {
            for j in 0..6 {
                let c = FLASHING_BULBS[j][i];
                let p = Point::new(c.0.into(), c.1.into());
                self.hwvideo.put_screen_tile(p, TILE1[j]);
            }
        } else {
            for j in 0..6 {
                let c = FLASHING_BULBS[j][i];
                let p = Point::new(c.0.into(), c.1.into());
                let c_old = if i > 0 {
                    FLASHING_BULBS[j][i-1]
                } else {
                    FLASHING_BULBS[j][7]    // loop
                };
                let p_old = Point::new(c_old.0.into(), c_old.1.into());
                let v_old = self.hwvideo.get_screen(p_old);
                self.hwvideo.put_screen_tile(p, TILE2[j]);
                self.hwvideo.put_screen_tile(p_old, v_old.0.next_flashing().unwrap());
            }
        }
    }

    // draws title screen logo and text (sets as tasks).
    // src:9642
    fn draw_the_midway_logo_and_copyright(&mut self) {
        self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::CMidwayMfgCo, false));
        self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Year19801981, false));

        let x = 6;
        let y = 28;
        let c= ColorE::Red;
    
        self.hwvideo.put_screen( Point::new(x+0,y+3), TileId::MidwayLogoLine11,  c);
        self.hwvideo.put_screen( Point::new(x+1,y+3), TileId::MidwayLogoLine12,  c);
        self.hwvideo.put_screen( Point::new(x+2,y+3), TileId::MidwayLogoLine13,  c);
        self.hwvideo.put_screen( Point::new(x+3,y+3), TileId::MidwayLogoLine14,  c);
    
        self.hwvideo.put_screen( Point::new(x+0,y+2), TileId::MidwayLogoLine21,  c);
        self.hwvideo.put_screen( Point::new(x+1,y+2), TileId::MidwayLogoLine22,  c);
        self.hwvideo.put_screen( Point::new(x+2,y+2), TileId::MidwayLogoLine23,  c);
        self.hwvideo.put_screen( Point::new(x+3,y+2), TileId::MidwayLogoLine24,  c);
    
        self.hwvideo.put_screen( Point::new(x+0,y+1), TileId::MidwayLogoLine31,  c);
        self.hwvideo.put_screen( Point::new(x+1,y+1), TileId::MidwayLogoLine32,  c);
        self.hwvideo.put_screen( Point::new(x+2,y+1), TileId::MidwayLogoLine33,  c);
        self.hwvideo.put_screen( Point::new(x+3,y+1), TileId::MidwayLogoLine34,  c);
    
        self.hwvideo.put_screen( Point::new(x+0,y  ), TileId::MidwayLogoLine41,  c);
        self.hwvideo.put_screen( Point::new(x+1,y  ), TileId::MidwayLogoLine42,  c);
        self.hwvideo.put_screen( Point::new(x+2,y  ), TileId::MidwayLogoLine43,  c);
        self.hwvideo.put_screen( Point::new(x+3,y  ), TileId::MidwayLogoLine44,  c);
    }
}