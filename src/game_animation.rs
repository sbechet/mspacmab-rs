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
    Intermission1=0,    // 0x00
    Intermission2=1,    // 0x0C
    Intermission3=2,    // 0x18
    Ghost1=3,   // 0x24
    Ghost2=4,   // 0x30
    Ghost3=5,   // 0x3c
    Ghost4=6,   // 0x48
    Man=7,      // 0x54
}

pub const NO_SPRITES: [SpriteId; 0] = [];

const ANIMATION: [ &'static [ (&'static [Instruction], usize) ; 6] ; 8] = [
    &INTERMISSION1,
    &INTERMISSION2,
    &INTERMISSION3,
    &GHOST1,
    &GHOST2,
    &GHOST3,
    &GHOST4,
    &MAN,
];

#[derive(Debug)]
pub enum Instruction {
    CmdF0Update(i8, i8, ColorE),    // update (xseek, yseek, color) repeat times
    CmdF1SetPosition(u8, u8),   //  set position (x, y) 
    CmdF2SetRepeat(u8),  // set repeat r
    CmdF3SetSprite(&'static [SpriteId]),    // switch to the specified sprite code table
    CmdF5Play(u8),          // play a sound
    CmdF6Pause,             // Pause repeat times
    CmdF7ShowAct,
    CmdF8ClearAct,          // Clear the act # from the screen
    CmdFFStop,
}

// Isn't in original code - we must init
// const NO_ANIMATION: [ (&'static [Instruction], usize) ; 6] = [
//     (&NO_DATA, 0), (&NO_DATA, 0), (&NO_DATA, 0),
//     (&NO_DATA, 0), (&NO_DATA, 0), (&NO_DATA, 0)
// ];

/// intermission 1
// src:81f0
const INTERMISSION1: [ (&'static [Instruction], usize) ; 6] = [
    (&INTERMISSION1_0, 0),
    (&INTERMISSION1_1, 0),
    (&INTERMISSION1_2, 0),
    (&INTERMISSION1_3, 0),
    (&INTERMISSION1_4, 0),
    (&INTERMISSION1_5, 0),
];
// intermission 2
// src:81fc
const INTERMISSION2: [ (&'static [Instruction], usize) ; 6] = [
    (&INTERMISSION2_0, 0),  // src:8395
    (&INTERMISSION2_1, 0),  // src:83f0
    (&INTERMISSION2_2, 0),  // src:852b
    (&INTERMISSION2_3, 0),  // src:854a
    (&INTERMISSION1_4, 0),  // src:8569
    (&INTERMISSION1_5, 0),  // src:857c
];
// intermission 3
// src:8208
const INTERMISSION3: [ (&'static [Instruction], usize) ; 6] = [
    (&INTERMISSION3_0, 0),  // src:8451
    (&INTERMISSION3_1, 0),  // src:846d
    (&INTERMISSION3_2, 0),  // src:84cf
    (&INTERMISSION3_3, 0),  // src:84fd
    (&INTERMISSION3_4, 0),  // src:8489
    (&INTERMISSION1_5, 0),  // src:857c
];
/// attract mode 1st ghost
// src:8214
const GHOST1: [ (&'static [Instruction], usize) ; 6] = [
    (&DATA_FOR_ATTRACT_MODE_1ST_GHOST, 0),
    (&NO_DATA, 0),
    (&NO_DATA, 0),                        
    (&NO_DATA, 0),                        
    (&NO_DATA, 0),
    (&NO_DATA, 0),
];

/// attract mode 2nd ghost
// src:8220
const GHOST2: [ (&'static [Instruction], usize) ; 6] = [
    (&NO_DATA, 0),
    (&DATA_FOR_ATTRACT_MODE_2ND_GHOST, 0),
    (&NO_DATA, 0),                        
    (&NO_DATA, 0),                        
    (&NO_DATA, 0),
    (&NO_DATA, 0),
];

/// attract mode 3rd ghost
// src:822c
const GHOST3: [ (&'static [Instruction], usize) ; 6] = [
    (&NO_DATA, 0),
    (&NO_DATA, 0),
    (&DATA_FOR_ATTRACT_MODE_3RD_GHOST, 0),
    (&NO_DATA, 0),
    (&NO_DATA, 0),
    (&NO_DATA, 0),
];

/// attract mode 4th ghost
// src:8238
const GHOST4: [ (&'static [Instruction], usize) ; 6] = [
    (&NO_DATA, 0),
    (&NO_DATA, 0),
    (&NO_DATA, 0),                        
    (&DATA_FOR_ATTRACT_MODE_4TH_GHOST, 0),
    (&NO_DATA, 0),
    (&NO_DATA, 0),
];

/// attract mode mspacman
// src:8244
const MAN: [ (&'static [Instruction], usize) ; 6] = [
    (&NO_DATA, 0),
    (&NO_DATA, 0),
    (&NO_DATA, 0),                        
    (&NO_DATA, 0),                        
    (&DATA_FOR_ATTRACT_MODE_MAN, 0),
    (&NO_DATA, 0),
];

// src:8250
pub const NO_DATA: [ Instruction; 1] = [ Instruction::CmdFFStop ];

// src:8251
const INTERMISSION1_0: [Instruction; 30] = [
    Instruction::CmdF1SetPosition(0,0),                      	// ram:8251 f1 00 00   
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN1), 	// ram:8254 f3 75 86   
    Instruction::CmdF2SetRepeat(1),	// ram:8257 f2 01      
    Instruction::CmdF0Update(0,0,ColorE::ColorMazeLevel3_4_5),	// ram:8259 f0 00 00 16
    Instruction::CmdF1SetPosition(189, 82),	// ram:825d f1 bd 52   
    Instruction::CmdF2SetRepeat(40),	// ram:8260 f2 28      
    Instruction::CmdF6Pause,	// ram:8262 f6         
    Instruction::CmdF2SetRepeat(22),	// ram:8263 f2 16      
    Instruction::CmdF0Update(0,0,ColorE::ColorMazeLevel3_4_5),	// ram:8265 f0 00 00 16
    Instruction::CmdF2SetRepeat(22),	// ram:8269 f2 16      
    Instruction::CmdF6Pause,	// ram:826b f6         
    Instruction::CmdF1SetPosition(255, 84),	// ram:826c f1 ff 54   
    Instruction::CmdF3SetSprite(&MSP_WALKING_RIGHT),	// ram:826f f3 14 86   
    Instruction::CmdF2SetRepeat(127),	// ram:8272 f2 7f      
    Instruction::CmdF0Update(-16, 0, ColorE::Yellow),	// ram:8274 f0 f0 00 09
    Instruction::CmdF2SetRepeat(127),	// ram:8278 f2 7f      
    Instruction::CmdF0Update(-16, 0, ColorE::Yellow),	// ram:827a f0 f0 00 09
    Instruction::CmdF1SetPosition(0, 127), 	// ram:827e f1 00 7f   
    Instruction::CmdF3SetSprite(&MSP_WALKING_LEFT),	// ram:8281 f3 1d 86   
    Instruction::CmdF2SetRepeat(117),	// ram:8284 f2 75      
    Instruction::CmdF0Update(16, 0, ColorE::Yellow),	// ram:8286 f0 10 00 09
    Instruction::CmdF2SetRepeat(4),	// ram:828a f2 04      
    Instruction::CmdF0Update(16, -16, ColorE::Yellow),	// ram:828c f0 10 f0 09
    Instruction::CmdF3SetSprite(&MSP_WALKING_UP),	// ram:8290 f3 26 86   
    Instruction::CmdF2SetRepeat(48),	// ram:8293 f2 30      
    Instruction::CmdF0Update(0,-16, ColorE::Yellow),	// ram:8295 f0 00 f0 09
    Instruction::CmdF3SetSprite(&MSP_WALKING_LEFT),	// ram:8299 f3 1d 86   
    Instruction::CmdF2SetRepeat(16),	// ram:829c f2 10      
    Instruction::CmdF0Update(0, 0, ColorE::Yellow),	// ram:829e f0 00 00 09
    Instruction::CmdFFStop,	// ram:82a2 ff         
];

// src:82a3
const INTERMISSION1_1: [Instruction; 42] = [
    Instruction::CmdF1SetPosition(0, 0),        // ram:82a3 f1 00 00
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN2),    // ram:82a6 f3 7f 86
    Instruction::CmdF2SetRepeat(1),    // ram:82a9 f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5),        // ram:82ab f0 00 00 16
    Instruction::CmdF1SetPosition(173,82),    // ram:82af f1 ad 52
    Instruction::CmdF2SetRepeat(40),    // ram:82b2 f2 28
    Instruction::CmdF6Pause,    // ram:82b4 f6
    Instruction::CmdF2SetRepeat(22),    // ram:82b5 f2 16
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5),    // ram:82b7 f0 00 00 16
    Instruction::CmdF2SetRepeat(22),    // ram:82bb f2 16
    Instruction::CmdF6Pause,    // ram:82bd f6
    Instruction::CmdF1SetPosition(255,84),// ram:82be f1 ff 54
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_RIGHT),    // ram:82c1 f3 5c 86
    Instruction::CmdF2SetRepeat(47), // ram:82c4 f2 2f
    Instruction::CmdF6Pause,  // ram:82c6 f6
    Instruction::CmdF2SetRepeat(112),   // ram:82c7 f2 70
    Instruction::CmdF0Update(-17, 0, ColorE::Blue), // ram:82c9 f0 ef 00 05
    Instruction::CmdF2SetRepeat(116),   // ram:82cd f2 74
    Instruction::CmdF0Update(-20, 0, ColorE::Blue), // ram:82cf f0 ec 00 05
    Instruction::CmdF1SetPosition(0,127), // ram:82d3 f1 00 7f
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_LEFT), // ram:82d6 f3 63 86
    Instruction::CmdF2SetRepeat(28),    // ram:82d9 f2 1c
    Instruction::CmdF6Pause,  // ram:82db f6
    Instruction::CmdF2SetRepeat(88),    // ram:82dc f2 58
    Instruction::CmdF0Update(22, 0, ColorE::Blue),  // ram:82de f0 16 00 05
    Instruction::CmdF5Play(16), // ram:82e2 f5 10
    Instruction::CmdF2SetRepeat(6), // ram:82e4 f2 06
    Instruction::CmdF0Update(-8, -8, ColorE::Blue),   // ram:82e6 f0 f8 f8 05
    Instruction::CmdF2SetRepeat(6), // ram:82ea f2 06
    Instruction::CmdF0Update(-8, 8, ColorE::Blue), // ram:82ec f0 f8 08 05
    Instruction::CmdF2SetRepeat(6), // ram:82f0 f2 06
    Instruction::CmdF0Update(-8, -8, ColorE::Blue),   // ram:82f2 f0 f8 f8 05
    Instruction::CmdF2SetRepeat(6), // ram:82f6 f2 06
    Instruction::CmdF0Update(-8, 8, ColorE::Blue),   // ram:82f8 f0 f8 08 05
    Instruction::CmdF1SetPosition(0, 0),    // ram:82fc f1 00 00
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_HEART), // ram:82ff f3 73 86
    Instruction::CmdF2SetRepeat(1), // ram:8302 f2 01
    Instruction::CmdF0Update(0, 0, ColorE::Pink),   // ram:8304 f0 00 00 03
    Instruction::CmdF1SetPosition(127, 58), // ram:8308 f1 7f 3a
    Instruction::CmdF2SetRepeat(64),    // ram:830b f2 40
    Instruction::CmdF0Update(0, 0, ColorE::Pink),   // ram:830d f0 00 00 03
    Instruction::CmdFFStop, // ram:8311 ff
];

// src:8312
const INTERMISSION1_2: [Instruction; 21] = [
    Instruction::CmdF2SetRepeat(90),    // ram:8312 f2 5a
    Instruction::CmdF6Pause,    // ram:8314 f6
    Instruction::CmdF1SetPosition(0, 164),  // ram:8315 f1 00 a4
    Instruction::CmdF3SetSprite(&MAN_LEFT), // ram:8318 f3 41 86
    Instruction::CmdF2SetRepeat(127),   // ram:831b f2 7f
    Instruction::CmdF0Update(16, 0, ColorE::Yellow),    // ram:831d f0 10 00 09
    Instruction::CmdF2SetRepeat(127),   // ram:8321 f2 7f
    Instruction::CmdF0Update(16, 0, ColorE::Yellow),    // ram:8323 f0 10 00 09
    Instruction::CmdF1SetPosition(255, 127), // ram:8327 f1 ff 7f
    Instruction::CmdF3SetSprite(&RIGHT_PAC), // ram:832a f3 38 86
    Instruction::CmdF2SetRepeat(118),   // ram:832d f2 76
    Instruction::CmdF0Update(-16, 0, ColorE::Yellow),   // ram:832f f0 f0 00 09
    Instruction::CmdF2SetRepeat(4), // ram:8333 f2 04
    Instruction::CmdF0Update(-16, -16, ColorE::Yellow), // ram:8335 f0 f0 f0 09
    Instruction::CmdF3SetSprite(&MSP_MOVING_UP_AT_THE_END), // ram:8339 f3 4a 86
    Instruction::CmdF2SetRepeat(48),    // ram:833c f2 30
    Instruction::CmdF0Update(0,-16, ColorE::Yellow),    // ram:833e f0 00 f0 09
    Instruction::CmdF3SetSprite(&RIGHT_PAC), // ram:8342 f3 38 86
    Instruction::CmdF2SetRepeat(16),    // ram:8345 f2 10
    Instruction::CmdF0Update(0, 0, ColorE::Yellow),   // ram:8347 f0 00 00 09
    Instruction::CmdFFStop, // ram:834b ff
];

// src:834c
const INTERMISSION1_3: [Instruction; 28] = [
    Instruction::CmdF2SetRepeat(95),    // ram:834c f2 5f
    Instruction::CmdF6Pause,    // ram:834e f6
    Instruction::CmdF1SetPosition(1, 164),  // ram:834f f1 01 a4
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_LEFT), // ram:8352 f3 63 86
    Instruction::CmdF2SetRepeat(47),    // ram:8355 f2 2f
    Instruction::CmdF6Pause,    // ram:8357 f6
    Instruction::CmdF2SetRepeat(112),   // ram:8358 f2 70
    Instruction::CmdF0Update(17, 0, ColorE::Pink),  // ram:835a f0 11 00 03
    Instruction::CmdF2SetRepeat(116),   // ram:835e f2 74
    Instruction::CmdF0Update(20, 0, ColorE::Pink),  // ram:8360 f0 14 00 03
    Instruction::CmdF1SetPosition(255, 127), // ram:8364 f1 ff 7f
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_RIGHT), // ram:8367 f3 5c 86
    Instruction::CmdF2SetRepeat(28),    // ram:836a f2 1c
    Instruction::CmdF6Pause,    // ram:836c f6
    Instruction::CmdF2SetRepeat(88),    // ram:836d f2 58
    Instruction::CmdF0Update(-22, 0, ColorE::Pink), // ram:836f f0 ea 00 03
    Instruction::CmdF2SetRepeat(6), // ram:8373 f2 06
    Instruction::CmdF0Update(8, -8, ColorE::Pink),    // ram:8375 f0 08 f8 03
    Instruction::CmdF2SetRepeat(6), // ram:8379 f2 06
    Instruction::CmdF0Update(8, 8, ColorE::Pink), // ram:837b f0 08 08 03
    Instruction::CmdF2SetRepeat(6), // ram:837f f2 06
    Instruction::CmdF0Update(8, -8, ColorE::Pink),    // ram:8381 f0 08 f8 03
    Instruction::CmdF2SetRepeat(6), // ram:8385 f2 06
    Instruction::CmdF0Update(8, 8, ColorE::Pink),   // ram:8387 f0 08 08 03
    Instruction::CmdF3SetSprite(&EMPTY_SPRITE), // ram:838b f3 71 86
    Instruction::CmdF2SetRepeat(16),    // ram:838e f2 10
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5),    // ram:8390 f0 00 00 16
    Instruction::CmdFFStop, // ram:8394 ff
];

