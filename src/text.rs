use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    prelude::*,
};
use embedded_graphics_simulator::{
    SimulatorDisplay, 
};

//--

use std::fmt;
use crate::game_hw_video::GameHwVideo;
use crate::tile::{TileId, Tile};
use crate::palette::{ PALETTE, ColorE };

pub struct Text<'a> {
    coord: (usize, usize),
    color: ColorE, 
    text: &'a [u8],
}

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ({}, {}), {:?}, {:?} }}", self.coord.0, self.coord.1, self.color as u8, self.text)
    }
}

#[derive(FromPrimitive)]
pub enum TextId {
    HighScore = 0,                  // 0x00
    Credit = 1,
    FreePlay = 2,
    PlayerOne = 3,
    PlayerTwo = 4,
    GameOver = 5,
    Ready = 6,                      // SPECIAL
    PushStartButton = 7,
    OnePlayerOnly = 8,
    OneOrTwoPlayers = 9,
    TileMsPacMan = 10,              // SPECIAL (TILE)
    AdditionalAt000Pts = 11,        // SPECIAL
    MsPacman = 12,
    Blinky = 13,
    With = 14,
    Pinky = 15,
    Starring = 16,                  // 0x10
    TenPoints = 17,
    FivtyPoints = 18,
    CMidwayMfgCo = 19,
    MadDog = 20,
    Junior = 21,
    Killer = 22,
    TheChase = 23,
    /* 24, 25, 26, 27 */
    SuperPacMan = 28,
    /* 29, 30 */
    WordRam38e2 = 31,   // ..: 00o
    Space4 = 32,                   // 0x20
    WordRam38f6 = 33,   // :: OOo
    WordRam3900 = 34,   // :: OOo
    MemoryOk = 35,
    BadRM = 36,
    FreePlayTest = 37,
    OneCoin1Credit = 38,
    OneCoin2Credits = 39,
    TwoCoins1Credit = 40,
    MsPacMen = 41,
    BonusNone = 42,
    Bonus = 43,
    Table = 44,
    Upright = 45,
    Zero00 = 46,
    Inky = 47,
    Space = 48,
    Sue = 49,
    TheyMeet = 50,
    MsPacMan = 51,
    MsPacMen_ = 52,
    Year19801981 = 53,
    ActIii = 54,
}

    
/* TODO: Change using ANSI escape color and recent charset */
// src:3713
const TEXT: [Text; 55] = [
    // 0
    Text { coord: ( 9, 0), color: ColorE::ColorFruit, text: b"HIGH@SCORE" },
    Text { coord: ( 2,35), color: ColorE::ColorFruit, text: b"CREDIT@@@" },
    Text { coord: ( 2,35), color: ColorE::ColorFruit, text: b"FREE@PLAY" },
    Text { coord: ( 9,14), color: ColorE::Blue, text: b"PLAYER@ONE" },    // Historical palette 1a 1a 1a 1a 1a 10 10
    Text { coord: ( 9,14), color: ColorE::Blue, text: b"PLAYER@TWO" },
    Text { coord: ( 9,20), color: ColorE::Red, text: b"GAME@@OVER" },
    Text { coord: (11,20), color: ColorE::Yellow, text: b"READY[" },        // SPECIAL
    Text { coord: ( 6,15), color: ColorE::Orange, text: b"PUSH@START@BUTTON" },
    Text { coord: ( 8,17), color: ColorE::Orange, text: b"1@PLAYER@ONLY@" },
    Text { coord: ( 8,17), color: ColorE::Orange, text: b"1@OR@2@PLAYERS" },
    // 10
    Text { coord: (11,28), color: ColorE::Black, text: b"@@@@@@@" }, // SPECIAL - Historical palette 10,10,10,1,1,1,1
    Text { coord: ( 1,24), color: ColorE::Brown, text: b"@ADDITIONAL@@@@AT@@@000@]^_" },    // SPECIAL: ADDITIONAL <tile> AT <..>000 pts
    Text { coord: ( 2, 7), color: ColorE::Orange, text: b"@@@@@@@@&MS@PAC;MAN'@" },
    Text { coord: (10,16), color: ColorE::Red, text: b"@@@BLINKY" },
    Text { coord: (10,13), color: ColorE::ColorFruit, text: b"WITH@@@@@" },
    Text { coord: (10,16), color: ColorE::Pink, text: b"@@@PINKY@" },
    Text { coord: (10,13), color: ColorE::ColorFruit, text: b"STARRING@" },
    Text { coord: (10,24), color: ColorE::White, text: b"\x10@10@]^_" },
    Text { coord: (10,26), color: ColorE::White, text: b"\x14@50@]^_" },
    Text { coord: (11,29), color: ColorE::Red, text: b"\\@MIDWAY@MFG@CO@@" },   // Historical BUG here two '@@' overflow
    // 20
    Text { coord: ( 7, 7), color: ColorE::Red, text: b";MAD@DOG@@" },
    Text { coord: (10,13), color: ColorE::ColorFruit, text: b"JUNIOR@@@@" },
    Text { coord: ( 7,10), color: ColorE::Pink, text: b";KILLER@@@" },
    Text { coord: (10,13), color: ColorE::ColorFruit, text: b"THE@CHASE@/" },

    Text { coord: ( 0, 0), color: ColorE::Black, text: b"" },    // 24
    Text { coord: ( 0, 0), color: ColorE::Black, text: b"" },    // 25
    Text { coord: ( 0, 0), color: ColorE::Black, text: b"" },    // 26
    Text { coord: ( 0, 0), color: ColorE::Black, text: b"" },    // 27

    Text { coord: (10,16), color: ColorE::Yellow, text: b"SUPER@PAC;MAN" },

    Text { coord: ( 0, 0), color: ColorE::Black, text: b"" },    // 29
    // 30
    Text { coord: ( 0, 0), color: ColorE::Black, text: b"" },    // 30

    Text { coord: (15,35), color: ColorE::LightBrown2, text: b"\x86\x8B\x8D\x8E" },
    Text { coord: (13,35), color: ColorE::ColorMazeLevel6_7_8_9, text: b"@@@@" },
    Text { coord: (11,35), color: ColorE::Yellow, text: b"\x89\x8A\x8D\x8E" },
    Text { coord: ( 9,35), color: ColorE::Yellow, text: b"\x89\x8A\x8D\x8E" },
    Text { coord: ( 5, 6), color: ColorE::ColorFruit, text: b"MEMORY@@OK" },
    Text { coord: ( 5, 6), color: ColorE::ColorFruit, text: b"BAD@@@@R@M" },
    Text { coord: ( 5,10), color: ColorE::ColorFruit, text: b"FREE@@PLAY@@@@@@@" },
    Text { coord: ( 5,10), color: ColorE::ColorFruit, text: b"1@COIN@@1@CREDIT@" },
    Text { coord: ( 5,10), color: ColorE::ColorFruit, text: b"1@COIN@@2@CREDITS" },
    // 40
    Text { coord: ( 5,10), color: ColorE::ColorFruit, text: b"2@COINS@1@CREDIT@" },
    Text { coord: ( 5,14), color: ColorE::ColorFruit, text: b"MS@PAC;MEN" },
    Text { coord: ( 5,12), color: ColorE::ColorFruit, text: b"BONUS@@NONE" },
    Text { coord: ( 5,12), color: ColorE::ColorFruit, text: b"BONUS@" },
    Text { coord: ( 5,16), color: ColorE::ColorFruit, text: b"TABLE@@" },
    Text { coord: ( 5,16), color: ColorE::ColorFruit, text: b"UPRIGHT" },
    Text { coord: (13,12), color: ColorE::ColorFruit, text: b"000" },
    Text { coord: (10,16), color: ColorE::Blue, text: b"@@@INKY@@" },
    Text { coord: (10,13), color: ColorE::Blue, text: b"@@@@@@@@@" },
    Text { coord: (10,16), color: ColorE::Orange, text: b"@@@@SUE" },
    // 50
    Text { coord: (10,13), color: ColorE::ColorFruit, text: b"THEY@MEET" },
    Text { coord: (10,16), color: ColorE::Yellow, text: b"MS@PAC;MAN" },
    Text { coord: ( 5,14), color: ColorE::ColorFruit, text: b"MS@PAC;MEN" },
    Text { coord: (12,31), color: ColorE::Red, text: b"@@1980:1981@" },
    Text { coord: (10,13), color: ColorE::Orange, text: b"ACT@III&@@" },

    /*
    // full data but no xref found
    Text [ coord: ( 7,14), color: ColorE::ColorFruit, text: b"OTTOMEN" },
    Text { coord: ( 3,24), color: ColorE::LightBrown2, b"BONUS@PUCKMAN@FOR@@@000@]^_" },
    Text { coord: (10,28), color: ColorE::Pink, b"\\@()*+,-.@1980" },
    Text { coord: (19, 2), color: ColorE::Red, b"&AKABEI&" },
    Text { coord: (21, 7), color: ColorE::Red, b"&MACKY&" },
    Text { coord: (21,10), color: ColorE::Pink, b"&PINKY&" },
    Text { coord: (21,10), color: ColorE::Pink, b"&MICKY&" },
    Text { coord: (13,31), color: ColorE::Pink, b"()*+,-." },
    Text { coord: ( 9, 7), color: ColorE::Red, b"@OIKAKE;;;;" },
    Text { coord: ( 9, 7), color: ColorE::Red, b"@URCHIN;;;;;" },
    Text { coord: ( 9,10), color: ColorE::Pink, b"@MACHIBUSE;;" },
    Text { coord: ( 9,10), color: ColorE::Pink, b"@ROMP;;;;;;;" },
    Text { coord: (17,35), color: ColorE::ColorFruit, b"\x86\x8B\x8D\x8E" },
    Text { coord: ( 7,14), color: ColorE::ColorFruit, b"PUCKMAN" },
    Text { coord: (15,12), color: ColorE::ColorFruit, b"000" },
    Text { coord: (20,13), color: ColorE::Blue, b"&AOSUKE&" },
    Text { coord: ( 9,13), color: ColorE::Blue, b"@KIMAGURE;;" },
    Text { coord: ( 9,13), color: ColorE::Blue, b"@STYLIST;;;;" },
    Text { coord: ( 9,16), color: ColorE::Orange, b"@OTOBOKE;;;" },
    Text { coord: ( 9,16), color: ColorE::Orange, b"@CRYBABY;;;;" },
    */
];


