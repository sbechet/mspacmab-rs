use sdl2::{
    keyboard::Keycode,
};

use embedded_graphics_simulator::{
    SimulatorEvent,
    Window, 
};


#[derive(Copy, Clone, Debug)]
pub enum Coinage {
    FreePlay=0,
    For1coin1credit=1,
    For1coin2credits=2,
    For2coins1credit=3,
}

#[derive(Copy, Clone, Debug)]
pub enum Live {
    One=0,
    Two=1,
    Three=2,
    Six=3,
}

#[derive(Debug)]
pub enum Bonus {
    Pts10000=0,
    Pts15000=1,
    Pts20000=2,
    None=3,
}

#[derive(Debug)]
pub struct Joystick {
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
}

#[derive(Debug)]
pub struct HardwareInput {
    // In0
    // src:5000
    // write true = enable hardware interrupts
    pub joystick1: Joystick,
    pub rack_test: bool,
    pub coin_insert: bool,
    pub service_mode_0: bool,           // force level end (cf. src:8cd)
    // In1
    // src:5040
    pub joystick2: Joystick,
    pub service_mode_1: bool,           // true = push button reset board
    pub player1_start_button: bool,
    pub player2_start_button: bool,
    pub cocktail_cabinet: bool,
    // dip switch Dsw1
    // src:5080
    pub coinage: Coinage,
    pub live: Live,
    pub bonus: Bonus,
    pub hard_game: bool,
    pub change_ghost_names: bool,
}

// see game_hw_audio for audio
pub struct HardwareOutput {
    // src:5001
    pub sound_enabled: bool,
    // src:5003
    pub flip_screen: bool,  // TODO must go to game_hw_video one day
    // src:5004
    pub lamp1: bool,
    // src:5005
    pub lamp2: bool,
    // src:5006
    pub coin_lockout: bool,
    // src:5007
    pub coin_counter: u8,
    // src:50c0
    pub watchdog: u8,   // hard reset 0xfc at start
}

impl Joystick {
    pub fn new() -> Joystick {
        Joystick {
            up: false,
            left: false,
            right: false,
            down: false,
        }
    }

    pub fn clean(&mut self) {
        self.up = false;
        self.left = false;
        self.right = false;
        self.down = false;
    }
}

/**
    Using SDL2 simulator mode:
        F1 : rack_test switch
        F2 : service_mode_0 switch
        F3 : service_mode_1 switch
        F4 : cocktail_cabinet switch
        F5 : Coinage::FreePlay, For1coin1credit, For1coin2credits, For2coins1credit
        F6 : Live::One, ...Two, Three, Six
        F7 : Bonus::Pts10000, Pts15000, Pts20000, None
        F8 : Hard Game switch
        F9 : Change Ghost names switch

        F11: player1 start button
        F12: player2 start button

        Player 1: left, right, up, down arrow
        Player 2: w, x, o, k

        Insert: Insert coin

    */
impl HardwareInput {
    pub fn new() -> HardwareInput {
        HardwareInput {
            // In0
            joystick1: Joystick::new(),
            rack_test: false,
            coin_insert: false,
            service_mode_0: false,
            // In1
            joystick2: Joystick::new(),
            service_mode_1: false,
            player1_start_button: false,
            player2_start_button: false,
            cocktail_cabinet: false,
            // dip switch
            coinage: Coinage::FreePlay,     // TODO: must change to finish 03fe/0413/execute_DEMO_task_state_patch()!
            live: Live::Six,
            bonus: Bonus::Pts20000,
            hard_game: false,
            change_ghost_names: false,
        }
    }

    pub fn debug(&self) {
        println!("{:?}", self);
    }