// src:8395
const INTERMISSION2_0: [Instruction; 41] = [
    Instruction::CmdF2SetRepeat(90), // f2 5a 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(255, 52), // f1 ff 34 
    Instruction::CmdF3SetSprite(&MSP_WALKING_RIGHT), // f3 14 86 
    Instruction::CmdF2SetRepeat(127), // f2 7f 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(36), // f2 24 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(104), // f2 68 
    Instruction::CmdF0Update(-40, 0, ColorE::Yellow), // f0 d8 00 09 
    Instruction::CmdF2SetRepeat(127), // f2 7f 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(24), // f2 18 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0, 148), // f1 00 94 
    Instruction::CmdF3SetSprite(&MAN_LEFT), // f3 41 86 
    Instruction::CmdF2SetRepeat(104), // f2 68 
    Instruction::CmdF0Update(40, 0, ColorE::Yellow), // f0 28 00 09 
    Instruction::CmdF2SetRepeat(127), // f2 7f 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0xfc, 0x7f), // f1 fc 7f 
    Instruction::CmdF3SetSprite(&MSP_WALKING_RIGHT), // f3 14 86
    Instruction::CmdF2SetRepeat(24), // f2 18
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(104), // f2 68 
    Instruction::CmdF0Update(-40, 0, ColorE::Yellow), // f0 d8 00 09 
    Instruction::CmdF2SetRepeat(127), // f2 7f 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(24), // f2 18 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0, 84), // f1 00 54 
    Instruction::CmdF3SetSprite(&MAN_LEFT), // f3 &8641 
    Instruction::CmdF2SetRepeat(32), // f2 20 
    Instruction::CmdF0Update(112, 0, ColorE::Yellow), // f0 70 00 09 
    Instruction::CmdF1SetPosition(255, 180), // f1 ff b4 
    Instruction::CmdF3SetSprite(&MSP_WALKING_RIGHT), // f3 &8614 
    Instruction::CmdF2SetRepeat(16), // f2 10 
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(36), // f2 24 
    Instruction::CmdF0Update(-112, 0, ColorE::Yellow), // f0 90 00 09 
    Instruction::CmdFFStop, // ff
];

