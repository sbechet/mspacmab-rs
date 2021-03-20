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
    Stork0=24,

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

    Stork1=44,

    MsRight3=45,       // Closed mouth - ManLeft3 w/ flip x (173)
    ManDown1=46,
    MsRight2=47,       // ManLeft2 w/ flip x (175)
    Stork2=48,
    MsTurn2=49,

    MsBack=50,
    MsDown2=51,
    MsDown1=52,
    MsLeft1=53,
    MsUp1=54,
    MsRight1=55,        // ManLeft1 w/ flip x (183)
    MsDown1Bis=56,
    MsLeft1Bis=57,
    MsUp1Bis=58,
    MsRight1Bis=59,
    MsDown1Ter=60,
    MsLeft1Ter=61,
    MsUp1Ter=62,
    FruitStart=63,

    /* FlipY */
    CherryFlipY=0 | 0x40,
    StrawberryFlipY=1 | 0x40,
    OrangeFlipY=2 | 0x40,
    PretzelFlipY=3 | 0x40,
    AppleFlipY=4 | 0x40,
    PearFlipY=5 | 0x40,
    BananaFlipY=6 | 0x40,
    PearSmallFlipY=7 | 0x40,

    S100FlipY=8 | 0x40,
    S200FlipY=9 | 0x40,
    S500FlipY=10 | 0x40,
    S700FlipY=11 | 0x40,
    S1000FlipY=12 | 0x40,
    S2000FlipY=13 | 0x40,
    S5000FlipY=14 | 0x40,

    ManSmallLeftFlipY=15 | 0x40,

    Gfx10FlipY=16 | 0x40,
    Gfx11FlipY=17 | 0x40,

    GfxActLeftFlipY=18 | 0x40,
    GfxActRightFlipY=19 | 0x40,

    Gfx14FlipY=20 | 0x40,
    Gfx15FlipY=21 | 0x40,
    Gfx16FlipY=22 | 0x40,
    Gfx17FlipY=23 | 0x40,
    Stork0FlipY=24 | 0x40,

    ManRight2FlipY=25 | 0x40,
    ManDown2FlipY=26 | 0x40,
    ManRight1FlipY=27 | 0x40,

    GhostFrozen1FlipY=28 | 0x40,
    GhostFrozen2FlipY=29 | 0x40,

    HeartFlipY=30 | 0x40,
    EmptyFlipY=31 | 0x40,

    GhostRight1FlipY=32 | 0x40,
    GhostRight2FlipY=33 | 0x40,
    GhostDown1FlipY=34 | 0x40,
    GhostDown2FlipY=35 | 0x40,
    GhostLeft1FlipY=36 | 0x40,
    GhostLeft2FlipY=37 | 0x40,
    GhostUp1FlipY=38 | 0x40,
    GhostUp2FlipY=39 | 0x40,

    T200FlipY=40 | 0x40,
    T400FlipY=41 | 0x40,
    T800FlipY=42 | 0x40,
    T1600FlipY=43 | 0x40,

    Stork1FlipY=44 | 0x40,

    MsRight3FlipY=45 | 0x40,       // Closed mouth - ManLeft3 w/ flip x (173)
    ManDown1FlipY=46 | 0x40,
    MsRight2FlipY=47 | 0x40,       // ManLeft2 w/ flip x (175)
    Stork2FlipY=48 | 0x40,
    MsTurn2FlipY=49 | 0x40,

    MsBackFlipY=50 | 0x40,
    MsDown2FlipY=51 | 0x40,
    MsDown1FlipY=52 | 0x40,
    MsLeft1FlipY=53 | 0x40,
    MsUp1FlipY=54 | 0x40,
    MsRight1FlipY=55 | 0x40,        // ManLeft1 w/ flip x (183)
    MsDown1BisFlipY=56 | 0x40,
    MsLeft1BisFlipY=57 | 0x40,
    MsUp1BisFlipY=58 | 0x40,
    MsRight1BisFlipY=59 | 0x40,
    MsDown1TerFlipY=60 | 0x40,
    MsLeft1TerFlipY=61 | 0x40,
    MsUp1TerFlipY=62 | 0x40,
    FruitStartFlipY=63 | 0x40,

    /* FlipX */
    CherryFlipX=0 | 0x80,
    StrawberryFlipX=1 | 0x80,
    OrangeFlipX=2 | 0x80,
    PretzelFlipX=3 | 0x80,
    AppleFlipX=4 | 0x80,
    PearFlipX=5 | 0x80,
    BananaFlipX=6 | 0x80,
    PearSmallFlipX=7 | 0x80,

    S100FlipX=8 | 0x80,
    S200FlipX=9 | 0x80,
    S500FlipX=10 | 0x80,
    S700FlipX=11 | 0x80,
    S1000FlipX=12 | 0x80,
    S2000FlipX=13 | 0x80,
    S5000FlipX=14 | 0x80,

    ManSmallLeftFlipX=15 | 0x80,

    Gfx10FlipX=16 | 0x80,
    Gfx11FlipX=17 | 0x80,

    GfxActLeftFlipX=18 | 0x80,
    GfxActRightFlipX=19 | 0x80,

    Gfx14FlipX=20 | 0x80,
    Gfx15FlipX=21 | 0x80,
    Gfx16FlipX=22 | 0x80,
    Gfx17FlipX=23 | 0x80,
    Stork0FlipX=24 | 0x80,

    ManRight2FlipX=25 | 0x80,
    ManDown2FlipX=26 | 0x80,
    ManRight1FlipX=27 | 0x80,

    GhostFrozen1FlipX=28 | 0x80,
    GhostFrozen2FlipX=29 | 0x80,

    HeartFlipX=30 | 0x80,
    EmptyFlipX=31 | 0x80,

    GhostRight1FlipX=32 | 0x80,
    GhostRight2FlipX=33 | 0x80,
    GhostDown1FlipX=34 | 0x80,
    GhostDown2FlipX=35 | 0x80,
    GhostLeft1FlipX=36 | 0x80,
    GhostLeft2FlipX=37 | 0x80,
    GhostUp1FlipX=38 | 0x80,
    GhostUp2FlipX=39 | 0x80,

    T200FlipX=40 | 0x80,
    T400FlipX=41 | 0x80,
    T800FlipX=42 | 0x80,
    T1600FlipX=43 | 0x80,

    Stork1FlipX=44 | 0x80,

    MsRight3FlipX=45 | 0x80,       // Closed mouth - ManLeft3 w/ flip x (173)
    ManDown1FlipX=46 | 0x80,
    MsRight2FlipX=47 | 0x80,       // ManLeft2 w/ flip x (175)
    Stork2FlipX=48 | 0x80,
    MsTurn2FlipX=49 | 0x80,

    MsBackFlipX=50 | 0x80,
    MsDown2FlipX=51 | 0x80,
    MsDown1FlipX=52 | 0x80,
    MsLeft1FlipX=53 | 0x80,
    MsUp1FlipX=54 | 0x80,
    MsRight1FlipX=55 | 0x80,        // ManLeft1 w/ flip x (183)
    MsDown1BisFlipX=56 | 0x80,
    MsLeft1BisFlipX=57 | 0x80,
    MsUp1BisFlipX=58 | 0x80,
    MsRight1BisFlipX=59 | 0x80,
    MsDown1TerFlipX=60 | 0x80,
    MsLeft1TerFlipX=61 | 0x80,
    MsUp1TerFlipX=62 | 0x80,
    FruitStartFlipX=63 | 0x80,

    /* FlipXFlipY */
    CherryFlipXFlipY=0 | 0xC0,
    StrawberryFlipXFlipY=1 | 0xC0,
    OrangeFlipXFlipY=2 | 0xC0,
    PretzelFlipXFlipY=3 | 0xC0,
    AppleFlipXFlipY=4 | 0xC0,
    PearFlipXFlipY=5 | 0xC0,
    BananaFlipXFlipY=6 | 0xC0,
    PearSmallFlipXFlipY=7 | 0xC0,

    S100FlipXFlipY=8 | 0xC0,
    S200FlipXFlipY=9 | 0xC0,
    S500FlipXFlipY=10 | 0xC0,
    S700FlipXFlipY=11 | 0xC0,
    S1000FlipXFlipY=12 | 0xC0,
    S2000FlipXFlipY=13 | 0xC0,
    S5000FlipXFlipY=14 | 0xC0,

    ManSmallLeftFlipXFlipY=15 | 0xC0,

    Gfx10FlipXFlipY=16 | 0xC0,
    Gfx11FlipXFlipY=17 | 0xC0,

    GfxActLeftFlipXFlipY=18 | 0xC0,
    GfxActRightFlipXFlipY=19 | 0xC0,

    Gfx14FlipXFlipY=20 | 0xC0,
    Gfx15FlipXFlipY=21 | 0xC0,
    Gfx16FlipXFlipY=22 | 0xC0,
    Gfx17FlipXFlipY=23 | 0xC0,
    Stork0FlipXFlipY=24 | 0xC0,

    ManRight2FlipXFlipY=25 | 0xC0,
    ManDown2FlipXFlipY=26 | 0xC0,
    ManRight1FlipXFlipY=27 | 0xC0,

    GhostFrozen1FlipXFlipY=28 | 0xC0,
    GhostFrozen2FlipXFlipY=29 | 0xC0,

    HeartFlipXFlipY=30 | 0xC0,
    EmptyFlipXFlipY=31 | 0xC0,

    GhostRight1FlipXFlipY=32 | 0xC0,
    GhostRight2FlipXFlipY=33 | 0xC0,
    GhostDown1FlipXFlipY=34 | 0xC0,
    GhostDown2FlipXFlipY=35 | 0xC0,
    GhostLeft1FlipXFlipY=36 | 0xC0,
    GhostLeft2FlipXFlipY=37 | 0xC0,
    GhostUp1FlipXFlipY=38 | 0xC0,
    GhostUp2FlipXFlipY=39 | 0xC0,

    T200FlipXFlipY=40 | 0xC0,
    T400FlipXFlipY=41 | 0xC0,
    T800FlipXFlipY=42 | 0xC0,
    T1600FlipXFlipY=43 | 0xC0,

    Stork1FlipXFlipY=44 | 0xC0,

    MsRight3FlipXFlipY=45 | 0xC0,       // Closed mouth - ManLeft3 w/ flip x (173)
    ManDown1FlipXFlipY=46 | 0xC0,
    MsRight2FlipXFlipY=47 | 0xC0,       // ManLeft2 w/ flip x (175)
    Stork2FlipXFlipY=48 | 0xC0,
    MsTurn2FlipXFlipY=49 | 0xC0,

    MsBackFlipXFlipY=50 | 0xC0,
    MsDown2FlipXFlipY=51 | 0xC0,
    MsDown1FlipXFlipY=52 | 0xC0,
    MsLeft1FlipXFlipY=53 | 0xC0,
    MsUp1FlipXFlipY=54 | 0xC0,
    MsRight1FlipXFlipY=55 | 0xC0,        // ManLeft1 w/ flip x (183)
    MsDown1BisFlipXFlipY=56 | 0xC0,
    MsLeft1BisFlipXFlipY=57 | 0xC0,
    MsUp1BisFlipXFlipY=58 | 0xC0,
    MsRight1BisFlipXFlipY=59 | 0xC0,
    MsDown1TerFlipXFlipY=60 | 0xC0,
    MsLeft1TerFlipXFlipY=61 | 0xC0,
    MsUp1TerFlipXFlipY=62 | 0xC0,

    CodeForNoSprite=63 | 0xC0,  // 255
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
    flip_x: bool,
    flip_y: bool,
}

impl<'a> Sprite<'a> {

    pub fn from_id(id: usize, palette: [u32; 4]) -> Self {
        /* 2 bits per pixel => 4 bytes width */
        Sprite {
            width: 16,
            height: 16,
            pixel_data: &SPRITE[id & 0x3F],
            palette: palette,
            flip_x: id&0x80 != 0,
            flip_y: id&0x40 != 0,
        }
    }

    pub fn new(id: &SpriteId, palette: [u32; 4]) -> Self {
        Sprite::from_id(*id as usize, palette)
    }

    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }

    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
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
        // TODO: implement self.flip_x, self.flip_y
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
            flip_x: self.flip_x,    // TODO: implement x
            flip_y: self.flip_y,    // TODO: implement y
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
    flip_x: bool,
    flip_y: bool,
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