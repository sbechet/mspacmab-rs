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

// use 21 palettes (RGB888 => 63 useful bytes)
pub const PALETTE: [ [u32; 4]; 32] = [
    [BLACK,BLACK,BLACK,BLACK],  // 0 TOREMOVE?
    [BLACK,GREY,BLUE,RED],      // 1
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,GREY,BLUE,PINK],     // 3
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,GREY,BLUE,LIGHT_BLUE],   // 5
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,GREY,BLUE,LIGHT_BROWN],  // 7
    [BLACK,BLACK,BLACK,BLACK],  // TOREMOVE
    [BLACK,BLUE,RED,YELLOW],    // 9 -> mspacmab
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