// src:83f0
const INTERMISSION2_1: [Instruction; 45] = [
    Instruction::CmdF2SetRepeat(0x63), // f2 63
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0xff, 0x34), // f1 ff 34
    Instruction::CmdF3SetSprite(&RIGHT_PAC), // f3 &8638
    Instruction::CmdF2SetRepeat(0x24), // f2 24
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x18), // f2 18
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x57), // f2 57
    Instruction::CmdF0Update(-48, 0, ColorE::Yellow), // f0 d0 00 09
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x28), // f2 28
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0x00, 0x94), // f1 00 94
    Instruction::CmdF3SetSprite(&MSP_WALKING_LEFT), // f3 &861d
    Instruction::CmdF2SetRepeat(0x58), // f2 58
    Instruction::CmdF0Update(48, 0, ColorE::Yellow), // f0 30 00 09
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x24), // f2 24
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0xff, 0x7f), // f1 ff 7f
    Instruction::CmdF3SetSprite(&RIGHT_PAC), // f3 &8638
    Instruction::CmdF2SetRepeat(0x58), // f2 58
    Instruction::CmdF0Update(-48, 0, ColorE::Yellow), // f0 d0 00 09
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x20), // f2 20
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0x00, 0x54), // f1 00 54
    Instruction::CmdF3SetSprite(&MSP_WALKING_LEFT), // f3 &861d
    Instruction::CmdF2SetRepeat(0x20), // f2 20
    Instruction::CmdF0Update(112, 0, ColorE::Yellow), // f0 70 00 09
    Instruction::CmdF1SetPosition(0xff, 0xb4), // f1 ff b4
    Instruction::CmdF3SetSprite(&RIGHT_PAC), // f3 &8638
    Instruction::CmdF2SetRepeat(0x10), // f2 10
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x24), // f2 24
    Instruction::CmdF0Update(-112, 0, ColorE::Yellow), // f0 90 00 09
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdFFStop, // ff
];

