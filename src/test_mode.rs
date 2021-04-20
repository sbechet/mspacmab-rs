use core::time::Duration;
use std::thread::sleep;

use embedded_graphics::{
    prelude::*,
};

use crate::game::{ Game, MainStateE };
use crate::hardware::{Bonus, Coinage, Live};
use crate::palette::ColorE;
use crate::text::TextId;
use crate::tile::TileId;

const WIDTH: usize = 28;
const HEIGHT: usize = 36;

// Extracted using mspacmab/maze_data sub-project
// src:3a4f
pub const NAMCO_EASTER_EGG: [ [char; 28]; 36] = [
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*','*','*',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*','*','*',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ',' ','*',' ',' ',' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ',' ',' ','*','*','*',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ',' ','*',' ',' ',' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ','*','*','*','*','*',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',' ','*','*','*',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ','*','*','*',' ',' ',' ',' ',' ','*',' ','*',' ',' ',' ',' ',' ','*','*','*','*',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ','*','*','*','*','*',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ',' ',' ','*',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ','*','*','*','*','*',' ',' ',' ','*','*','*','*','*',' ',' ',' ','*','*','*','*','*',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ','*','*','*','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*','*','*','*',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ','*',' ','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ','*',' ','*',' ',' ',' ',],
    [' ',' ',' ',' ',' ','*','*','*','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*','*','*',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ','*','*','*','*','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*','*','*','*',' ',' ',' ',],
    [' ',' ',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ','*','*','*','*',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ','*','*','*','*','*',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
    [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',],
];

// Because it is an historical project: "Hello" where you are Nakamura!
// src:97d0
const GCC_EASTER_EGG:&str = "GENERAL COMPUTER  CORPORATION   Hello, Nakamura!";

// src:3000
pub fn test_mode(g: &mut Game) -> bool {
    // ** First Part: general cleanup
    // src:3000
    g.hwvideo.clear_test_mode();
    // src:30f3
    g.hwvideo.put_text(TextId::MemoryOk);
    g.hwvideo.update();
    // src:3174
    g.hwoutput.coin_lockout = true;
    g.hwoutput.lamp1 = true;
    g.hwoutput.lamp2 = true;
    g.hwoutput.sound_enabled = true;
    // src:317d
    g.hwoutput.flip_screen = false;

    println!("You can close service_mode_1 (F3 on SDL simulator) for next step.");

    // ** Second Part: IO test
    // src:3188
    loop {
        // src:318b
        g.main_state_init_done = true;
        g.main_state = MainStateE::Init;

        if g.hwinput.update(&mut g.hwvideo.window) == false {
            return false;
        }

        if g.hwinput.coin_insert {
            g.hwsound.effect[0].num = 2;
        }
        if g.hwinput.player1_start_button || g.hwinput.player2_start_button {
            g.hwsound.effect[0].num = 1;
        }
        if g.hwinput.joystick1.up {
            g.hwsound.effect[2].num = 8;
        }
        if g.hwinput.joystick1.left {
            g.hwsound.effect[2].num = 4;
        }
        if g.hwinput.joystick1.right {
            g.hwsound.effect[2].num = 16;
        }
        if g.hwinput.joystick1.down {
            g.hwsound.effect[2].num = 32;
        }
        match g.hwinput.coinage {
            Coinage::FreePlay => g.hwvideo.put_text(TextId::FreePlayTest),
            Coinage::For1coin1credit => g.hwvideo.put_text(TextId::OneCoin1Credit),
            Coinage::For1coin2credits => g.hwvideo.put_text(TextId::OneCoin2Credits),
            Coinage::For2coins1credit => g.hwvideo.put_text(TextId::TwoCoins1Credit),
        }

        // src:31ea
        if let Bonus::None = g.hwinput.bonus {
            g.hwvideo.put_text(TextId::BonusNone);
        } else {
            g.hwvideo.put_text(TextId::Bonus);
            g.hwvideo.put_text(TextId::Zero00);
            match g.hwinput.bonus {
                Bonus::Pts10000 =>  {
                    g.hwvideo.put_screen_tile(Point::new(11,12), TileId::Number1);
                    g.hwvideo.put_screen_tile(Point::new(12,12), TileId::Number0);
                },
                Bonus::Pts15000 => {
                    g.hwvideo.put_screen_tile(Point::new(11,12), TileId::Number1);
                    g.hwvideo.put_screen_tile(Point::new(12,12), TileId::Number5);
                },
                Bonus::Pts20000 => {
                    g.hwvideo.put_screen_tile(Point::new(11,12), TileId::Number2);
                    g.hwvideo.put_screen_tile(Point::new(12,12), TileId::Number0);
                },
                _ => {},
            }
        }

        // src:321c
        match g.hwinput.live {
            Live::One => g.hwvideo.put_screen_tile(Point::new(16,14), TileId::Number1),
            Live::Two => g.hwvideo.put_screen_tile(Point::new(16,14), TileId::Number2),
            Live::Three => g.hwvideo.put_screen_tile(Point::new(16,14), TileId::Number3),
            Live::Six => g.hwvideo.put_screen_tile(Point::new(16,14), TileId::Number6),
        }

        // src:322d
        g.hwvideo.put_text(TextId::MsPacMen);

        if g.hwinput.cocktail_cabinet {
            g.hwvideo.put_text(TextId::Table);
        } else {
            g.hwvideo.put_text(TextId::Upright);
        }

        // Place the game in the test grid screen (Monitor Convergence screen) by switching test mode on.
        if g.hwinput.service_mode_1 {
            break;
        }

        // don't forget to update screen
        g.hwvideo.update();
        // and wait a little...because in loop it's good for CPU
        sleep(Duration::from_millis(500)); 
    }

    // src:3246
    g.hwoutput.coin_counter = 0;
    g.hwoutput.coin_lockout = false;
    g.hwoutput.lamp1 = false;
    g.hwoutput.lamp2 = false;
    g.hwoutput.flip_screen = false;
    g.hwoutput.sound_enabled = false;


    /*
        Check the condition to display the easter egg
        This piece of code is found in the original Midway Pac-Man ROMs @ #3289.
        Place the game in the test grid screen (Monitor Convergence screen) by switching test mode on.
        Next move the joystick:
        Up 4 times
        Left 4 times
        Right 4 times
        Down 4 times
            - Widel/Mowerman


        In fact:
        - service_mode_1 (F3 on SDL simulator)
        - press and retain the player 1 and player 2 buttons (F11 and F12 on SDL simulator)
        - !service_mode_1
        - service_mode_1 (before 500ms)
        - release buttons if you want
        - Up 4 times
        - Left 4 times
        - Right 4 times
        - Down 4 times

    */


    // ** Third Part: Check Easter egg

    // src:3253
    g.hwvideo.clear_tiles();
    g.hwvideo.grid(ColorE::ColorFruit);
    g.hwvideo.update();

    // src:327f
    sleep(Duration::from_millis(500));

    // Then, hold down the player 1 and player 2 buttons and then quickly jiggle the test switch out & back into test. 
    println!("Press and retain the player 1 and player 2 buttons (F11 and F12 on SDL simulator)");
    println!("...then open service_mode_1.");

    // switch out
    // src:3286
    loop {
        if g.hwinput.update(&mut g.hwvideo.window) == false {
            return false;
        }
        if ! g.hwinput.service_mode_1 {
            break;
        }
        // wait a little...because in loop it's good for CPU
        sleep(Duration::from_millis(500)); 
    }

    // src:3295
    if g.hwinput.player1_start_button && g.hwinput.player2_start_button {
        println!("OK! SPEEDUP: close service_mode_1");
        // src:3298
        sleep(Duration::from_millis(500));
        if g.hwinput.update(&mut g.hwvideo.window) == false {
            return false;
        }

        // back into test
        // src:329f
        if g.hwinput.service_mode_1 {
            println!("GOOD! You can release buttons if you want");
            // UP x 4
            println!("Please: Joystick #1 UP x 4");
            for _ in 0..4 {
                while ! g.hwinput.joystick1.up {
                    sleep(Duration::from_millis(69));
                    if g.hwinput.update(&mut g.hwvideo.window) == false {
                        return false;
                    }
                }
                while g.hwinput.joystick1.up || g.hwinput.joystick1.down || g.hwinput.joystick1.left || g.hwinput.joystick1.right {
                    sleep(Duration::from_millis(69));
                    if g.hwinput.update(&mut g.hwvideo.window) == false {
                        return false;
                    }
                }
            }
            // LEFT x 4
            println!("Please: Joystick #1 LEFT x 4");
            for _ in 0..4 {
                while ! g.hwinput.joystick1.left {
                    sleep(Duration::from_millis(69));
                    if g.hwinput.update(&mut g.hwvideo.window) == false {
                        return false;
                    }
                }
                while g.hwinput.joystick1.up || g.hwinput.joystick1.down || g.hwinput.joystick1.left || g.hwinput.joystick1.right {
                    sleep(Duration::from_millis(69));
                    if g.hwinput.update(&mut g.hwvideo.window) == false {
                        return false;
                    }
            
                }
            }
            // RIGHT x 4
            println!("Please: Joystick #1 RIGHT x 4");
            for _ in 0..4 {
                while ! g.hwinput.joystick1.right {
                    sleep(Duration::from_millis(69));
                    if g.hwinput.update(&mut g.hwvideo.window) == false {
                        return false;
                    }
            
                }
                while g.hwinput.joystick1.up || g.hwinput.joystick1.down || g.hwinput.joystick1.left || g.hwinput.joystick1.right {
                    sleep(Duration::from_millis(69));
                    if g.hwinput.update(&mut g.hwvideo.window) == false {
                        return false;
                    }
            
                }
            }
            // DOWN x 4
            println!("Please: Joystick #1 DOWN x 4");
            for _ in 0..4 {
                while ! g.hwinput.joystick1.down {
                    sleep(Duration::from_millis(69));
                    if g.hwinput.update(&mut g.hwvideo.window) == false {
                        return false;
                    }
            
                }
                while g.hwinput.joystick1.up || g.hwinput.joystick1.down || g.hwinput.joystick1.left || g.hwinput.joystick1.right {
                    sleep(Duration::from_millis(69));
                    if g.hwinput.update(&mut g.hwvideo.window) == false {
                        return false;
                    }
            
                }
            }

            draw_namco_easter_egg(g);

            // waiting end of service_mode_1
            while g.hwinput.service_mode_1 {
                if g.hwinput.update(&mut g.hwvideo.window) == false {
                    return true;
                }
            }

        } else {
            println!("Too late! Try again later.");
        }
    }
    return true;
}

// src:3af4
pub fn draw_namco_easter_egg(g: &mut Game) {
    println!("{}", GCC_EASTER_EGG);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if NAMCO_EASTER_EGG[y][x] == '*' {
                g.hwvideo.put_screen_tile(Point::new(x as i32,y as i32), TileId::Pill5);
            } else {
                g.hwvideo.put_screen_tile(Point::new(x as i32,y as i32), TileId::Space);
            }
        }
    }
}
