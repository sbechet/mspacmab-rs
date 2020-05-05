use core::marker::PhantomData;

use embedded_graphics::{
    drawable::Pixel as EgPixel,
    geometry::Point,
    image::{ImageDimensions, IntoPixelIter},
    pixelcolor::{raw::RawData, PixelColor},
};

use crate::palette::PALETTE;
use crate::pixel::Pixel;
use crate::mspacmab_data::{SPRITE};


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SpriteId {
    Sprite0 = 0,
    /* TODO */
}

impl SpriteId {
    pub fn get_sprite(&self, palette_id: u8) -> Sprite {
        Sprite::new(self, PALETTE[palette_id as usize])
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Sprite<'a> {
    width: u16,
    height: u16,
    pixel_data: &'a [u32; 16],
    palette: [u32; 4],
}

impl<'a> Sprite<'a> {

    pub fn from_id(id: usize, palette: [u32; 4]) -> Self {
        /* 2 bits per pixel => 4 bytes width */
        Sprite {
            width: 16,
            height: 16,
            pixel_data: &SPRITE[id],
            palette: palette,
        }
    }

    pub fn new(id: &SpriteId, palette: [u32; 4]) -> Self {
        Sprite::from_id(*id as usize, palette)
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