// src:8451
const INTERMISSION3_0: [Instruction; 11] = [
    Instruction::CmdF2SetRepeat(0x5a), // f2 5a
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0x00, 0x60), // f1 00 60
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_STORK), // f3 &868d
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF0Update(10, 0, ColorE::ColorMazeLevel3_4_5), // f0 0a 00 16
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF0Update(16, 0, ColorE::ColorMazeLevel3_4_5), // f0 10 00 16
    Instruction::CmdF2SetRepeat(0x30), // f2 30
    Instruction::CmdF0Update(16, 0, ColorE::ColorMazeLevel3_4_5), // f0 10 00 16
    Instruction::CmdFFStop, // ff
];

// src:846d
const INTERMISSION3_1: [Instruction; 11] = [
    Instruction::CmdF2SetRepeat(0x6f), // f2 6f
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0x00, 0x60), // f1 00 60
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_STORKS), // f3 &868f
    Instruction::CmdF2SetRepeat(0x6a), // f2 6a
    Instruction::CmdF0Update(10, 0, ColorE::ColorMazeLevel3_4_5), // f0 0a 00 16
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF0Update(16, 0, ColorE::ColorMazeLevel3_4_5), // f0 10 00 16
    Instruction::CmdF2SetRepeat(0x3a), // f2 3a
    Instruction::CmdF0Update(16, 0, ColorE::ColorMazeLevel3_4_5), // f0 10 00 16
    Instruction::CmdFFStop, // ff
];

