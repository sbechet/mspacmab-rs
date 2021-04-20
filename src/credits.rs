use embedded_graphics::prelude::*;

use crate::game::MainStateE;
use crate::game_attract::GameAttract;
use crate::game_counter::CurrentTime;
use crate::game_hw_video::{ GameHwVideo, ScreenPart };
use crate::game_hw_sound::{ SoundChannels, Wave };
use crate::game_playing::GamePlaying;
use crate::game_task::{ GameTask, TaskCoreE };
use crate::game_task_timed::{ GameTaskTimed, TaskTimedNameE };
use crate::hardware::{ HardwareInput, HardwareOutput, Coinage, Bonus };
use crate::palette::{ColorE};
use crate::text::{ TextId, Text };
use crate::tile::TileId;

pub struct Credits {
    // src:4dce
    pub counter_blink_for_lights_coin_and_players: u8,
    // src:4dd6
    pub can_led_blink: bool,

    // src:4e03
    pub subroutine_coin_inserted_state: u8,

    // COINS, CREDITS                                                                                                                                                                                                                                                           *
    // src:4e66
    input_state_coin_service0: u8,
    // src:4e67
    input_state_coin_insert: u8,
    // src:4e68
    input_state_coin_rack: u8,
    // src:4e69 - Coin->credts, this gets decremented
    counter_coin: u8,
    // src:4e6a - Used to write coin counters
    counter_coin_timeout: u8,
    // src:4e6b - These are copied from the dipswitches
    number_of_coins_per_credit: u8,
    // src:4e6c
    number_of_coins_inserted: u8,
    // src:4e6d
    number_of_credits_per_coin: Coinage,
    // src:4e6e
    pub number_of_credits: u8,
    // src:4e6f
    number_of_lives: u8,
    // src:4e70
    pub number_of_players: u8,      // 0=1 player, 1=2 players
    // src:4e71
    pub bonus: u8,                  // (10, 15, 20, 255) * 1000
    // src:4e75
    pub ghost_names_mode: bool,
}

impl Credits {
    pub fn new() -> Credits {
        Credits {
            counter_blink_for_lights_coin_and_players: 0,
            can_led_blink: false,

            subroutine_coin_inserted_state: 0,
            
            input_state_coin_service0: 0,
            input_state_coin_insert: 0,
            input_state_coin_rack: 0,
            counter_coin: 0,
            counter_coin_timeout: 0,
            number_of_coins_per_credit: 0,
            number_of_coins_inserted: 0,
            number_of_credits_per_coin: Coinage::FreePlay,
            number_of_credits: 0,
            number_of_lives: 0,
            number_of_players: 0,
            bonus: 10,
            ghost_names_mode: false,
        }
    }

    /* src:0267 */
    // debounce rack input / add credits (if 99 or over, return)
    pub fn rack_input_add_credits(&mut self, hwinput: &HardwareInput, hwoutput: &mut HardwareOutput, hwsound: &mut SoundChannels) {
        // limit because screen drawing
        if self.number_of_credits >= 99 {
            hwoutput.coin_lockout = true;
            return;
        }

        // use service_0 switch on/off to add coin
        self.input_state_coin_service0 &= 0b0000_0111;
        self.input_state_coin_service0 <<= 1;
        self.input_state_coin_service0 |= hwinput.service_mode_0 as u8;
        if self.input_state_coin_service0 == 0b0000_1100 {
            self.coins_to_credits(hwsound);
        }

        // use real money to add coin
        self.input_state_coin_insert &= 0b0000_0111;
        self.input_state_coin_insert <<= 1;
        self.input_state_coin_insert |= hwinput.coin_insert as u8;
        if self.input_state_coin_insert == 0b0000_1100 {
          self.counter_coin+=1;
        }

        // use rack_test switch on/off to add coin
        self.input_state_coin_rack &= 0b0000_0111;
        self.input_state_coin_rack <<= 1;
        self.input_state_coin_rack |= hwinput.rack_test as u8;
        if self.input_state_coin_rack == 0b0000_1100 {
            self.counter_coin+=1;
        }

    }
    /* src:02ad - debounce coin input / add credits */
    pub fn debounce_coin_input(&mut self, hwoutput: &mut HardwareOutput, hwsound: &mut SoundChannels) {
        if self.counter_coin != 0 {
            if self.counter_coin_timeout == 0 {
                hwoutput.coin_counter = 1;
                self.coins_to_credits(hwsound);
            }
            if self.counter_coin_timeout == 8 {
                hwoutput.coin_counter = 0;
            }
            if self.counter_coin_timeout < 10 {
                self.counter_coin_timeout+=1;
            } else {
                self.counter_coin_timeout=0;
                self.counter_coin-=1;
            }
        }
    }
    /* src:02df */
    fn coins_to_credits(&mut self, hwsound: &mut SoundChannels) {
        self.number_of_coins_inserted+=1;
        if self.number_of_coins_inserted >= self.number_of_coins_per_credit {
            self.number_of_coins_inserted -= self.number_of_coins_per_credit;
            self.number_of_credits += self.number_of_credits_per_coin as u8;
            if self.number_of_credits > 99 {
                // limit because screen drawing
                self.number_of_credits = 99;
            }
            // sound insert
            hwsound.effect[0].num = 1;
        }
    }

