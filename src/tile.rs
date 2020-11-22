use core::marker::PhantomData;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

use embedded_graphics::{
    drawable::Pixel as EgPixel,
    geometry::Point,
    image::{ImageDimensions, IntoPixelIter},
    pixelcolor::{raw::RawData, PixelColor},
};

use crate::palette::{ PALETTE, ColorE };
use crate::pixel::Pixel;
use crate::mspacmab_data::{TILE};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, FromPrimitive, ToPrimitive)]
pub enum TileId {
    HexNumber0 =   0,
    HexNumber1 =   1,
    HexNumber2 =   2,
    HexNumber3 =   3,
    HexNumber4 =   4,
    HexNumber5 =   5,
    HexNumber6 =   6,
    HexNumber7 =   7,
    HexNumber8 =   8,
    HexNumber9 =   9,
    HexNumberA =  10,
    HexNumberB =  11,
    HexNumberC =  12,
    HexNumberD =  13,
    HexNumberE =  14,
    HexNumberF =  15,
    Pill1   =  16,
    Pill2   =  17,
    Pill3   =  18,
    Pill4   =  19,
    Pill5   =  20,
    Pill6   =  21,
    /* 22..=31 empty */
    MspacBigUpperRight  = 32,
    MspacBigUpperLeft   = 33,
    MspacBigLowerRight  = 34,
    MspacBigLowerLeft   = 35,
    /* 36 empty */
    Dot = 37,
    QuoteLeft = 38,
    QuoteRight = 39,
    HeartUpperRight = 40,
    HeartUpperLeft = 41,
    HeartLowerRight = 42,
    HeartLowerLeft = 43,

    DoNo1 = 44,
    DoNo2 = 45,
    DoNo3 = 46,
    /* 47 empty */
    Number0 =   48, // 0x30 (b'0')
    Number1 =   49,
    Number2 =   50,
    Number3 =   51,
    Number4 =   52,
    Number5 =   53,
    Number6 =   54,
    Number7 =   55,
    Number8 =   56,
    Number9 =   57,
    Slash   =   58,
    Minus   =   59,
    GridUpperRight = 60,
    GridLowerRight = 61,
    GridUpperLeft = 62,
    GridLowerLeft = 63,
    Space = 64,
    A = b'A' as isize,   // 0d65 0x41 (b'A')
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = b'Z' as isize,   // 0d90 0x5A (b'Z'),
    Exclamation = 91,
    Copyright = 92,
    PtsLeft = 93,
    PtsMiddle = 94,
    PtsRight = 95,  // 0x5F

    // 0x60..0x8F

    CherryUR = 144, // 0x90
    CherryUL = 145, // 0x91
    CherryLR = 146, // 0x92
    CherryLL = 147, // 0x93

    StrawberryUR = 148, // 0x94
    StrawberryUL = 149, // 0x95
    StrawberryLR = 150, // 0x96
    StrawberryLL = 151, // 0x97

    PeachUR = 152, // 0x98
    PeachUL = 153, // 0x99
    PeachLR = 154, // 0x9A
    PeachLL = 155, // 0x9B

    PretzelUR = 156, // 0x9C
    PretzelUL = 157, // 0x9D
    PretzelLR = 158, // 0x9E
    PretzelLL = 159, // 0x9F

    AppleUR = 160, // 0xA0
    AppleUL = 161, // 0xA1
    AppleLR = 162, // 0xA2
    AppleLL = 163, // 0xA3

    PearUR = 164, // 0xA4
    PearUL = 165, // 0xA5
    PearLR = 166, // 0xA6
    PearLL = 167, // 0xA7

    BananaUR = 168, // 0xA8
    BananaUL = 169, // 0xA9
    BananaLR = 170, // 0xAA
    BananaLL = 171, // 0xAB

    KeyUR = 172, // 0xAC
    KeyUL = 173, // 0xAD
    KeyLR = 174, // 0xAE
    KeyLL = 175, // 0xAF

    MidwayLogoLine11 = 176,  // 0xB0
    MidwayLogoLine12 = 177,  // 0xB1
    MidwayLogoLine13 = 178,  // 0xB2
    MidwayLogoLine14 = 179,  // 0xB3

    MidwayLogoLine21 = 180,  // 0xB4
    MidwayLogoLine22 = 181,  // 0xB5
    MidwayLogoLine23 = 182,  // 0xB6
    MidwayLogoLine24 = 183,  // 0xB7

    MidwayLogoLine31 = 184,  // 0xB8
    MidwayLogoLine32 = 185,  // 0xB9
    MidwayLogoLine33 = 186,  // 0xBA
    MidwayLogoLine34 = 187,  // 0xBB

    MidwayLogoLine41 = 188,  // 0xBC
    MidwayLogoLine42 = 189,  // 0xBD
    MidwayLogoLine43 = 190,  // 0xBE
    MidwayLogoLine44 = 191,  // 0xBF

    MazeFill = 192,  // 0xC0
    MazeFillS = 193,  // 0xC1 -- sym(192)

    Test194 = 194,  // 0xC2
    Test195 = 195,  // 0xC3
    // 0xC4, ... 0xCD

    MazeGhostDoor206 = 206,  // 0xCE
    MazeGhostDoor207 = 207,  // 0xCF -- sym

    MazeUpRight = 208,  // 0xD0
    MazeUpLeft = 209,   // 0xD1 -- sym

