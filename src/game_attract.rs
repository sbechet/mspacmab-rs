use std::collections::VecDeque;
use embedded_graphics::prelude::*;

use crate::palette::ColorE;
use crate::sprite::SpriteId;
use crate::text::TextId;
use crate::tile::TileId;

use crate::hardware::{ HardwareInput, HardwareOutput };
use crate::game_hw_video::{ GameHwVideo, SpriteElement, ScreenPart };
use crate::game_hw_sound::SoundChannels;
use crate::game_task::{ GameTask, TaskCoreE };
use crate::game_counter::CurrentTime;
use crate::game_task_timed::{ TaskTimedNameE, GameTaskTimed };
use crate::game_playing::{ GamePlaying, KillingGhostState };
use crate::game::MainStateE;

use crate::mspacmab_data_animation::{ ANIMATION, AnimationE, Instruction, NO_DATA, NO_SPRITES };

pub struct GameAttract {
    // src:4e02
    pub subroutine_attract_state: u8,  // 0.. 16 case w/ 34

    // src:4f00
    pub animation_enable: bool,
    // src:4f01
    flashing_bulbs_counter: u8,
    // src:4f02
    animation_current: [ (&'static [Instruction], usize) ; 6],
    // src:4f0f
    animation_cmd_table_sprite_index: [ u8; 6],
    // src:4f17
    animation_cmd_table_repeat: [ u8; 6],
    // src:4f1f
    animation_cmd_table_stop:  [bool; 6],
    // src:4f2e
    animation_cmd_table_coord: [ (i8,i8); 6],
    // src:4f3e
    animation_cmd_table_sprite: [ &'static [SpriteId]; 6],
    // src:4f4e
    // pub animation_cmd_table_color: [ u8; 6],
    // src:4f2e, src:4f4e
    // pub animation_cmd_sprite: [SpriteElement; 6],
}

impl GameAttract {
    pub fn new() -> GameAttract {
        GameAttract {
            subroutine_attract_state: 0,
            // Is set to true during intermissions and parts of the attract mode, otherwise false
            animation_enable: false,
            flashing_bulbs_counter: 0,
            animation_current: [ (&NO_DATA,0); 6],
            animation_cmd_table_sprite_index: [ 0; 6],
            animation_cmd_table_repeat: [ 0; 6],
            animation_cmd_table_stop:  [false; 6],
            animation_cmd_table_coord: [ (0,0); 6],
            animation_cmd_table_sprite: [ &NO_SPRITES; 6],
        }
    }


    // src:058e
    fn tt02_increase_subroutine_attract_state(&mut self) {
        self.subroutine_attract_state += 1;
    }

    // src:3556
    fn animation_code_assert_coord(c:i8) -> (i8, i8) {
        let mut h = c>>4;    // for real coord
        let mut l = c;  // intermediate
        if c < 0 {
            /* arrive here when ghost is moving up the left side of the marquee */
            l = (l as u8 | 0xf0) as i8;
            h += 1;
        } else {
            l &= 0x0f;
        }
        return (h,l);
    }

    // src:3611
    fn animation_init(&mut self, id:AnimationE,
        hwsound: &mut SoundChannels, playing: &mut GamePlaying) {
        if self.subroutine_attract_state == 0 {
            hwsound.channel[0].set_wave(2);
            hwsound.channel[1].set_wave(2);
        }
        self.animation_enable = true;
        self.animation_current = *ANIMATION[id as usize];
        playing.number_of_ghost_killed_but_no_collision_for_yet = KillingGhostState::KillRed;
        playing.man_dead_animation_state = 0;
        self.animation_cmd_table_stop = [false; 6];
    }
    
    // main routine to handle intermissions and attract mode ANIMATIONS
    // src:349c
    pub fn animation(&mut self, id:AnimationE,
                    timed_task: &mut GameTaskTimed,
                    tasks: &mut GameTask, 
                    hwvideo: &mut GameHwVideo, 
                    hwsound: &mut SoundChannels,
                    playing: &mut GamePlaying) {
        if self.animation_enable == false {
            self.animation_init(id, hwsound, playing);
        }

        // sprite index
        for index in 0..6 {
            let (arr, mut ind) = self.animation_current[index];
            match arr[ind] {
                Instruction::CmdFFStop => {},
                _ => print!("animation: {:?}", arr[ind]),
            }
            match arr[ind] {
                Instruction::CmdF0Update(a,b,c) => {
                    /* for value == #F0 */
                    let (ah, al) = Self::animation_code_assert_coord(a + self.animation_cmd_table_coord[index].0);
                    let (bh, bl) = Self::animation_code_assert_coord(b + self.animation_cmd_table_coord[index].1);
                    self.animation_cmd_table_coord[index].0 = al;
                    self.animation_cmd_table_coord[index].1 = bl;
                    playing.sprite[index].p += Point::new(-ah as i32, bh as i32);
                    // print!("(ah, al)=({},{}), (bh, bl)=({},{})", ah, al, bh, bl);
                    // print!(", p:{:?}, t:{:?}", playing.sprite[index].p, self.animation_cmd_table_coord[index]);

                    let mut sprite_index = self.animation_cmd_table_sprite_index[index] + 1;
                    let mut sprite_id = self.animation_cmd_table_sprite[index][sprite_index as usize];
                    if sprite_id == SpriteId::CodeForNoSprite {
                        sprite_index = 0;   // loop sprite
                        sprite_id = self.animation_cmd_table_sprite[index][sprite_index as usize];
                    }
                    self.animation_cmd_table_sprite_index[index] = sprite_index;

                    playing.sprite[index].c = c;

                    /* flip x and y in cocktail mode */
                    if playing.cocktail && playing.current_player != 0 {
                        playing.sprite[index].s = playing.sprite[index].s.flip_xy();
                    }

                    /* select sprite */
                    playing.sprite[index].s = sprite_id;

                    /* repeat */
                    self.animation_cmd_table_repeat[index] -= 1;
                    // print!(", repeat: {}", self.animation_cmd_table_repeat[index]);
                    if self.animation_cmd_table_repeat[index] == 0 {
                        ind += 1;
                    }

                },
                Instruction::CmdF1SetPosition(x,y) => {
                    /* for value == #F1 */
                    // TODO: Check correct values
                    // +8 because middle of sprite?
                    let x2:i32 = 224-(x as i32)+8;
                    let y2:i32 = (y as i32)+8;
                    print!(" -> ({},{})", x2, y2);
                    playing.sprite[index].p = Point::new(x2 as i32,y2 as i32);
                    ind += 1;

                },
                Instruction::CmdF2SetRepeat(a) => {
                    /* For value = #F2 */
                    self.animation_cmd_table_repeat[index] = a;
                    ind += 1;
                },
                Instruction::CmdF3SetSprite(s) => {
                    /* For value == #F3 */
                    self.animation_cmd_table_sprite_index[index] = 0;
                    self.animation_cmd_table_sprite[index] = s;
                    ind += 1;
                },
                Instruction::CmdF5Play(n) => {
                    /* For value == #F5 */
                    hwsound.effect[2].num = n;
                    ind += 1;
                },
                Instruction::CmdF6Pause => {
                    /* For value == #F6 */
                    print!("({})", self.animation_cmd_table_repeat[index]);
                    self.animation_cmd_table_repeat[index] -= 1;
                    if self.animation_cmd_table_repeat[index] == 0 {
                        ind += 1;
                    }
                },
                Instruction::CmdF7ShowAct => {
                    /* For value == #F7 */
                    tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Space, false));
                    ind += 1;
                },
                Instruction::CmdF8ClearAct => {
                    /* For value == #F8 */
                    // blank out the character where the 'ACT' # was displayed
                    hwvideo.put_screen_tile(Point::new(8,14), TileId::Space);
                    ind += 1;
                },
                Instruction::CmdFFStop => {
                    /* For value == #FF  */
                    self.animation_cmd_table_stop[index] = true;
                    ind = 0;
                    /* check animation end */
                    if self.animation_cmd_table_stop[0] &&
                    self.animation_cmd_table_stop[1] &&
                    self.animation_cmd_table_stop[2] &&
                    self.animation_cmd_table_stop[3] &&
                    self.animation_cmd_table_stop[4] &&
                    self.animation_cmd_table_stop[5] {
                        println!(", End of animation");
                        if self.subroutine_attract_state == 0 {
                            timed_task.add(CurrentTime::LessTenth, 5, TaskTimedNameE::IncreaseSubroutinePlayingState);
                        }
                        self.animation_enable = false;
                        self.subroutine_attract_state += 1;
                        return;
                    }
                },
            }
            self.animation_current[index].1 = ind;
            match arr[ind] {
                Instruction::CmdFFStop => {},
                _ => println!(),
            }
        }
    }

    // src:3e5c
    pub fn execute_attract_task_state_patch(&mut self,
            timed_task: &mut GameTaskTimed,
            tasks: &mut GameTask,
            hwvideo: &mut GameHwVideo, 
            hwsound: &mut SoundChannels,
            hwinput: &HardwareInput,
            hwoutput: &mut HardwareOutput,
            playing: &mut GamePlaying,
            main_state: &MainStateE) {

        if self.subroutine_attract_state != 16 {
            // PATCH: must remotimed_tasksve 0 attract_state else maze cleared but 
            // flashing bulbs fn started on recent hardware! 
            if self.subroutine_attract_state != 0 {    
                self.flashing_bulbs_around_the_marquee(hwvideo);
            }
        }
        match self.subroutine_attract_state {
            0 => {
                // src:045f
                // attract_mode_prepare_screen
                tasks.add(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                tasks.add(TaskCoreE::SelectMazeColor(0));
                tasks.add(TaskCoreE::ResetSpritesToDefaultValues(true));
                tasks.add(TaskCoreE::ClearFruitAndPacmanPosition);
                // src:0585
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::MsPacman, false));
                timed_task.add(CurrentTime::LessTenth, 10, TaskTimedNameE::IncreaseSubroutineAttractState);
                // _then_ src:058e!
                self.tt02_increase_subroutine_attract_state();
            },
            1 => {
                // src:3e96
                // attract_mode_draw_the_midway_logo_and_copyright
                self.draw_the_midway_logo_and_copyright(tasks, hwvideo);
                self.tt02_increase_subroutine_attract_state();
            }
            2 => {
                // src:3e8b
                // attract_mode_display_MS_pacman
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::MsPacman, false));
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
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::With, false));
                self.tt02_increase_subroutine_attract_state();
            },
            5 => {
                // src:3e9c
                // attract_mode_display_Blinky
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Blinky, false));
                self.tt02_increase_subroutine_attract_state();
            },
            6 => {
                // src:3483
                // attract_mode_move_Blinky_around
                self.animation(AnimationE::Ghost1, timed_task, tasks, hwvideo, hwsound, playing);
            },
            7 => {
                // src:3ea2
                // attract_mode_clear_with_display_Pinky
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Space, false));
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Pinky, false));
                self.tt02_increase_subroutine_attract_state();
            },
            8 => {
                // src:3488
                // attract_mode_move_Pinky_across
                self.animation(AnimationE::Ghost2, timed_task, tasks, hwvideo, hwsound, playing);
            },
            9 => {
                // src:3eab
                // attract_mode_display_Inky
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Inky, false));
                self.tt02_increase_subroutine_attract_state();
            },
            10 => {
                // src:348d
                // attract_mode_move_Inky_across
                self.animation(AnimationE::Ghost3, timed_task, tasks, hwvideo, hwsound, playing);
            },
            11 => {
                // src:3eb1
                // attract_mode_display_Sue
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Sue, false));
                self.tt02_increase_subroutine_attract_state();
            },
            12 => {
                // src:3492
                // attract_mode_move_Sue_across
                self.animation(AnimationE::Ghost4, timed_task, tasks, hwvideo, hwsound, playing);
            },
            13 => {
                // src:3ec3
                // attract_mode_display_Starring
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Starring, false));
                self.tt02_increase_subroutine_attract_state();
            },
            14 => {
                // src:3eb7
                // attract_mode_display_Ms_pacman
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::MsPacMan, false));
                self.tt02_increase_subroutine_attract_state();
            },
            15 => {
                // src:3497
                // attract_mode_move_mspacman_across
                self.animation(AnimationE::Man, timed_task, tasks, hwvideo, hwsound, playing);
            },
            16 => {
                // src:3ec9
                // attract_mode_start_attract_mode_where_mspacman_plays_herself
                playing.state_player[playing.current_player].real_number_of_lives = 0;
                // src:057c
                playing.execute_playing_task_state(timed_task, tasks, hwvideo, hwsound, hwinput, hwoutput, main_state, self.subroutine_attract_state);
            },
        _ => {},
        }
    }



    // This sub controls the flashing bulbs around the marquee in the attract screen 
    // src:3ed0
    fn flashing_bulbs_around_the_marquee(&mut self,
                    hwvideo: &mut GameHwVideo) {
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
                hwvideo.put_screen_tile(p, TILE1[j]);
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
                let v_old = hwvideo.get_screen(p_old);
                hwvideo.put_screen_tile(p, TILE2[j]);
                hwvideo.put_screen_tile(p_old, v_old.0.next_flashing().unwrap());
            }
        }
    }

    // draws title screen logo and text (sets as tasks).
    // src:9642
    pub fn draw_the_midway_logo_and_copyright(&mut self, tasks: &mut GameTask, hwvideo: &mut GameHwVideo) {
        tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::CMidwayMfgCo, false));
        tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Year19801981, false));

        let x = 6;
        let y = 28;
        let c= ColorE::Red;
    
        hwvideo.put_screen( Point::new(x+0,y+3), TileId::MidwayLogoLine11,  c);
        hwvideo.put_screen( Point::new(x+1,y+3), TileId::MidwayLogoLine12,  c);
        hwvideo.put_screen( Point::new(x+2,y+3), TileId::MidwayLogoLine13,  c);
        hwvideo.put_screen( Point::new(x+3,y+3), TileId::MidwayLogoLine14,  c);
    
        hwvideo.put_screen( Point::new(x+0,y+2), TileId::MidwayLogoLine21,  c);
        hwvideo.put_screen( Point::new(x+1,y+2), TileId::MidwayLogoLine22,  c);
        hwvideo.put_screen( Point::new(x+2,y+2), TileId::MidwayLogoLine23,  c);
        hwvideo.put_screen( Point::new(x+3,y+2), TileId::MidwayLogoLine24,  c);
    
        hwvideo.put_screen( Point::new(x+0,y+1), TileId::MidwayLogoLine31,  c);
        hwvideo.put_screen( Point::new(x+1,y+1), TileId::MidwayLogoLine32,  c);
        hwvideo.put_screen( Point::new(x+2,y+1), TileId::MidwayLogoLine33,  c);
        hwvideo.put_screen( Point::new(x+3,y+1), TileId::MidwayLogoLine34,  c);
    
        hwvideo.put_screen( Point::new(x+0,y  ), TileId::MidwayLogoLine41,  c);
        hwvideo.put_screen( Point::new(x+1,y  ), TileId::MidwayLogoLine42,  c);
        hwvideo.put_screen( Point::new(x+2,y  ), TileId::MidwayLogoLine43,  c);
        hwvideo.put_screen( Point::new(x+3,y  ), TileId::MidwayLogoLine44,  c);
    }
}