    #[cfg(feature = "simulator")]
    pub fn update(&mut self, w: &mut Window) -> bool {

        // Events
        for event in w.events() {
            match event {
                SimulatorEvent::KeyDown { keycode, keymod, repeat } => {
                    if ! repeat {
                        // println!("Event : {:?}", event);
                        match keycode {
                            Keycode::Up => self.joystick1.up = true,
                            Keycode::Down => self.joystick1.down = true,
                            Keycode::Left => self.joystick1.left = true,
                            Keycode::Right => self.joystick1.right = true,
                            Keycode::F11 => self.player1_start_button = true,

                            Keycode::O => self.joystick2.up = true,
                            Keycode::K => self.joystick2.down = true,
                            Keycode::W => self.joystick2.left = true,
                            Keycode::X => self.joystick2.right = true,
                            Keycode::F12 => self.player2_start_button = true,

                            Keycode::Insert => self.coin_insert = true,

                            _ => {},
                        }
                    }
                },
                SimulatorEvent::KeyUp { keycode, keymod, repeat } => {
                    if ! repeat {
                        // println!("Event : {:?}", event);
                        match keycode {
                            Keycode::Up => self.joystick1.up = false,
                            Keycode::Down => self.joystick1.down = false,
                            Keycode::Left => self.joystick1.left = false,
                            Keycode::Right => self.joystick1.right = false,
                            Keycode::F11 => self.player1_start_button = false,

                            Keycode::O => self.joystick2.up = false,
                            Keycode::K => self.joystick2.down = false,
                            Keycode::W => self.joystick2.left = false,
                            Keycode::X => self.joystick2.right = false,
                            Keycode::F12 => self.player2_start_button = false,

                            Keycode::Insert => self.coin_insert = false,

                            /* switch configuration */
                            Keycode::F1 => self.rack_test = ! self.rack_test,
                            Keycode::F2 => self.service_mode_0 = ! self.service_mode_0,
                            Keycode::F3 => self.service_mode_1 = ! self.service_mode_1,
                            Keycode::F4 => self.cocktail_cabinet = ! self.cocktail_cabinet,
                            Keycode::F5 => {
                                self.coinage = match self.coinage {
                                    Coinage::FreePlay => Coinage::For1coin1credit,
                                    Coinage::For1coin1credit => Coinage::For1coin2credits,
                                    Coinage::For1coin2credits => Coinage::For2coins1credit,
                                    Coinage::For2coins1credit => Coinage::FreePlay,
                                };
                            },
                            Keycode::F6 => {
                                self.live = match self.live {
                                    Live::One => Live::Two,
                                    Live::Two => Live::Three,
                                    Live::Three => Live::Six,
                                    Live::Six => Live::One,
                                };
                            },
                            Keycode::F7 => {
                                self.bonus = match self.bonus {
                                    Bonus::Pts10000 => Bonus::Pts15000,
                                    Bonus::Pts15000 => Bonus::Pts20000, 
                                    Bonus::Pts20000 => Bonus::None,
                                    Bonus::None => Bonus::Pts10000,
                                };
                            },
                            Keycode::F8 => self.hard_game = ! self.hard_game,
                            Keycode::F9 => self.change_ghost_names = ! self.change_ghost_names,
                            _ => {},
                        }
                    }
                },
                SimulatorEvent::MouseButtonUp { mouse_btn: _, point: _ } => self.player1_start_button = true,
                SimulatorEvent::Quit => return false,
                _ => {
                    println!("Event : {:?}", event);
                    self.debug();
                }
            }
        }

        return true;
    }

    #[cfg(not(feature = "simulator"))]
    pub fn update(&self) {
        // MODIFY DIP SWITCH AND HARDWARE CONFIG HERE
        // TODO

        // update all here
    }


}

impl HardwareOutput {
    pub fn new() -> HardwareOutput {
        HardwareOutput {
            sound_enabled: false,
            flip_screen: false,
            lamp1: false,
            lamp2: false,
            coin_lockout: false,
            coin_counter: 0,
            watchdog: 0xfc,
        }
    }
    pub fn update(&mut self) {
        // WATCHDOG
        self.watchdog -= 1;
        if self.watchdog == 0 {
            self.watchdog = 0xfc;
            println!("watchdog timeout!");
        }
    }
}