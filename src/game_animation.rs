use embedded_graphics::prelude::*;

use crate::game::Game;
use crate::game_task::TaskCoreE;
use crate::game_task_timed::{ TaskTimedNameE, TaskTimedE };
use crate::game_counter::CurrentTime;
use crate::sprite::SpriteId;
use crate::text::TextId;
use crate::tile::TileId;
use crate::palette::ColorE;

pub enum AnimationE {
    Ghost1=0,
}
/*
pub enum AnimationE {
    Intermission1=0,
    Intermission2=1,
    Intermission3=2,
    Intermission4=3,
    Ghost1=4,
    Ghost2=5,
    Ghost3=6,
    Ghost4=7,
    MsPacman=8,
    None=9,
}*/

const ANIMATION: [ &'static [ (&'static [Instruction], usize) ; 6] ; 1] = [
    &GHOST1,
];

pub enum Instruction {
    CmdF0(u8, u8, ColorE),  // repeat this N times, perhaps?
    CmdF1SetPosition(u8, u8),    // position
    CmdF2SetDelay(u8),
    CmdF3SetSprite(&'static [SpriteId]),    // switch to the specified sprite code table
    CmdF4Nop,               // No operation
    CmdF5Play(u8),          // play a sound
    CmdF6Pause,             // Pause
    CmdF7ShowAct,
    CmdF8ClearAct,          // Clear the act # from the screen
    CmdFFStop,
}


const NO_ANIMATION: [ (&'static [Instruction], usize) ; 6] = [
    (&NO_DATA, 0), (&NO_DATA, 0), (&NO_DATA, 0),
    (&NO_DATA, 0), (&NO_DATA, 0), (&NO_DATA, 0)
];

/// attract mode 1st ghost
const GHOST1: [ (&'static [Instruction], usize) ; 6] = [
    (&DATA_FOR_ATTRACT_MODE_1ST_GHOST, 0),  // ? red
    (&NO_DATA, 0),                          // ? pink
    (&NO_DATA, 0),                          // ? blue
    (&NO_DATA, 0),                          // ? orange
    (&NO_DATA, 0),                          // ? man
    (&NO_DATA, 0)                           // ? fruit
];

const DATA_FOR_ATTRACT_MODE_1ST_GHOST: [Instruction; 10] = [
    Instruction::CmdF1SetPosition(0, 0x94),  // (0, 148)
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_LEFT),
    Instruction::CmdF2SetDelay(0x70),
    Instruction::CmdF0(0x10, 0x00, ColorE::Red),
    Instruction::CmdF2SetDelay(0x50),
    Instruction::CmdF0(0x10, 0x00, ColorE::Red),
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_UP),
    Instruction::CmdF2SetDelay(0x48),
    Instruction::CmdF0(0x00, 0xF0, ColorE::Red),
    Instruction::CmdFFStop,
];

pub const NO_DATA: [ Instruction; 1] = [ Instruction::CmdFFStop ];
pub const NO_SPRITES: [SpriteId; 0] = [];

const GHOST_EYES_LOOKING_LEFT: [SpriteId; 7] = [ SpriteId::GhostLeft1,SpriteId::GhostLeft1,SpriteId::GhostLeft1,SpriteId::GhostLeft2,SpriteId::GhostLeft2,SpriteId::GhostLeft2, SpriteId::CodeForNoSprite ];
const GHOST_EYES_LOOKING_UP: [SpriteId; 7] = [ SpriteId::GhostUp1, SpriteId::GhostUp1, SpriteId::GhostUp1, SpriteId::GhostUp2, SpriteId::GhostUp2, SpriteId::GhostUp2, SpriteId::CodeForNoSprite ];

pub struct GameAnimationT {
    // src:4f00
    on: bool,                                            
    // src:4f02
    animation_current: [ (&'static [Instruction], usize) ; 6],
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
}

impl GameAnimationT {
    pub fn new() -> Self {
        GameAnimationT {
            on: false,
            animation_current: NO_ANIMATION,
            animation_cmd_table_sprite_index: [ 0; 8],
            animation_cmd_table_delay: [ 0; 8],
            animation_cmd_table_stop: [true; 6],
            animation_cmd_table_f0_loop: [ (0,0); 6 ],
            animation_cmd_table_sprite: [ &NO_SPRITES; 8],
        }
    }
}

pub trait GameAnimation {
    fn animation_init(&mut self, id:AnimationE);
    fn animation(&mut self, id:AnimationE);

    // src:3556
    fn  animation_code_f0_loop_sub_stuff(v: i8) -> i8 {
        if v < 0 {
            /* arrive here when ghost is moving up the left side of the marquee */
            (v as u8 | 0xf0) as i8
        } else {
            v & 0x0f
        }
    }

}

impl GameAnimation for Game {

    // src:3611
    fn animation_init(&mut self, id:AnimationE) {
        if self.subroutine_attract_state == 0 {
            self.hwsound.channel[0].set_wave(2);
            self.hwsound.channel[1].set_wave(2);
        }
        self.animation_enable = true;
        self.animation_current = *ANIMATION[id as usize];
        self.number_of_ghost_killed_but_no_collision_for_yet = 1;
        self.ghost_eat_ability = false;
        self.animation_cmd_table_stop = [false; 6];
    }
    
    // main routine to handle intermissions and attract mode ANIMATIONS
    // src:349c
    fn animation(&mut self, id:AnimationE) {
        if self.animation_enable == false {
            self.animation_init(id);
        }

        // sprite index
        for index in 5..0 {
            let (arr, mut ind) = self.animation_current[index];
            match arr[ind] {
                Instruction::CmdF0(a,b,c) => {
                    /* for value == #F0 */
                    let coord = self.animation_cmd_table_f0_loop[index];
                    let a1 = Self::animation_code_f0_loop_sub_stuff(a as i8 + coord.0);
                    let b1 = Self::animation_code_f0_loop_sub_stuff(b as i8 + coord.1);
                    self.animation_cmd_table_f0_loop[index] = (a1, b1);
                    self.sprite[index].p += Point::new(a1 as i32, b1 as i32);

                    let mut sprite_index = self.animation_cmd_table_sprite_index[index] + 1;
                    if self.animation_cmd_table_sprite[index][sprite_index as usize] == SpriteId::CodeForNoSprite {
                        sprite_index = 0;
                    }
                    self.animation_cmd_table_sprite_index[index] = sprite_index;

                    // ?????
                    // self.sprite_flip_id_color_t_ram_4f4e[index].c = c; ?
                    self.sprite[index].c = c;
                    
                    /* reverse coord in cocktail mode */
                    // TODO
                    // if self.cocktail_mode & self.current_player_number != 0 {
                    //     animation_code = animation_code ^ 0xc0;
                    // }
                    // dest_xy[-1] = animation_code;

                    /* delay */
                    self.animation_cmd_table_delay[index] -= 1;
                    if self.animation_cmd_table_delay[index] != 0 {
                        ind += 1;
                    }

                },
                Instruction::CmdF1SetPosition(x,y) => {
                    /* for value == #F1 */
                    self.sprite[index].p = Point::new(x as i32,y as i32);
                    ind += 1;

                },
                Instruction::CmdF2SetDelay(a) => {
                    /* For value = #F2 */
                    self.animation_cmd_table_delay[index] = a;
                    ind += 1;
                },
                Instruction::CmdF3SetSprite(a) => {
                    /* For value == #F3 */
                    self.animation_cmd_table_sprite_index[index] = 0;
                    self.animation_cmd_table_sprite[index] = a;
                    ind += 1;
                },
                Instruction::CmdF4Nop => {
                    /* For value == #F4 */
                    ind += 1;
                },
                Instruction::CmdF5Play(a) => {
                    /* For value == #F5 */
                    self.hwsound.effect[2].num = a;
                },
                Instruction::CmdF6Pause => {
                    /* For value == #F6 */
                    self.animation_cmd_table_delay[index] -= 1;
                    if self.animation_cmd_table_delay[index] == 0 {
                        ind += 1;
                    }
                },
                Instruction::CmdF7ShowAct => {
                    /* For value == #F7 */
                    self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Space, false));
                    ind += 1;
                },
                Instruction::CmdF8ClearAct => {
                    /* For value == #F8 */
                    // blank out the character where the 'ACT' # was displayed
                    self.hwvideo.put_screen_tile(Point::new(8,14), TileId::Space);
                    ind += 1;
                },
                Instruction::CmdFFStop => {
                    /* For value == #FF  */
                    self.animation_cmd_table_stop[index] = true;

                    /* check animation end */
                    if self.animation_cmd_table_stop[0] &&
                    self.animation_cmd_table_stop[1] &&
                    self.animation_cmd_table_stop[2] &&
                    self.animation_cmd_table_stop[3] &&
                    self.animation_cmd_table_stop[4] &&
                    self.animation_cmd_table_stop[5] {
                        ind = 0;
                        if self.subroutine_attract_state == 0 {
                            // TODO: Howto?
                            // self.timed_tasks.timed_task_add(CurrentTime::Second, 5, TaskTimedNameE::IncreaseSubroutinePlayingState);
                            let task = TaskTimedE {
                                unit: CurrentTime::Second,
                                counter: 5,
                                task: TaskTimedNameE::IncreaseSubroutinePlayingState,
                            };
                            self.timed_tasks.push_back(task);
                    
                        }
                        self.animation_enable = false;
                        self.subroutine_attract_state += 1;                      
                    }
                },
            }
            self.animation_current[index].1 = ind;

        }
    }
}