    /* src:02fd */
    pub fn blink_coin_lights(&mut self, hwoutput: &mut HardwareOutput, hwvideo: &mut GameHwVideo, playing: &mut GamePlaying, main_state: MainStateE) {

        self.counter_blink_for_lights_coin_and_players = self.counter_blink_for_lights_coin_and_players.wrapping_add(1);

        // ======================== Blink coin lights
        if self.counter_blink_for_lights_coin_and_players & 0xf == 0 {
            // lamp off when credits inserted, else use counter bit 4
            let onoff = !self.can_led_blink |
                        ((self.counter_blink_for_lights_coin_and_players >> 4) & 1 == 1);

            let (lamp1, lamp2) = match self.number_of_credits {
                0 => (false, false),
                1 => (onoff, false),
                _ => (onoff, onoff)
            };

            hwoutput.lamp1 = lamp1;
            hwoutput.lamp2 = lamp2;
        }

        // ============================= 1UP & 2UP
        if main_state != MainStateE::Playing && self.subroutine_coin_inserted_state < 2 {
            Text::print(hwvideo,  (3, 0), "1UP");
            Text::print(hwvideo, (22, 0), "2UP");
        } else {
            // display and blink 1UP/2UP depending on player up
            if playing.current_player == 0 {
                if self.counter_blink_for_lights_coin_and_players & 0x10 == 0 {
                    Text::print(hwvideo, (3, 0), "1UP");
                } else {
                    Text::print(hwvideo, (3, 0), "   ");
                }
            } else {
                if self.counter_blink_for_lights_coin_and_players & 0x10 == 0 {
                    Text::print(hwvideo, (22, 0), "2UP");
                } else {
                    Text::print(hwvideo, (22, 0), "   ");
                }
            }

            if self.number_of_players == 0 {
                Text::print(hwvideo, (22, 0), "   ");
            }
        }
    }