// src:8489
const INTERMISSION3_4: [Instruction; 26] = [
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN3), // f3 &8689
    Instruction::CmdF2SetRepeat(0x01), // f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF1SetPosition(0xbd, 0x62), // f1 bd 62
    Instruction::CmdF2SetRepeat(0x5a), // f2 5a
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0x05, 0x60), // f1 05 60
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_STORK_CARRIES), // f3 &8698
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF0Update(10, 0, ColorE::ColorMazeLevel3_4_5), // f0 0a 00 16
    Instruction::CmdF2SetRepeat(0x7f), // f2 7f
    Instruction::CmdF0Update(6, 12, ColorE::ColorMazeLevel3_4_5), // f0 06 0c 16
    Instruction::CmdF2SetRepeat(0x06), // f2 06
    Instruction::CmdF0Update(6, -16, ColorE::ColorMazeLevel3_4_5), // f0 06 f0 16
    Instruction::CmdF2SetRepeat(0x0c), // f2 0c
    Instruction::CmdF0Update(3, 9, ColorE::ColorMazeLevel3_4_5), // f0 03 09 16
    Instruction::CmdF2SetRepeat(0x05), // f2 05
    Instruction::CmdF0Update(5, -10, ColorE::ColorMazeLevel3_4_5), // f0 05 f6 16
    Instruction::CmdF2SetRepeat(0x0a), // f2 0a
    Instruction::CmdF0Update(4, 3, ColorE::ColorMazeLevel3_4_5), // f0 04 03 16
    Instruction::CmdF3SetSprite(&SPRITE_JUNIOR_MAN), // f3 &869a
    Instruction::CmdF2SetRepeat(0x01), // f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF2SetRepeat(0x20), // f2 20
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdFFStop, // ff
];

// src:84cf
const INTERMISSION3_2: [Instruction; 19] = [
    Instruction::CmdF1SetPosition(0x00, 0x00), // f1 00 00
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN1), // f3 &8675
    Instruction::CmdF2SetRepeat(0x01), // f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF1SetPosition(0xbd, 0x52), // f1 bd 52
    Instruction::CmdF2SetRepeat(0x28), // f2 28
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x16), // f2 16
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF2SetRepeat(0x16), // f2 16
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0x00, 0x00), // f1 00 00
    Instruction::CmdF3SetSprite(&RIGHT_PAC), // f3 &8638
    Instruction::CmdF2SetRepeat(0x01), // f2 01
    Instruction::CmdF0Update(0, 0, ColorE::Yellow), // f0 00 00 09
    Instruction::CmdF1SetPosition(0xc0, 0xc0), // f1 c0 c0
    Instruction::CmdF2SetRepeat(0x30), // f2 30
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdFFStop, // ff
];

// src:84fd
const INTERMISSION3_3: [Instruction; 19] = [
    Instruction::CmdF1SetPosition(0x00, 0x00), // f1 00 00
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN2), // f3 &867f
    Instruction::CmdF2SetRepeat(0x01), // f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF1SetPosition(0xad, 0x52), // f1 ad 52
    Instruction::CmdF2SetRepeat(0x28), // f2 28
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x16), // f2 16
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF2SetRepeat(0x16), // f2 16
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0x00, 0x00), // f1 00 00
    Instruction::CmdF3SetSprite(&MSP_WALKING_RIGHT), // f3 &8614
    Instruction::CmdF2SetRepeat(0x01), // f2 01
    Instruction::CmdF0Update(0, 0, ColorE::Yellow), // f0 00 00 09
    Instruction::CmdF1SetPosition(0xd0, 0xc0), // f1 d0 c0
    Instruction::CmdF2SetRepeat(0x30), // f2 30
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdFFStop, // ff
];

// src:852b
const INTERMISSION2_2: [Instruction; 13] = [
    Instruction::CmdF1SetPosition(0x00, 0x00), // f1 00 00
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN1), // f3 &8675
    Instruction::CmdF2SetRepeat(0x01), // f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF1SetPosition(0xbd, 0x52), // f1 bd 52
    Instruction::CmdF2SetRepeat(0x28), // f2 28
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x16), // f2 16
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF2SetRepeat(0x16), // f2 16
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0x00, 0x00), // f1 00 00
    Instruction::CmdFFStop, // ff
];

// src:854a
const INTERMISSION2_3: [Instruction; 13] = [
    Instruction::CmdF1SetPosition(0x00, 0x00), // f1 00 00
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN2), // f3 &867f
    Instruction::CmdF2SetRepeat(0x01), // f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF1SetPosition(0xad, 0x52), // f1 ad 52
    Instruction::CmdF2SetRepeat(0x28), // f2 28
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF2SetRepeat(0x16), // f2 16
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // f0 00 00 16
    Instruction::CmdF2SetRepeat(0x16), // f2 16
    Instruction::CmdF6Pause, // f6 
    Instruction::CmdF1SetPosition(0, 0), // f1 00 00
    Instruction::CmdFFStop, // ff
];

