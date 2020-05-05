use embedded_graphics::{
    pixelcolor::raw::RawU8, 
    prelude::*
};

/*
    An encoded palette byte contains RGB information bit-packed as follows:
       
          bit: 7 6 5 4 3 2 1 0
        color: b b g g g r r r
*/

/// Bgr233 color - 8 bits per pixel
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Bgr233(RawU8);

impl Bgr233 {
    /// Creates a Bgr233 color.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        let b3_r = (red as usize * 8 / 256) & 0b111;
        let b3_g = (green as usize * 8 / 256) & 0b111;
        let b3_b = (blue as usize * 4 / 256) & 0b11;
    
        let value:u8 = ( b3_r | (b3_g<<3) | (b3_b<<6) ) as u8;

        Self(RawU8::new(value))
    }
}

/// Implement `PixelColor` to associate a raw data type with the `Bgr233` struct.
impl PixelColor for Bgr233 {
    type Raw = RawU8;
}

/// `From<RawU8>` is used by `Image` to construct Bgr233 colors.
impl From<RawU8> for Bgr233 {
    fn from(data: RawU8) -> Self {
        Self(data)
    }
}
