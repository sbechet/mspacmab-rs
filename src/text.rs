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
use crate::tile::Tile;
use crate::palette::PALETTE;

pub struct Text<'a> {
    coord: (usize, usize),
    color: &'a [u8], 
    text: &'a [u8],
}

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ({}, {}), {:?}, {:?} }}", self.coord.0, self.coord.1, self.color, self.text)
    }
}

pub enum TextId {
    HighScore = 0,
    Credit = 1,
    FreePlay = 2,
    PlayerOne = 3,
    PlayerTwo = 4,
    GameOver = 5,
    Ready = 6,
    PushStartButton = 7,
    OnePlayerOnly = 8,
    OneOrTwoPlayers = 9,
    EmptyString = 10,
    AdditionalAt000Pts = 11,
    MsPacman = 12,
    Blinky = 13,
    With = 14,
    Pinky = 15,
    Starring = 16,
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
    Space4 = 32,
    WordRam38f6 = 33,   // :: OOo
    WordRam3900 = 34,   // :: OOo
    MemoryOk = 35,
    BadRM = 36,
    FreePlay_ = 37,
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
const TEXT: [Text; 55] = [
    // 0
    Text { coord: (11-2, 0), color: &[15], text: b"HIGH@SCORE" },
    Text { coord: ( 4-2,35), color: &[15], text: b"CREDIT@@@" },
    Text { coord: ( 4-2,35), color: &[15], text: b"FREE@PLAY" },
    Text { coord: (11-2,14), color: &[ 5], text: b"PLAYER@ONE" },    // 1a 1a 1a 1a 1a 10 10
    Text { coord: (11-2,14), color: &[ 5], text: b"PLAYER@TWO" },
    Text { coord: (11-2,20), color: &[ 1], text: b"GAME@@OVER" },
    Text { coord: (13-2,20), color: &[ 9], text: b"READY{" },
    Text { coord: ( 8-2,15), color: &[ 7], text: b"PUSH@START@BUTTON" },
    Text { coord: (10-2,17), color: &[ 7], text: b"1@PLAYER@ONLY@" },
    Text { coord: (10-2,17), color: &[ 7], text: b"1@OR@2@PLAYERS" },
    // 10
    Text { coord: (13-2,28), color: &[10,10,10,1,1,1,1], text: b"@@@@@@@" },
    Text { coord: ( 3-2,24), color: &[21], text: b"@ADDITIONAL@@@@AT@@@000@]^_" },
    Text { coord: ( 4-2, 7), color: &[ 7], text: b"@@@@@@@@&MS@PAC;MAN'@" },
    Text { coord: (12-2,16), color: &[ 1], text: b"@@@BLINKY" },
    Text { coord: (12-2,13), color: &[15], text: b"WITH@@@@@" },
    Text { coord: (12-2,16), color: &[ 3], text: b"@@@PINKY@" },
    Text { coord: (12-2,13), color: &[15], text: b"STARRING@" },
    Text { coord: (12-2,24), color: &[31], text: b"\x10@10@]^_" },
    Text { coord: (12-2,26), color: &[31], text: b"\x14@50@]^_" },
    Text { coord: (13-2,29), color: &[ 1], text: b"\\@MIDWAY@MFG@CO@@@@" },
    // 20
    Text { coord: ( 9-2, 7), color: &[ 1], text: b";MAD@DOG@@" },
    Text { coord: (12-2,13), color: &[15], text: b"JUNIOR@@@@" },
    Text { coord: ( 9-2,10), color: &[ 3], text: b";KILLER@@@" },
    Text { coord: (12-2,13), color: &[15], text: b"THE@CHASE@/" },

    Text { coord: ( 0, 0), color: &[ 0], text: b"" },    // 24
    Text { coord: ( 0, 0), color: &[ 0], text: b"" },    // 25
    Text { coord: ( 0, 0), color: &[ 0], text: b"" },    // 26
    Text { coord: ( 0, 0), color: &[ 0], text: b"" },    // 27

    Text { coord: (12-2,16), color: &[ 9], text: b"SUPER@PAC;MAN" },

    Text { coord: ( 0, 0), color: &[ 0], text: b"" },    // 29
    // 30
    Text { coord: ( 0, 0), color: &[ 0], text: b"" },    // 30

    Text { coord: (17-2,35), color: &[14], text: b"\x86\x8B\x8D\x8E" },
    Text { coord: (15-2,35), color: &[20], text: b"@@@@" },
    Text { coord: (13-2,35), color: &[ 9], text: b"\x89\x8A\x8D\x8E" },
    Text { coord: (11-2,35), color: &[ 9], text: b"\x89\x8A\x8D\x8E" },
    Text { coord: ( 7-2, 6), color: &[15], text: b"MEMORY@@OK" },
    Text { coord: ( 7-2, 6), color: &[15], text: b"BAD@@@@R@M" },
    Text { coord: ( 7-2,10), color: &[15], text: b"FREE@@PLAY@@@@@@@" },
    Text { coord: ( 7-2,10), color: &[15], text: b"1@COIN@@1@CREDIT@" },
    Text { coord: ( 7-2,10), color: &[15], text: b"1@COIN@@2@CREDITS" },
    // 40
    Text { coord: ( 7-2,10), color: &[15], text: b"2@COINS@1@CREDIT@" },
    Text { coord: ( 7-2,14), color: &[15], text: b"MS@PAC;MEN" },
    Text { coord: ( 7-2,12), color: &[15], text: b"BONUS@@NONE" },
    Text { coord: ( 7-2,12), color: &[15], text: b"BONUS@" },
    Text { coord: ( 7-2,16), color: &[15], text: b"TABLE@@" },
    Text { coord: ( 7-2,16), color: &[15], text: b"UPRIGHT" },
    Text { coord: (15-2,12), color: &[15], text: b"000" },
    Text { coord: (12-2,16), color: &[ 5], text: b"@@@INKY@@" },
    Text { coord: (12-2,13), color: &[ 5], text: b"@@@@@@@@@" },
    Text { coord: (12-2,16), color: &[ 7], text: b"@@@@SUE" },
    // 50
    Text { coord: (12-2,13), color: &[15], text: b"THEY@MEET" },
    Text { coord: (12-2,16), color: &[ 9], text: b"MS@PAC;MAN" },
    Text { coord: ( 7-2,14), color: &[15], text: b"MS@PAC;MEN" },
    Text { coord: (14-2,31), color: &[ 1], text: b"@@1980:1981@" },
    Text { coord: (12-2,13), color: &[ 7], text: b"ACT@III&@@" },

    // full data but no xref
    /*
    Text [ coord: ( 7,14), color: &[15], text: b"OTTOMEN" },
    Text { coord: ( 3,24), color: &[14], b"BONUS@PUCKMAN@FOR@@@000@]^_" },
    Text { coord: (10,28), color: &[ 3], b"\\@()*+,-.@1980" },
    Text { coord: (19, 2), color: &[ 1], b"&AKABEI&" },
    Text { coord: (21, 7), color: &[ 1], b"&MACKY&" },
    Text { coord: (21,10), color: &[ 3], b"&PINKY&" },
    Text { coord: (21,10), color: &[ 3], b"&MICKY&" },
    Text { coord: (13,31), color: &[ 3], b"()*+,-." },
    Text { coord: ( 9, 7), color: &[ 1], b"@OIKAKE;;;;" },
    Text { coord: ( 9, 7), color: &[ 1], b"@URCHIN;;;;;" },
    Text { coord: ( 9,10), color: &[ 3], b"@MACHIBUSE;;" },
    Text { coord: ( 9,10), color: &[ 3], b"@ROMP;;;;;;;" },
    Text { coord: (17,35), color: &[15], b"\x86\x8B\x8D\x8E" },
    Text { coord: ( 7,14), color: &[15], b"PUCKMAN" },
    Text { coord: (15,12), color: &[15], b"000" },
    Text { coord: (20,13), color: &[ 5], b"&AOSUKE&" },
    Text { coord: ( 9,13), color: &[ 5], b"@KIMAGURE;;" },
    Text { coord: ( 9,13), color: &[ 5], b"@STYLIST;;;;" },
    Text { coord: ( 9,16), color: &[ 7], b"@OTOBOKE;;;" },
    Text { coord: ( 9,16), color: &[ 7], b"@CRYBABY;;;;" },
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
    pub fn draw_text(&self, display: &mut SimulatorDisplay<Rgb888>, clean: bool) {
        let mut p = Point::new( 8 * self.coord.0 as i32, 8 * self.coord.1 as i32);
        for c in self.text {
            if clean != true {
                let t = Tile::from_id(b'@' as usize, PALETTE[self.color[0] as usize]);
                let image: Image<Tile, Rgb888> = Image::new(&t, p);
                image.draw(display).unwrap();
            } else {
                let t = Tile::from_id(*c as usize, PALETTE[self.color[0] as usize]);
                let image: Image<Tile, Rgb888> = Image::new(&t, p);
                image.draw(display).unwrap();
            }
            p.x += 8;
        }
    }
}
