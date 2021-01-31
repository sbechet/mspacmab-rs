use num_derive::{ FromPrimitive, ToPrimitive };

const BLACK: u32        = 0x00_00_00_00;
const GREY: u32         = 0x00_DE_DE_DE;

const RED: u32          = 0x00_FF_00_00;
const GREEN: u32        = 0x00_00_FF_00;
const BLUE: u32         = 0x00_21_21_DE;

const PINK: u32         = 0x00_FF_B8_DE;
const YELLOW: u32       = 0x00_FF_FF_00;
const BROWN: u32        = 0x00_DE_97_47;

const LIGHT_BLUE: u32   = 0x00_00_FF_DE;
const LIGHT_BLUE2: u32  = 0x00_47_B8_DE;
const LIGHT_BROWN: u32  = 0x00_FF_B8_47;
const LIGHT_BROWN2: u32 = 0x00_FF_B8_97;
const LIGHT_GREEN: u32  = 0x00_47_B8_97;

/*

    Ignored when actually coloring the grid, so it is invisible onscreen:

    bit 7 = ? (0x80)
    bit 6 = 1 (0x40) Historicaly was used for tunnel slowdown (see is_tunnel_slowdown() fn)
    bit 5 = ? (0x20)

*/
// use 21 palettes (RGB888 => 63 useful bytes)
pub const PALETTE: [ [u32; 4]; 32] = [
    [BLACK,BLACK,BLACK,BLACK],  // 0 TOREMOVE?
    [BLACK,GREY,BLUE,RED],      // 1 RED
    [BLACK,BLACK,BLACK,BLACK],  // flashing?
    [BLACK,GREY,BLUE,PINK],     // 3 PINK
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,GREY,BLUE,LIGHT_BLUE],   // 5 BLUE
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,GREY,BLUE,LIGHT_BROWN],  // 7 ORANGE
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,BLUE,RED,YELLOW],    // 9 YELLOW
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,GREY,BLACK,LIGHT_BROWN2],    // 14
    [BLACK,RED,GREEN,GREY],     // 15 -> fruits
    [BLACK,LIGHT_BROWN2,BLACK,BLUE],    // 16
    [BLACK,GREEN,BLUE,LIGHT_BROWN2],    // 17
    [BLACK,GREEN,GREY,RED],     // 18
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,RED,BROWN,GREY],     // 20
    [BLACK,LIGHT_BROWN,GREEN,BROWN],    // 21
    [BLACK,YELLOW,LIGHT_BLUE2,GREY],    // 22
    [BLACK,LIGHT_GREEN,GREEN,GREY],     // 23
    [BLACK,LIGHT_BLUE,PINK,YELLOW],     // 24
    [BLACK,GREY,BLUE,BLACK],    // 25
    [BLACK,LIGHT_BROWN2,BLACK,BLUE],    // 26
    [BLACK,LIGHT_BROWN2,BLACK,BLUE],    // 27
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,GREY,LIGHT_BROWN2,RED],      // 29
    [BLACK,GREY,BLUE,LIGHT_BROWN2],     // 30
    [BLACK,LIGHT_BROWN2,BLACK,GREY],    // 31
];

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ColorE {
    Black=0,
    Red=1,
    Flashing=2,
    Pink=3,
    Blue=5,
    Orange=7,
    Yellow=9,
    LightBrown2=14,
    ColorFruit=15,
    ColorGreyRed=18,
    ColorMazeLevel6_7_8_9=20,
    Brown=21,
    ColorMazeLevel3_4_5=22,
    ColorFruitPear=23,
    ColorMazeLevel14_15_16_17AndGhostsDoor=24,
    ColorPacmanAndGhostInitialMapPositions=26,
    ColorTunnelArea=27,
    ColorMazeLevel1_2_18_19_20_21=29,
    White=31,
}