impl Text<'_> {

    pub fn get(id: TextId) -> &'static Self {
        &TEXT[id as usize]
    }

    pub fn get_id(id: usize) -> &'static Self {
        &TEXT[id]
    }

    // Maybe a day use color[i]  with i in 0..n and self.text.len()==n
    pub fn draw_text(&self, hwvideo: &mut GameHwVideo, clean: bool) {
        let mut p = Point::new( self.coord.0 as i32, self.coord.1 as i32);
        // println!("({},{}) -> {}", self.coord.0, self.coord.1, self);
        for c in self.text {
            if clean == true {
                // let t = Tile::from_id(b'@' as usize, PALETTE[self.color[0] as usize]);
                // let image: Image<Tile, Rgb888> = Image::new(&t, p);
                // image.draw(display).unwrap();
                hwvideo.put_screen(p, TileId::Space, self.color);
            } else {
                // let t = Tile::from_id(*c as usize, PALETTE[self.color[0] as usize]);
                // let image: Image<Tile, Rgb888> = Image::new(&t, p);
                // image.draw(display).unwrap();
                // println!("({},{}) -> {}", p.x, p.y, *c);
                match TileId::from_u8(*c) {
                    Some(e) => {
                        hwvideo.put_screen(p, e, self.color);
                        // print!("{:?}", e);
                    },
                    _ => {},
                }
            }
            p.x += 1;
        }
        // println!();
    }

    // src:369, src:376, src:383, src:390
    pub fn print(hwvideo: &mut GameHwVideo, p:(u8, u8), text:&str) {
        let mut x = p.0 as i32;
        let y = p.1 as i32;

        for c in text.as_bytes() {
            let p = Point::new(x,y);
            x += 1;
            // Hack for space
            let tileid = match c {
                b' ' => TileId::Space,
                _ => TileId::from_u8(*c).unwrap(),
            };
            hwvideo.put_screen_tile(p, tileid);
        }
    }
}