// src:8569
const INTERMISSION1_4: [Instruction; 8] = [
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN3), // ram:8569 f3 89 86
    Instruction::CmdF2SetRepeat(1), // ram:856c f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5),    // ram:856e f0 00 00 16
    Instruction::CmdF1SetPosition(189, 98), // ram:8572 f1 bd 62
    Instruction::CmdF2SetRepeat(90),    // ram:8575 f2 5a
    Instruction::CmdF6Pause,    // ram:8577 f6
    Instruction::CmdF1SetPosition(0, 0),    // ram:8578 f1 00 00
    Instruction::CmdFFStop, // ram:857b ff
];

// src:857c
const INTERMISSION1_5: [Instruction; 12] = [
    Instruction::CmdF3SetSprite(&SPRITE_CODES_FOR_ACT_SIGN4), // ram:857c f3 8b 86
    Instruction::CmdF2SetRepeat(1), // ram:857f f2 01
    Instruction::CmdF0Update(0, 0, ColorE::ColorMazeLevel3_4_5), // ram:8581 f0 00 00 16
    Instruction::CmdF1SetPosition(173, 98), // ram:8585 f1 ad 62
    Instruction::CmdF2SetRepeat(57), // ram:8588 f2 39
    Instruction::CmdF6Pause, // ram:858a f6
    Instruction::CmdF7ShowAct, // ram:858b f7
    Instruction::CmdF2SetRepeat(30), // ram:858c f2 1e
    Instruction::CmdF6Pause, // ram:858e f6
    Instruction::CmdF8ClearAct, // ram:858f f8
    Instruction::CmdF1SetPosition(0, 0), // ram:8590 f1 00 00
    Instruction::CmdFFStop, // ram:8593 ff
];

// src:8594
const DATA_FOR_ATTRACT_MODE_1ST_GHOST: [Instruction; 10] = [
    Instruction::CmdF1SetPosition(0, 148),    // f1 00 94
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_LEFT),  // f3 &8663
    Instruction::CmdF2SetRepeat(112),   // f2 70
    Instruction::CmdF0Update(16, 0, ColorE::Red),   // f0 10 00 01
    Instruction::CmdF2SetRepeat(80),              // f2 50
    Instruction::CmdF0Update(16, 0, ColorE::Red),   // f0 10 00 01
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_UP),    // f3 &866a
    Instruction::CmdF2SetRepeat(72),    // f2 48
    Instruction::CmdF0Update(0, -16, ColorE::Red),   // f0 00 f0 01
    Instruction::CmdFFStop,
];

// src:85b0
const DATA_FOR_ATTRACT_MODE_2ND_GHOST: [Instruction; 10] = [
    Instruction::CmdF1SetPosition(0, 148),    // f1 00 94
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_LEFT),  // f3 &8663
    Instruction::CmdF2SetRepeat(0x70),              // f2 70
    Instruction::CmdF0Update(16, 0, ColorE::Pink),  // f0 10 00 03
    Instruction::CmdF2SetRepeat(0x50),              // f2 50
    Instruction::CmdF0Update(16, 0, ColorE::Pink),  // f0 10 00 03
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_UP),    // f3 &866a
    Instruction::CmdF2SetRepeat(0x38),  // f2 38
    Instruction::CmdF0Update(0, -16, ColorE::Pink),  // f0 00 f0 03
    Instruction::CmdFFStop,
];

// src:85cc
const DATA_FOR_ATTRACT_MODE_3RD_GHOST: [Instruction; 10] = [
    Instruction::CmdF1SetPosition(0, 148),    // f1 00 94
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_LEFT),  // f3 &8663
    Instruction::CmdF2SetRepeat(0x70),              // f2 70
    Instruction::CmdF0Update(16, 0, ColorE::Blue),  // f0 10 00 05
    Instruction::CmdF2SetRepeat(0x50),              // f2 50
    Instruction::CmdF0Update(16, 0, ColorE::Blue),  // f0 10 00 05
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_UP),    // f3 &866a
    Instruction::CmdF2SetRepeat(0x28),  // f2 28
    Instruction::CmdF0Update(0, -16, ColorE::Blue),  // f0 00 f0 05
    Instruction::CmdFFStop, // ff
];

// src:85e8
const DATA_FOR_ATTRACT_MODE_4TH_GHOST: [Instruction; 10] = [
    Instruction::CmdF1SetPosition(0, 148),    // f1 00 94
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_LEFT),  // f3 &8663
    Instruction::CmdF2SetRepeat(0x70),              // f2 70
    Instruction::CmdF0Update(16, 0, ColorE::Orange),  // f0 10 00 07
    Instruction::CmdF2SetRepeat(0x50),              // f2 50
    Instruction::CmdF0Update(16, 0, ColorE::Orange),    // f0 10 00 07
    Instruction::CmdF3SetSprite(&GHOST_EYES_LOOKING_UP), // f3 &866a
    Instruction::CmdF2SetRepeat(0x18), // f2 18
    Instruction::CmdF0Update(0, -16, ColorE::Orange),  // f0 00 f0 07
    Instruction::CmdFFStop, // ff
];

