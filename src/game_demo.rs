use embedded_graphics::prelude::*;

use crate::palette::ColorE;
use crate::text::TextId;
use crate::tile::TileId;

use crate::game::Game;
use crate::game_task::{ TaskCoreE, ScreenPart };
use crate::game_counter::CurrentTime;
use crate::game_task_timed::{ TaskTimedNameE, GameTaskTimed };

pub trait GameDemo {
    fn tt02_increase_subroutine_demo_state(&mut self);
    fn execute_demo_task_state_patch(&mut self);
    fn flashing_bulbs_around_the_marquee(&mut self);
    fn draw_the_midway_logo_and_copyright(&mut self);
}

impl GameDemo for Game {

    // src:058e
    fn tt02_increase_subroutine_demo_state(&mut self) {
        self.subroutine_demo_state += 1;
    }

    // src:3e5c
    fn execute_demo_task_state_patch(&mut self) {

        if self.subroutine_demo_state != 16 {
            // PATCH: must remove 0 demo_state else maze cleared but 
            // flashing bulbs fn started on recent hardware! 
            if self.subroutine_demo_state != 0 {    
                self.flashing_bulbs_around_the_marquee();
            }
        }

        match self.subroutine_demo_state {
            0 => {
                // src:045f
                // demo_mode_prepare_screen
                self.task.add_to_task_list(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                self.task.add_to_task_list(TaskCoreE::SelectMazeColor(0));
                self.task.add_to_task_list(TaskCoreE::ResetSpritesToDefaultValues(false));
                self.task.add_to_task_list(TaskCoreE::ClearFruitAndPacmanPosition);
                // src:0585
                self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::MsPacman, false));
                self.timed_task_add(CurrentTime::LessTenth, 10, TaskTimedNameE::IncreaseSubroutineDemoState);
                // _then_ src:058e!
                self.tt02_increase_subroutine_demo_state();
            },
            1 => {
                // src:3e96
                // demo_mode_draw_the_midway_logo_and_copyright
                self.draw_the_midway_logo_and_copyright();
                self.tt02_increase_subroutine_demo_state();
            }
            2 => {
                // src:3e8b
                // demo_mode_display_MS_pacman
                self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::MsPacman, false));
                // PATCH: Maybe a bug in original code?
                // self.flashing_bulbs_counter = 96;
                self.tt02_increase_subroutine_demo_state();
            }
            3 => {
                // src:000c
                // RET
                // Here we wait for timer
            },
            4 => {
                // src:3ebd
                // demo_mode_display_with
                self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::With, false));
                self.tt02_increase_subroutine_demo_state();
            },
            5 => {
                // src:3e9c
                // demo_mode_display_Blinky
                self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::Blinky, false));
                self.tt02_increase_subroutine_demo_state();
            },
            6 => {
                // src:3483
                // demo_mode_move_Blinky_around
                //intermissions_and_attract_mode_animation_main_routine(0x24);
            },
            _ => {},
        }
            // TODO
        /*
        // match self.subroutine_demo_state {
            demo_mode_clear_with_display_Pinky
            demo_mode_move_Pinky_across
            demo_mode_display_Inky
            demo_mode_move_Inky_across
            demo_mode_display_Sue
            demo_mode_move_Sue_across
            demo_mode_display_Starring
            demo_mode_display_Ms_pacman
            demo_mode_move_mspacman_across
            demo_mode_start_demo_mode_where_mspacman_plays_herself
        // }
        */
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
        self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::CMidwayMfgCo, false));
        self.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::Year19801981, false));

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