    MazeVerticalOnTheLeft = 210,  // 0xD2
    MazeVerticalOnTheRight = 211,  // 0xD3 -- sym

    MazeRoundDownRight = 212,   // 0xD4
    MazeRoundDownLeft = 213,    // 0xD5 -- sym

    Maze214 = 214,  // 0xD6 (in level >=2)
    Maze215 = 215,  // 0xD7 -- sym

    Maze216 = 216,  // 0xD8
    Maze217 = 217,  // 0xD9 -- sym

    MazeHorizontalDown = 218,  // 0xDA
    MazeHorizontalDownS = 219,  // 0xDB -- sym

    MazeHorizontalUp = 220,  // 0xDC
    MazeHorizontalUpS = 221,  // 0xDD -- sym

    MazeHorizontalDownFull = 222,  // 0xDE
    MazeHorizontalDownFullS = 223,  // 0xDF -- sym

    // 0xE0, 0xE1, 0xE2, 0xE3

    MazeHorizontalUpFull = 228,  // 0xE4
    MazeHorizontalUpFullS = 229,  // 0xE5 -- sym

    MazeRoundUpRightFilled = 230, // 0xE6
    MazeRoundUpLeftFilled = 231, // 0xE7

    MazeFullVerticalRight = 232,   // 0xE8
    MazeFullVerticalLeft = 233,    // 0xE9 -- sym

    MazeRoundDownRightFilled = 234,  // 0xEA
    MazeRoundDownLeftFilled = 235,  // 0xEB -- sym

    MazeJailUpRight = 236,  // 0xEC
    MazeJailUpLeft = 237,  // 0xED
    MazeJailDownRight = 238,  // 0xEE
    MazeJailDownLeft = 239,  // 0xEF

    MazeJailHorizontalUp = 240, // 0xF0
    MazeJailHorizontalUpS = 241, // 0xF1

    MazeInternalRoundDownRightFilled = 242, // 0xF2
    MazeInternalRoundDownLeftFilled = 243, // 0xF3
    MazeInternalRoundUpRightFilled = 244, // 0xF4
    MazeInternalRoundUpLeftFilled = 245, // 0xF5

    // 0xF6: DotUpRight
    // 0xF7: DotUpLeft
    // 0xF8: DotDownRight
    // 0xF9: DotDownLeft

    MazeInternalRoundDownRightSimpleToFilled = 250,     // 0xFA
    MazeInternalRoundDownLeftSimpleToFilled = 251,  // 0xFB

    GhostJailFloor = 252,  // 0xFC
    GhostJailFloorSym = 253,  // 0xFD -- sym(252)

    // 0xFE, 0xFF

}

impl TileId {
    pub fn get_tile(&self, palette_id: ColorE) -> Tile {
        Tile::new(self, PALETTE[palette_id as usize])
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Tile<'a> {
    width: u16,
    height: u16,
    pixel_data: &'a [u16; 8],
    palette: [u32; 4],
}

impl<'a> Tile<'a> {

    pub fn from_id(id: usize, palette: [u32; 4]) -> Self {
        /* 2 bits per pixel => 2 bytes width */
        Tile {
            width: 8,
            height: 8,
            pixel_data: &TILE[id],
            palette: palette,
        }
    }

    pub fn new(id: &TileId, palette: [u32; 4]) -> Self {
        Tile::from_id(*id as usize, palette)
    }

    /// Get the image width in pixels
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get the image height in pixels
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get the raw image data contained in this image
    pub fn image_data(&self) -> &[u16; 8] {
        self.pixel_data
    }
}


impl<'a> IntoIterator for &'a Tile<'a> {
    type Item = Pixel;
    type IntoIter = TileIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TileIterator {
            tile: self.image_data(),
            palette: self.palette,
            seek: 0,
            shift: 14,   // 2 bits per pixel => 14, 12, 10, 8, 6, 4, 2
            x: 0,
            y: 0,

        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TileIterator<'a> {
    /// Reference to original Tile image
    tile: &'a [u16; 8],
    palette: [u32; 4],
    seek: usize,
    shift: usize,
    x: u32,
    y: u32,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seek == self.tile.len() {
            return None;
        }
        let value = (self.tile[self.seek] >> self.shift) & 0b11;
        let x = self.x;
        let y = self.y;

        if self.shift > 0 {
            self.x += 1;
            self.shift -= 2;
        } else {
            self.seek += 1;
            self.shift = 14;
            self.x = 0;
            self.y += 1;
        }

        Some(Pixel {
            x: x,
            y: y,
            color: self.palette[value as usize],
        })

    }
}


#[derive(Debug)]
pub struct EgPixelIterator<'a, C> {
    it: TileIterator<'a>,
    c: PhantomData<C>,
}

impl<'a, C> Iterator for EgPixelIterator<'a, C>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type Item = EgPixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|p| {
            let raw = C::Raw::from_u32(p.color);
            EgPixel(Point::new(p.x as i32, p.y as i32), raw.into())
        })
    }
}

impl ImageDimensions for Tile<'_> {
    fn width(&self) -> u32 {
        Tile::width(&self).into()
    }

    fn height(&self) -> u32 {
        Tile::height(&self).into()
    }
}


impl<'a, C> IntoPixelIter<C> for &'a Tile<'_>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
{
    type PixelIterator = EgPixelIterator<'a, C>;

    fn pixel_iter(self) -> Self::PixelIterator {
        EgPixelIterator {
            it: self.into_iter(),
            c: PhantomData,
        }
    }
}