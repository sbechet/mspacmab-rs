use core::marker::PhantomData;

use embedded_graphics::{
    drawable::Pixel as EgPixel,
    geometry::Point,
    image::{ImageDimensions, IntoPixelIter},
    pixelcolor::{raw::RawData, PixelColor},
};

use crate::palette::{ ColorE, PALETTE };
use crate::pixel::Pixel;
use crate::mspacmab_data::{SPRITE};


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SpriteId {
    Cherry=0,
    Strawberry=1,
    Orange=2,
    Pretzel=3,
    Apple=4,
    Pear=5,
    Banana=6,
    PearSmall=7,

    S100=8,
    S200=9,
    S500=10,
    S700=11,
    S1000=12,
    S2000=13,
    S5000=14,

    ManSmallLeft=15,

    Gfx10=16,
    Gfx11=17,

    GfxActLeft=18,
    GfxActRight=19,

    Gfx14=20,
    Gfx15=21,
    Gfx16=22,
    Gfx17=23,
    Gfx18=24,

    ManRight2=25,
    ManDown2=26,
    ManRight1=27,

    GhostFrozen1=28,
    GhostFrozen2=29,

    Heart=30,
    Empty=31,

    GhostRight1=32,
    GhostRight2=33,
    GhostDown1=34,
    GhostDown2=35,
    GhostLeft1=36,
    GhostLeft2=37,
    GhostUp1=38,
    GhostUp2=39,

    T200=40,
    T400=41,
    T800=42,
    T1600=43,

    ManStart=44,

    MsTurn1=45,
    ManDown1=46,
    MsRight2=47,
    Gfx30=48,
    MsTurn2=49,

    MsBack=50,
    MsDown2=51,
    MsDown1=52,
    MsLeft1=53,
    MsUp1=54,
    MsRight1=55,
    MsDown1Bis=56,
    MsLeft1Bis=57,
    MsUp1Bis=58,
    MsRight1Bis=59,
    MsDown1Ter=60,
    MsLeft1Ter=61,
    MsUp1Ter=62,
    FruitStart=63,

    CodeForNoSprite=255,
}

impl SpriteId {
    pub fn get_sprite(&self, palette_id: ColorE) -> Sprite {
        Sprite::new(self, PALETTE[palette_id as usize])
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Sprite<'a> {
    width: u16,
    height: u16,
    pixel_data: &'a [u32; 16],
    palette: [u32; 4],
    flip: bool,
}

impl<'a> Sprite<'a> {

    pub fn from_id(id: usize, palette: [u32; 4]) -> Self {
        /* 2 bits per pixel => 4 bytes width */
        Sprite {
            width: 16,
            height: 16,
            pixel_data: &SPRITE[id],
            palette: palette,
            flip: false,
        }
    }

    pub fn new(id: &SpriteId, palette: [u32; 4]) -> Self {
        Sprite::from_id(*id as usize, palette)
    }

    pub fn set_flip(&mut self, flip: bool) {
        self.flip = flip;
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
    pub fn image_data(&self) -> &[u32; 16] {
        // TODO: implement self.flip
        self.pixel_data
    }
}


impl<'a> IntoIterator for &'a Sprite<'a> {
    type Item = Pixel;
    type IntoIter = SpriteIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SpriteIterator {
            sprite: self.image_data(),
            palette: self.palette,
            seek: 0,
            shift: 30,   // 2 bits per pixel => 30, 28, 26, 24, 22, 20, 18, 16, 14, 12, 10, 8, 6, 4, 2
            x: 0,
            y: 0,

        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SpriteIterator<'a> {
    /// Reference to original Sprite image
    sprite: &'a [u32; 16],
    palette: [u32; 4],
    seek: usize,
    shift: usize,
    x: u32,
    y: u32,
}

impl<'a> Iterator for SpriteIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seek == self.sprite.len() {
            return None;
        }
        let value = (self.sprite[self.seek] >> self.shift) & 0b11;
        let x = self.x;
        let y = self.y;

        if self.shift > 0 {
            self.x += 1;
            self.shift -= 2;
        } else {
            self.seek += 1;
            self.shift = 30;
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
    it: SpriteIterator<'a>,
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

impl ImageDimensions for Sprite<'_> {
    fn width(&self) -> u32 {
        Sprite::width(&self).into()
    }

    fn height(&self) -> u32 {
        Sprite::height(&self).into()
    }
}


impl<'a, C> IntoPixelIter<C> for &'a Sprite<'_>
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