// src:8604
const DATA_FOR_ATTRACT_MODE_MAN: [Instruction; 7] = [
    Instruction::CmdF1SetPosition(0, 148),    // f1 00 94
    Instruction::CmdF3SetSprite(&MAN_LEFT), // f3 &8641
    Instruction::CmdF2SetRepeat(0x72),              // f2 72
    Instruction::CmdF0Update(16, 0, ColorE::Yellow),  // f0 10 00 09
    Instruction::CmdF2SetRepeat(0x7f),              // f2 7f
    Instruction::CmdF6Pause, // f6
    Instruction::CmdFFStop, // ff
];

// src:8614
const MSP_WALKING_RIGHT: [SpriteId; 9] = [
    SpriteId::ManRight1, SpriteId::ManRight1, 
    SpriteId::ManRight2, SpriteId::ManRight2,
    SpriteId::ManRight1, SpriteId::ManRight1, 
    SpriteId::MsBack, SpriteId::MsBack,
    SpriteId::CodeForNoSprite    
];
// src:861d
const MSP_WALKING_LEFT: [SpriteId; 9] = [
    SpriteId::ManRight1FlipX, SpriteId::ManRight1FlipX,
    SpriteId::ManRight2FlipX, SpriteId::ManRight2FlipX,
    SpriteId::ManRight1FlipX, SpriteId::ManRight1FlipX,
    SpriteId::MsBackFlipX, SpriteId::MsBackFlipX,
    SpriteId::CodeForNoSprite
];
// src:8626
const MSP_WALKING_UP: [SpriteId; 9] = [
    SpriteId::ManDown1FlipY, SpriteId::ManDown1FlipY,
    SpriteId::ManDown2FlipY, SpriteId::ManDown2FlipY,
    SpriteId::ManDown1FlipY, SpriteId::ManDown1FlipY,
    SpriteId::MsBackFlipY, SpriteId::MsBackFlipY,
    SpriteId::CodeForNoSprite
];
/*
// src:862f
const MSP_WALKING_UP_FLIPPED: [SpriteId; 9] = [
    SpriteId::ManDown1FlipXFlipY, SpriteId::ManDown1FlipXFlipY,
    SpriteId::ManDown2FlipXFlipY, SpriteId::ManDown2FlipXFlipY,
    SpriteId::ManDown1FlipXFlipY, SpriteId::ManDown1FlipXFlipY,
    SpriteId::MsBackFlipXFlipY, SpriteId::MsBackFlipXFlipY,
    SpriteId::CodeForNoSprite
];
*/
// src:8638
const RIGHT_PAC: [SpriteId; 9] = [
    SpriteId::MsRight1, SpriteId::MsRight1, 
    SpriteId::MsRight3, SpriteId::MsRight3, 
    SpriteId::MsRight1, SpriteId::MsRight1, 
    SpriteId::MsRight2, SpriteId::MsRight2,
    SpriteId::CodeForNoSprite    
];
// src:8641
const MAN_LEFT: [SpriteId; 9] = [ 
    SpriteId::MsRight1FlipX, SpriteId::MsRight1FlipX, 
    SpriteId::MsRight3FlipX, SpriteId::MsRight3FlipX,
    SpriteId::MsRight1FlipX, SpriteId::MsRight1FlipX, 
    SpriteId::MsRight2FlipX, SpriteId::MsRight2FlipX, 
    SpriteId::CodeForNoSprite
];
// src:864a
const MSP_MOVING_UP_AT_THE_END: [SpriteId; 9] = [
    SpriteId::MsUp1, SpriteId::MsUp1,
    SpriteId::MsTurn2FlipXFlipY, SpriteId::MsTurn2FlipXFlipY,
    SpriteId::MsUp1, SpriteId::MsUp1,
    SpriteId::MsDown2FlipXFlipY, SpriteId::MsDown2FlipXFlipY,
    SpriteId::CodeForNoSprite
];
/*
// src:8653
const MSP_SPRITE_CODES: [SpriteId; 9] = [
    SpriteId::MsDown1, SpriteId::MsDown1,
    SpriteId::MsTurn2, SpriteId::MsTurn2,
    SpriteId::MsDown1, SpriteId::MsDown1,
    SpriteId::MsDown2, SpriteId::MsDown2,
    SpriteId::CodeForNoSprite
];
*/
// src:865c
const GHOST_EYES_LOOKING_RIGHT: [SpriteId; 7] = [
    SpriteId::GhostLeft1FlipX, SpriteId::GhostLeft1FlipX, SpriteId::GhostLeft1FlipX, 
    SpriteId::GhostLeft2FlipX, SpriteId::GhostLeft2FlipX, SpriteId::GhostLeft2FlipX, 
    SpriteId::CodeForNoSprite 
];
// src:8663
const GHOST_EYES_LOOKING_LEFT: [SpriteId; 7] = [ 
    SpriteId::GhostLeft1, SpriteId::GhostLeft1, SpriteId::GhostLeft1,
    SpriteId::GhostLeft2, SpriteId::GhostLeft2, SpriteId::GhostLeft2, 
    SpriteId::CodeForNoSprite 
];
// src:866a
const GHOST_EYES_LOOKING_UP: [SpriteId; 7] = [ 
    SpriteId::GhostUp1, SpriteId::GhostUp1, SpriteId::GhostUp1, 
    SpriteId::GhostUp2, SpriteId::GhostUp2, SpriteId::GhostUp2, 
    SpriteId::CodeForNoSprite 
];
// src:8671
const EMPTY_SPRITE: [SpriteId; 2] = [
    SpriteId::Empty, SpriteId::CodeForNoSprite
];
// src:8673
const SPRITE_CODES_FOR_HEART: [SpriteId; 2] = [
    SpriteId::Heart, SpriteId::CodeForNoSprite
];
// src:8675
const SPRITE_CODES_FOR_ACT_SIGN1: [SpriteId; 10] = [
    SpriteId::Gfx10, SpriteId::Gfx10, SpriteId::Gfx10, 
    SpriteId::Gfx14, SpriteId::Gfx14, SpriteId::Gfx14,
    SpriteId::Gfx16, SpriteId::Gfx16, SpriteId::Gfx16, 
    SpriteId::CodeForNoSprite
];
// src:867f
const SPRITE_CODES_FOR_ACT_SIGN2: [SpriteId; 10] = [
    SpriteId::Gfx11, SpriteId::Gfx11, SpriteId::Gfx11,
    SpriteId::Gfx15, SpriteId::Gfx15, SpriteId::Gfx15, 
    SpriteId::Gfx17, SpriteId::Gfx17, SpriteId::Gfx17, 
    SpriteId::CodeForNoSprite
];
// src:8689
const SPRITE_CODES_FOR_ACT_SIGN3: [SpriteId; 2] = [
    SpriteId::GfxActLeft, SpriteId::CodeForNoSprite
];
// src:868b
const SPRITE_CODES_FOR_ACT_SIGN4: [SpriteId; 2] = [
    SpriteId::GfxActRight, SpriteId::CodeForNoSprite
];
// src:868d
const SPRITE_CODES_FOR_STORK: [SpriteId; 2] = [
    SpriteId::Stork2, SpriteId::CodeForNoSprite
];
// src:868f
const SPRITE_CODES_FOR_STORKS: [SpriteId; 9] = [
    SpriteId::Stork0, SpriteId::Stork0, SpriteId::Stork0, SpriteId::Stork0, 
    SpriteId::Stork1, SpriteId::Stork1, SpriteId::Stork1, SpriteId::Stork1, 
    SpriteId::CodeForNoSprite
];
// src:8698
const SPRITE_CODES_FOR_STORK_CARRIES: [SpriteId; 2] = [
    SpriteId::PearSmall, SpriteId::CodeForNoSprite,
];
// src:869a
const SPRITE_JUNIOR_MAN: [SpriteId; 2] = [
    SpriteId::ManSmallLeft, SpriteId::CodeForNoSprite,
];

