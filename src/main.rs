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
use std::time::{ Instant, Duration };

// TODO: after full retro, create tree
mod mspacmab_data;
mod mspacmab_data_maze;
mod mspacmab_data_fruit;
mod mspacmab_data_animation;
mod credits;
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
mod score;
mod state_player;
mod game_attract;
mod game_playing;
mod game_task;
mod game_task_timed;
mod game;
mod ghost_difficulty;
mod fruit;

use test_mode::test_mode;
use game_hw_video::GameHwVideo;
use palette::{PALETTE, ColorE};
use tile::{TileId, Tile};
use sprite::{SpriteId, Sprite};
use text::{TextId, Text};
use game::Game;
use game_task::GameTask;


const SLOW_DOWN_GAME: u64 = 1; //10000;

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
    let mut g = Game::new();

    g.hwoutput.sound_enabled = true;
    g.hwoutput.flip_screen = false;
    g.hwoutput.lamp1 = false;
    g.hwoutput.lamp2 = false;
    g.hwoutput.coin_lockout = false;
    g.hwoutput.coin_counter = 0;
    g.hwvideo.clear_tiles();
    g.main_state_init_done = false;

    g.update();

    // let mut line = String::new();
    // let _input = std::io::stdin().read_line(&mut line).expect("Failed to read line");

    let mut vblank_time = Instant::now();

    'running: loop {
        let start = Instant::now();
        if g.hwinput.update(&mut g.hwvideo.window) == false {
            break;
        }

        let after_hw_update = Instant::now() - start;

        // vblank
        if g.hwinput.service_mode_1 {
            g.hwinput.service_mode_1 = ! test_mode(&mut g.hwinput, &mut g.hwoutput, &mut g.hwvideo, &mut g.hwsound, &mut g.main_state, &mut g.main_state_init_done);
            // hack
            if ! g.hwinput.service_mode_1 {
                g.main_state_init_done = false;
            }
    
        } else {
            g.timed_60_hz();
        }

        let after_timed = Instant::now() - start - after_hw_update;
        g.update();
        let after_update = Instant::now() - start - after_timed;

        // 60Hz
        // let now = Instant::now();
        // if now >= vblank_time {
        //     vblank_time = now + Duration::from_millis(SLOW_DOWN_GAME * 1/60); // remove 10000 for real speed
        //     println!("<VBlank>");
        //     g.timed_60_hz();
        //     g.update();
        // }

        // we assume no more than 1/60 / 10 for idle time
        // thread::sleep(Duration::from_millis(1/60 / 10));
        g.tasks.idle(&mut g.hwinput, &mut g.hwvideo, &mut g.hwsound, &mut g.credits, &mut g.score, &mut g.game_attract, &mut g.playing, &mut g.main_state, &mut g.main_state_init_done);
        let after_idle = Instant::now() - start - after_update;
        // println!("ahu:{:?}, at:{:?}, au:{:?}, ai:{:?}\n", after_hw_update, after_timed, after_update, after_idle);
    }

    return Ok(());
}
