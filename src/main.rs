use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    prelude::*,
};
use embedded_graphics_simulator::{
    SimulatorDisplay, 
    SimulatorEvent,
    Window, 
    OutputSettingsBuilder
};

use std::thread;
use std::time::Duration;

mod mspacmab_data;
mod mspacmab_data_maze;
mod mspacmab_data_fruit;
mod hardware;
mod game_hw_video;
mod game_hw_sound;
mod game_counter;
mod bgr233;
mod palette;
mod pixel;
mod tile;
mod sprite;
mod sprite_element;
mod text;
mod test_mode;
mod game_demo;
mod game_task;
mod game_task_timed;
mod game;

use test_mode::test_mode;
use game_hw_video::GameHwVideo;
use palette::{PALETTE, ColorE};
use tile::{TileId, Tile};
use sprite::{SpriteId, Sprite};
use text::{TextId, Text};
use game::{Game};


// big ms-pacman
fn print_tile_big_mspacman(hwvideo: &mut GameHwVideo, x: i32, y:i32, c:ColorE) {
    hwvideo.put_screen( Point::new(x  ,y  ), TileId::MspacBigUpperLeft,  c);
    hwvideo.put_screen( Point::new(x+1,y  ), TileId::MspacBigUpperRight, c);
    hwvideo.put_screen( Point::new(x+1,y+1), TileId::MspacBigLowerRight, c);
    hwvideo.put_screen( Point::new(x  ,y+1), TileId::MspacBigLowerLeft,  c);
}

// big heart
fn print_tile_heart(hwvideo: &mut GameHwVideo, x: i32, y:i32, c:ColorE) {
    hwvideo.put_screen( Point::new(x  ,y  ), TileId::HeartUpperLeft,  c);
    hwvideo.put_screen( Point::new(x+1,y  ), TileId::HeartUpperRight,  c);
    hwvideo.put_screen( Point::new(x+1,y+1), TileId::HeartLowerRight,  c);
    hwvideo.put_screen( Point::new(x  ,y+1), TileId::HeartLowerLeft,  c);
}

// use std::thread;
// use std::thread::sleep;
// use std::time::Duration;

// fn timed_60_hz() {
//     let mut i = 0;
//     loop {
//         println!("timed fn running: {}", i);
//         i += 1;
//         i %= 60;
//         // For 60 Hz:
//         // ----------
//         // Freq (Hz) | Buffer Size (Bytes)
//         // 44_100    | 735 (44100/60)
//         // 48_000    | 800
//         // 96_000    | 1600
//         sleep(Duration::from_micros(1_000_000/60));
//     }
// }

// fn create_timed_60_hz_thread(g:&mut Game) -> Result<(), std::io::Error> {
//     thread::Builder::new().name("timed_60_hz".to_string()).spawn(move || {
//         g.timed_60_hz();
//     })?;

//     Ok(())
// }

fn main() -> Result<(), core::convert::Infallible> {
    // let mut hwvideo = GameHwVideo::new_simulator();


    // hwvideo.put_screen( Point { x:1, y:1 }, TileId::LetterA, ColorE::Yellow);
    // hwvideo.put_sprite( 0, Point {x:32, y:32 }, SpriteId::ManStart, ColorE::Yellow);
    // print_tile_big_mspacman(&mut hwvideo, 8, 8, ColorE::Yellow);
    // print_tile_heart(&mut hwvideo, 16, 16, ColorE::Red);
    // for i in 0..55 {
    //     hwvideo.put_text_id(i);
    // }

    // let output_settings = OutputSettingsBuilder::new().scale(1).build();
    // let mut window = Window::new("mspacmab", &output_settings);
    // let mut g = Game::new(&mut hwvideo);
    let mut g = Game::new();

    if test_mode(&mut g) == false {
        return Ok(());
    }
    println!("Return from test_mode()");
    g.hwoutput.sound_enabled = true;
    g.hwoutput.flip_screen = false;
    g.hwoutput.lamp1 = false;
    g.hwoutput.lamp2 = false;
    g.hwoutput.coin_lockout = false;
    g.hwoutput.coin_counter = 0;
    g.hwvideo.clear_tiles();
    g.subroutine_init_state = 0;

    g.update();

    // let mut line = String::new();
    // let _input = std::io::stdin().read_line(&mut line).expect("Failed to read line");

    'running: loop {
        if g.hwinput.update(&mut g.hwvideo.window) == false {
            break;
        }
        // 60Hz
        thread::sleep(Duration::from_millis(10000*1/60));   // TODO: Change for real-timer
        g.timed_60_hz();
        g.update();
        g.idle();
    }

    return Ok(());
}
