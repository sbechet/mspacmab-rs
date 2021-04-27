use num_traits::FromPrimitive;

use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    prelude::*,
};

use embedded_graphics_simulator::{
    OutputSettingsBuilder,
    SimulatorDisplay, 
    Window, 
};

use crate::sprite::{SpriteId, Sprite};
use crate::tile::{TileId, Tile};
use crate::text::{TextId, Text};
use crate::palette::{PALETTE, ColorE};

pub const WIDTH: usize = 28;
pub const HEIGHT: usize = 36;
const MAX_SPRITES: usize = 8;

pub enum ScreenPart {
    All=0,
    Maze=1,
}

#[derive(Copy, Clone)]
pub struct SpriteElement {
    pub p: Point,
    pub s: SpriteId,
    pub c: ColorE,
}

pub struct GameHwVideo {
    pub window: Window,
    screen_tile: [[TileId; HEIGHT]; WIDTH],     // src:4000
    screen_color: [[u8; HEIGHT]; WIDTH],    // src:4400 (we use bit 7 to refresh)
    screen_sprite: [SpriteElement; MAX_SPRITES],    // src:4ff0 and src:5060
    pub sprites_enabled: bool,
    display: SimulatorDisplay<Rgb888>,
    display_reset: bool,
    updated: bool,
}


impl GameHwVideo {
    pub fn new() -> Self {
        let display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(WIDTH as u32 * 8, HEIGHT as u32 * 8));
        let output_settings = OutputSettingsBuilder::new().scale(4).build();

