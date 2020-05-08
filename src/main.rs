use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    prelude::*,
};
use embedded_graphics_simulator::{
    SimulatorDisplay, 
    Window, 
    OutputSettingsBuilder
};

mod mspacmab_data;
mod bgr233;
mod palette;
mod pixel;
mod tile;
mod sprite;
mod text;
mod test_mode;

use palette::PALETTE;
use tile::{TileId, Tile};
use sprite::{SpriteId, Sprite};
use text::{TextId, Text};
use test_mode::{test_mode, clear_screen};


struct Resources {
    // #[init(Vec::new())]
    // task:Task,   // = Vec::new();
}

fn init() {
}


fn idle(r: Resources) {
    loop {
        /* Si tâche dans la FIFO, executer la tâche */

    }
}

// big ms-pacman
fn print_tile_big_mspacman(display: &mut SimulatorDisplay<Rgb888>, x: i32, y:i32, c:u8) {
    let tile = TileId::get_tile(&TileId::MspacBigUpperLeft, c);
    Image::new(&tile, Point::new(x,y)).draw(display).unwrap();

    let tile = TileId::get_tile(&TileId::MspacBigUpperRight, c);
    Image::new(&tile, Point::new(x+8,y)).draw(display).unwrap();

    let tile = TileId::get_tile(&TileId::MspacBigLowerRight, c);
    Image::new(&tile, Point::new(x+8,y+8)).draw(display).unwrap();

    let tile = TileId::get_tile(&TileId::MspacBigLowerLeft, c);
    Image::new(&tile, Point::new(x,y+8)).draw(display).unwrap();
}

// big heart
fn print_tile_heart(display: &mut SimulatorDisplay<Rgb888>, x: i32, y:i32, c:u8) {
    let tile = TileId::get_tile(&TileId::HeartUpperLeft, c);
    Image::new(&tile, Point::new(x,y)).draw(display).unwrap();

    let tile = TileId::get_tile(&TileId::HeartUpperRight, c);
    Image::new(&tile, Point::new(x+8,y)).draw(display).unwrap();

    let tile = TileId::get_tile(&TileId::HeartLowerRight, c);
    Image::new(&tile, Point::new(x+8,y+8)).draw(display).unwrap();

    let tile = TileId::get_tile(&TileId::HeartLowerLeft, c);
    Image::new(&tile, Point::new(x,y+8)).draw(display).unwrap();
}



fn print_tile(display: &mut SimulatorDisplay<Rgb888>, tile: u8, x: i32, y:i32, c:u8) {
    let t = Tile::from_id(tile as usize, PALETTE[c as usize]);
    let image: Image<Tile, Rgb888> = Image::new(&t, Point::new(x,y));
    image.draw(display).unwrap();
}

fn print_sprite(display: &mut SimulatorDisplay<Rgb888>, sprite: u8, x: i32, y:i32, c:u8) {
    let s = Sprite::from_id(sprite as usize, PALETTE[c as usize]);
    let image: Image<Sprite, Rgb888> = Image::new(&s, Point::new(x,y));
    image.draw(display).unwrap();
}

fn check_tile_colors(display: &mut SimulatorDisplay<Rgb888>) {
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let nums = vec![1,3,5,7,9,14,15,16,17,18,20,21,22,23,24,25,26,27,29,30,31];

    for m in nums {
        println!("palette_id = {}", m);
        for n in 0..255 {
            let x = n % 16;
            let y = n / 16;
            // println!("({}, {}, {})", x, y, m);
            let t1 = Tile::from_id(n, PALETTE[m]);
            let image: Image<Tile, Rgb888> = Image::new(&t1, Point::new( (x*8) as i32, (y*8) as i32));
            image.draw(display).unwrap();
        }
        Window::new("check_tile_colors", &output_settings).show_static(&display);
    }
}

fn check_sprite_colors(display: &mut SimulatorDisplay<Rgb888>) {
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let nums = vec![1,3,5,7,9,14,15,16,17,18,20,21,22,23,24,25,26,27,29,30,31];

    for m in nums {
        println!("palette_id = {}", m);
        for n in 0..64 {
            let x = n % 8;
            let y = n / 8;
            // println!("({}, {}, {})", x, y, m);
            let s = Sprite::from_id(n, PALETTE[m]);
            let image: Image<Sprite, Rgb888> = Image::new(&s, Point::new( (x*16) as i32, (y*16) as i32));
            image.draw(display).unwrap();
        }
        Window::new("check_sprite_colors", &output_settings).show_static(&display);
    }
}


fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(28*8, 36*8));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();

    test_mode(&mut display);
    print_tile(&mut display, 65, 1, 1, 9);
    print_sprite(&mut display, 0, 32, 32, 9);
    print_tile_big_mspacman(&mut display, 64, 64, 9);
    print_tile_heart(&mut display, 128, 128, 1);
    // Window::new("mspacmab", &output_settings).show_static(&display);
    clear_screen(&mut display);
    for i in 0..55 {
        let t = Text::get_id(i);
        t.draw_text(&mut display, true);
    }
    Window::new("mspacmab", &output_settings).show_static(&display);

    // check_tile_colors(&mut display);
    // check_sprite_colors(&mut display);

    Ok(())
}