    pub fn state_machine(&mut self, timed_task: &mut GameTaskTimed, tasks: &mut GameTask, hwinput: &HardwareInput, hwvideo: &mut GameHwVideo, game_attract: &mut GameAttract, playing: &mut GamePlaying, wave: &mut [Wave; 3], main_state: &mut MainStateE, ) {
        match self.subroutine_coin_inserted_state {
            0 => {
                // src:05f3
                self.t1d_draw_credit_qty(hwvideo);
                tasks.add(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                tasks.add(TaskCoreE::SelectMazeColor(0));
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::PushStartButton, false));
                // Midway logo and copyright text
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::AdditionalAt000Pts, false));
                tasks.add(TaskCoreE::ClearFruitAndPacmanPosition);
                self.subroutine_coin_inserted_state += 1;
                self.can_led_blink = true;
                if self.bonus != 255 {
                    tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::TileMsPacMan, false));
                    tasks.add(TaskCoreE::DrawExtraLifePoints);
                }
            },
            1 => {
                // src:061b
                // display credits qty
                self.t1d_draw_credit_qty(hwvideo);
                // display number of players using credits qty
                let player_tile = if self.number_of_credits == 1 {
                    TextId::OnePlayerOnly
                } else {
                    TextId::OneOrTwoPlayers
                };
                hwvideo.put_text(player_tile);

                // player want to play?

                if self.number_of_credits != 1 && hwinput.player2_start_button {
                    self.number_of_players = 1;
                } else if hwinput.player1_start_button {
                    self.number_of_players = 0;
                } else {
                    // no start button has been pressed
                    return;
                }

                // Check FreePlay
                if self.number_of_coins_per_credit != 0 {
                    // two players
                    if self.number_of_players == 1 {
                        self.number_of_credits -= 1;
                    }
                    self.number_of_credits -= 1;
                    self.t1d_draw_credit_qty(hwvideo);
                }
        
                self.subroutine_coin_inserted_state += 1;
                self.can_led_blink = false;
                wave[0].num = 1;
                wave[1].num = 1;
            },
            2 => {
                // src:0674
                tasks.add(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::Maze));
                tasks.add(TaskCoreE::SelectMazeColor(1));
                tasks.add(TaskCoreE::DrawMaze);
                tasks.add(TaskCoreE::ClearsPillsAndPowerPills);
                tasks.add(TaskCoreE::DrawPellets);
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::PlayerOne, false));
                tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Ready, false));
                tasks.add(TaskCoreE::ResetThenPrintPlayersScore);
                tasks.add(TaskCoreE::DrawFruitsBottomRightScreen);
                playing.state_player[playing.current_player].level = 0;
                playing.state_player[playing.current_player].real_number_of_lives = self.number_of_lives;
                playing.state_player[playing.current_player].number_of_lives_displayed = self.number_of_lives;
                tasks.add(TaskCoreE::DrawRemainingLivesBottomLeftScreen);
                timed_task.add(CurrentTime::LessTenth, 23, TaskTimedNameE::IncreaseSubroutineCoinInsertedState);
                // TODO: HACK: must add manual inc here. Why? Original bug?
                self.subroutine_coin_inserted_state += 1;   //XXX
            },
            3 => {
                // src:000c
                // RET
            },
            4 => {
                // src:06a8
                playing.state_player[playing.current_player].number_of_lives_displayed -= 1;
                self.t1a_draw_remaining_lives_bottom_screen(hwvideo, playing, *main_state);
                *main_state = MainStateE::Playing;
                game_attract.subroutine_attract_state = 0;
                self.subroutine_coin_inserted_state = 0;
                playing.subroutine_playing_state = 0;
            },
            _ => {},
        }
    }

    // src:26d0
    pub fn setup_config_from_dip_switches(&mut self, hwinput: &HardwareInput, playing: &mut GamePlaying) {
        self.number_of_credits_per_coin = hwinput.coinage;
        if let Coinage::FreePlay = self.number_of_credits_per_coin {
            self.number_of_credits = 255;
        }
        /*
        number_of_coins_per_credit => number_of_credits_per_coin
                                    0 => 0
                                    1 => 1
                                    1 => 2
                                    2 => 1
            Historical : 
                number_of_coins_per_credit = (number_of_credits_per_coin >> 1) + (number_of_credits_per_coin & 1);
                number_of_credits_per_coin = number_of_coins_per_credit & 2 ^ number_of_credits_per_coin;
        */
        self.number_of_coins_per_credit = match self.number_of_credits_per_coin {
            Coinage::FreePlay => 0,
            Coinage::For1coin1credit => 1,
            Coinage::For1coin2credits => 2,
            Coinage::For2coins1credit => 1,
        };

        /* check dip switches 2 and 3.  number of starting lives per game */
        self.number_of_lives = hwinput.live as u8 + 1;
        if self.number_of_lives == 4 {
            self.number_of_lives += 2;
        }

        /* check dip switches 4 and 5.  points for bonus pac man */
        /* CONFIG for extra life
            10 for 10.000 pts
            15 for 15.0000 pts
            20 for 20_0000 pts
            FF for no extra life
        */
        self.bonus = match hwinput.bonus {
            Bonus::Pts10000 => 10,
            Bonus::Pts15000 => 15,
            Bonus::Pts20000 => 20,
            Bonus::None     => 255,
        };

        /* check dip switch 7 for ghost names during attract mode */
        self.ghost_names_mode = hwinput.change_ghost_names;

        /* check dip switch 6 for difficulty */
        playing.is_hard_game = hwinput.hard_game; // RUST HACK: not a ref but a bool here

        /* check bit 7 on IN1 for upright / cocktail */
        playing.cocktail = hwinput.cocktail_cabinet;
    }

    // src:2b4a
    pub fn draw_lives(&mut self, hwvideo: &mut GameHwVideo, number_of_lives: u8) {
        let mut places = 5;
        let mut x = 3;  // TODO: 1?

        // first draw lives
        if number_of_lives != 0 && number_of_lives < 6 {
            for _i in 0..number_of_lives {
                // draw 16x16 Ms Pacman 
                hwvideo.draw_big_tile(TileId::MspacBigUpperRight, (x,34));
                x += 2;
                places -= 1;
            }
        }
        // clean last ones
        for _i in 0..places {
            hwvideo.draw_big_tile_blank( (x,34) );
            x += 2;
        }
    }

    // src:2b6a
    pub fn t1a_draw_remaining_lives_bottom_screen(&mut self, hwvideo: &mut GameHwVideo, playing: &mut GamePlaying, main_state: MainStateE) {
        if main_state != MainStateE::Attract {
            return;   // do not update screen
        }
        self.set_bottom_left_background_to_yellow(hwvideo);
        self.draw_lives(hwvideo, playing.state_player[playing.current_player].number_of_lives_displayed);
    }

    // src:2ba1
    pub fn t1d_draw_credit_qty(&mut self, hwvideo: &mut GameHwVideo) {
        /* display number of credits */
        if self.number_of_credits == 255 {
            hwvideo.put_text(TextId::FreePlay);
        } else {
            hwvideo.put_text(TextId::Credit);
            let text = format!("{}", self.number_of_credits);
            Text::print(hwvideo, (9, 35), &text);
        }
    }

    // src:2bcd
    fn set_bottom_left_background_to_yellow(&mut self, hwvideo: &mut GameHwVideo) {
        for y in 0..2 {
            for x in 1..=11 {
                let p = Point::new(x, 34+y);
                hwvideo.put_screen_color(p, ColorE::Yellow);
            }
        }
    }

}