/*

pub const NO_SPRITES: [SpriteId; 0] = [];

pub struct GameAnimationT {
    // src:4f00
    on: bool,                                            
    // src:4f02
    animation_current: [ (&'static [Instruction], usize) ; 6],
    // src:4f0f
    pub animation_cmd_table_sprite_index: [ u8; 6],
    // src:4f17
    pub animation_cmd_table_repeat: [ u8; 6],
    // src:4f1f
    pub animation_cmd_table_stop:  [bool; 6],
    // src:4f2e
    pub animation_cmd_table_coord: [ (i8,i8); 6],
    // src:4f3e
    pub animation_cmd_table_sprite: [ &'static [SpriteId]; 6],
}

impl GameAnimationT {
    pub fn new() -> Self {
        GameAnimationT {
            on: false,
            animation_current: NO_ANIMATION,
            animation_cmd_table_sprite_index: [ 0; 6],
            animation_cmd_table_repeat: [ 0; 6],
            animation_cmd_table_stop: [true; 6],
            animation_cmd_table_coord: [ (0,0); 6 ],
            animation_cmd_table_sprite: [ &NO_SPRITES; 6],
        }
    }
}
*/

pub trait GameAnimation {
    fn animation_init(&mut self, id:AnimationE);
    fn animation(&mut self, id:AnimationE);

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
                    self.sprite[index].p += Point::new(-ah as i32, bh as i32);
                    // print!("(ah, al)=({},{}), (bh, bl)=({},{})", ah, al, bh, bl);
                    // print!(", p:{:?}, t:{:?}", self.sprite[index].p, self.animation_cmd_table_coord[index]);

                    let mut sprite_index = self.animation_cmd_table_sprite_index[index] + 1;
                    let mut sprite = self.animation_cmd_table_sprite[index][sprite_index as usize];
                    if sprite == SpriteId::CodeForNoSprite {
                        sprite_index = 0;   // loop sprite
                        sprite = self.animation_cmd_table_sprite[index][sprite_index as usize];
                    }
                    self.animation_cmd_table_sprite_index[index] = sprite_index;

                    self.sprite[index].c = c;

                    /* flip x and y in cocktail mode */
                    if self.cocktail_mode && self.current_player_number != 0 {
                        self.sprite[index].s.flip_xy();
                    }

                    /* select sprite */
                    self.sprite[index].s = sprite;

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
                    self.sprite[index].p = Point::new(x2 as i32,y2 as i32);
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
                    self.hwsound.effect[2].num = n;
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

}