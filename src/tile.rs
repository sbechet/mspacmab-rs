use core::marker::PhantomData;

use embedded_graphics::{
    drawable::Pixel as EgPixel,
    geometry::Point,
    image::{ImageDimensions, IntoPixelIter},
    pixelcolor::{raw::RawData, PixelColor},
};

use crate::palette::PALETTE;
use crate::pixel::Pixel;
use crate::mspacmab_data::{TILE};


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
    Number0 =   48,
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
    LetterA = 65,
    LetterB = 66,
    LetterC = 67,
    LetterD = 68,
    LetterE = 69,
    LetterF = 70,
    LetterG = 71,
    LetterH = 72,
    LetterI = 73,
    LetterJ = 74,
    LetterK = 75,
    LetterL = 76,
    LetterM = 77,
    LetterN = 78,
    LetterO = 79,
    LetterP = 80,
    LetterQ = 81,
    LetterR = 82,
    LetterS = 83,
    LetterT = 84,
    LetterU = 85,
    LetterV = 86,
    LetterW = 87,
    LetterX = 88,
    LetterY = 89,
    LetterZ = 90,
    Exclamation = 91,
    Copyright = 92,
    PtsPt = 93,
    PtsTs = 94,
    Ptss = 95,
    /* TODO .. 96..=255 */
}

impl TileId {
    pub fn get_tile(&self, palette_id: u8) -> Tile {
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