        GameHwVideo {
            window: Window::new("mspacmab", &output_settings),
            screen_tile: [ [TileId::Space; HEIGHT]; WIDTH],
            screen_color: [ [ColorE::Black as u8; HEIGHT]; WIDTH],
            screen_sprite: [ SpriteElement { p: Point {x: 0, y: 0}, s: SpriteId::Stork0, c: ColorE::Black}; MAX_SPRITES],
            sprites_enabled: false,
            display: display,
            display_reset: false,
            updated: true,
        }
    }

    /// push on "real" hardware
    /// TODO: push this call in a specific thread
    pub fn update(&mut self,) {
        if self.display_reset {
            self.display_reset = false;
            self.display.clear(Rgb888::BLACK).unwrap();
        }
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.screen_color[x][y]&0x80 == 0x80 {
                    self.screen_color[x][y] &= !0x80;
                    let tile = TileId::get_tile(&self.screen_tile[x][y], ColorE::from_u8(self.screen_color[x][y]).unwrap());
                    let image: Image<Tile, Rgb888> = Image::new(&tile, Point::new(x as i32 * 8, y as i32 * 8));
                    image.draw(&mut self.display).unwrap();    
                }
            }
        }

        if self.sprites_enabled {
            if self.updated {
                for spr in self.screen_sprite.iter() {
                    let s = Sprite::from_id(spr.s as usize, PALETTE[spr.c as usize]);
                    let image: Image<Sprite, Rgb888> = Image::new(&s, spr.p);
                    image.draw(&mut self.display).unwrap();
                }
                self.updated = false;
            }
        }

        self.window.update(&self.display);
    }

    // src:23ed
    pub fn clear_whole_screen_or_maze(&mut self, part:ScreenPart) {
        match part {
            // src:23f3
            ScreenPart::All => {
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        self.put_screen_tile(Point::new(x as i32,y as i32), TileId::Space);
                    }
                }
            },
            // src:2400
            ScreenPart::Maze => {
                for x in 0..WIDTH {
                    for y in 2..=33 {
                        self.put_screen_tile(Point::new(x as i32,y as i32), TileId::Space);
                    }
                }
            }
        }
    }

    // src:240d
    pub fn clear_color_ram(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.put_screen_color(Point::new(x as i32,y as i32), ColorE::Black);
            }
        }
    }

    // src:2b7e
    pub fn draw_big_tile_blank(&mut self, p: (i32, i32) ) {
        let t = TileId::Space;

        let point = Point::new(p.0, p.1);
        self.put_screen_tile(point, t);

        let point = Point::new(p.0 - 1, p.1);
        self.put_screen_tile(point, t);

        let point = Point::new(p.0, p.1 + 1);
        self.put_screen_tile(point, t);

        let point = Point::new(p.0 - 1, p.1 + 1);
        self.put_screen_tile(point, t);
    }

    // src:2b8f
    pub fn draw_big_tile(&mut self, t: TileId, p: (i32, i32) ) {
        let tnum = t as u8;

        let point = Point::new(p.0, p.1);
        self.put_screen_tile(point, t);

        let point = Point::new(p.0 - 1, p.1);
        let tnext = TileId::from_u8(tnum + 1).unwrap();
        self.put_screen_tile(point, tnext);

        let point = Point::new(p.0, p.1 + 1);
        let tnext = TileId::from_u8(tnum + 2).unwrap();
        self.put_screen_tile(point, tnext);

        let point = Point::new(p.0 - 1, p.1 + 1);
        let tnext = TileId::from_u8(tnum + 3).unwrap();
        self.put_screen_tile(point, tnext);
    }


    /// draw test_mode like fullscreen grid
    // src:3253, src:3ae2
    pub fn grid(&mut self, palette_id: ColorE) {
        for y in 0..18 {
            for x in 0..14 {
                self.screen_tile[x * 2][y * 2] = TileId::GridUpperLeft;
                self.screen_color[x * 2][y * 2] = palette_id as u8 | 0x80;

                self.screen_tile[x * 2 + 1][y * 2] = TileId::GridUpperRight; // fix
                self.screen_color[x * 2 + 1][y * 2] = palette_id as u8 | 0x80;

                self.screen_tile[x * 2 + 1][y * 2 + 1] = TileId::GridLowerRight;
                self.screen_color[x * 2 + 1][y * 2 + 1] = palette_id as u8 | 0x80;

                self.screen_tile[x * 2][y * 2 + 1] = TileId::GridLowerLeft; // fix
                self.screen_color[x * 2][y * 2 + 1] = palette_id as u8 | 0x80;
            }
        }
    }

    pub fn clear_tiles(&mut self) {
        self.screen_tile = [ [TileId::Space; HEIGHT]; WIDTH];
        self.screen_color = [ [ColorE::Black as u8 | 0x80; HEIGHT]; WIDTH];
        self.display_reset = true;  // not sure
    }

    pub fn clear_sprites(&mut self) {
        self.screen_sprite = [ SpriteElement { p: Point {x: 0, y: 0}, s: SpriteId::Stork0, c: ColorE::Black}; MAX_SPRITES];
        self.sprites_enabled = false;
        self.updated = true;
    }

    pub fn clear_test_mode(&mut self) {
        self.screen_tile = [ [TileId::Space; HEIGHT]; WIDTH];
        self.screen_color = [ [ColorE::ColorFruit as u8 | 0x80; HEIGHT]; WIDTH];
        self.clear_sprites();
        self.display_reset = true;
    }

    pub fn put_screen_tile(&mut self, p: Point, t:TileId) {
        let x = p.x as usize;
        let y = p.y as usize;
        // print!("({},{})",x,y);
        self.screen_tile[x][y] = t;
        self.screen_color[x][y] |= 0x80;
    }

    pub fn put_screen_color(&mut self, p: Point, c:ColorE) {
        let x = p.x as usize;
        let y = p.y as usize;
        // print!("({},{})",x,y);
        self.screen_color[x][y] = c as u8 | 0x80;
    }

    pub fn put_screen(&mut self, p: Point, t:TileId, c:ColorE) {
        let x = p.x as usize;
        let y = p.y as usize;
        // print!("({},{})",x,y);
        self.screen_tile[x][y] = t;
        self.screen_color[x][y] = c as u8 | 0x80;
    }

    pub fn get_screen(&mut self, p: Point) -> (TileId, ColorE) {
        let x = p.x as usize;
        let y = p.y as usize;
        let t = self.screen_tile[x][y];
        let c = ColorE::from_u8(self.screen_color[x][y]&!0x80).unwrap();

        (t, c)
    }

    pub fn put_sprite(&mut self, idx: usize, p: Point, s:SpriteId, c:ColorE) {
        self.screen_sprite[idx] = SpriteElement { p: p, s: s, c: c };
        self.sprites_enabled = true;
        self.updated = true;
    }

    pub fn get_sprite(&mut self, idx: usize) -> (SpriteId, ColorE) {
        (self.screen_sprite[idx].s, self.screen_sprite[idx].c)
    }

    pub fn put_text(&mut self, t:TextId) {
        let t = Text::get(t);
        // println!("TODO: update put_text:{}", t);
        t.draw_text(self, false);
    }

    pub fn put_text_id(&mut self, id:usize) {
        let t = Text::get_id(id);
        t.draw_text(self, false);
    }

    pub fn del_text(&mut self, t:TextId) {
        let t = Text::get(t);
        t.draw_text(self, true);
    }


}