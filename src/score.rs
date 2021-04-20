use crate::credits::Credits;
use crate::game::MainStateE;
use crate::game_hw_sound::SoundChannels;
use crate::game_hw_video::GameHwVideo;
use crate::game_playing::GamePlaying;
use crate::text::Text;

pub struct Score {
    // src:4e80, src:4e83, src:4e84, src:4e87
    // (score, got_bonus_life)
    pub score: [(u32, bool); 2],
    // src:4e88
    high_score: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            score: [(0, false); 2],
            high_score: 0,
        }
    }

    pub fn reset_score_players(&mut self) {
        self.score = [(0, false); 2];
    }

    // src:2a5a
    pub fn T19_update_score_then_draw(&mut self, hwvideo: &mut GameHwVideo, hwsound: &mut SoundChannels, playing: &mut GamePlaying, credits: &mut Credits, main_state: MainStateE, score_index: usize) {
        // src:2b17
        const SCORE_TABLE:[u16; 14] = [
            10,   // dot             =   10  0
            50,   // power pellet    =   50  1
            200,  // ghost 1         =  200  2
            400,  // ghost 2         =  400  3
            800,  // ghost 3         =  800  4
            1600, // ghost 4         = 1600  5
            100,  // Cherry          =  100  6
            200,  // 300 in pac-man
            500,  // Orange          =  500  8
            700,  // Pretzel         =  700  9
            1000, // Apple           = 1000  10
            2000, // Pear            = 2000  11
            5000, // 3000 in pac-man
            5000, // Junior!         = 5000  12
        ];

        if main_state == MainStateE::Attract {
            return;
        }

        let score_to_add = SCORE_TABLE[score_index as usize] as u32;
        let bonus = 1000 * credits.bonus as u32;
        let iplayer = playing.current_player;

        self.score[iplayer].0 += score_to_add;
        let sp = self.score[iplayer];

        if sp.0  >= bonus {
            // src:2b33
            if ! sp.1 {
                self.score[iplayer].1 = true;

                hwsound.effect[0].num |= 1;
                playing.state_player[iplayer].real_number_of_lives += 1;
                playing.state_player[iplayer].number_of_lives_displayed += 1;
                credits.draw_lives(hwvideo, playing.state_player[iplayer].number_of_lives_displayed);
            }
        }

        self.draw_player_score_on_screen(hwvideo, playing);
        if sp.0 > self.high_score {
            self.high_score = sp.0;
            self.draw_score_to_screen(hwvideo, self.high_score, (13, 1));    // src:43f2
        }
    }

    // src:2aaf
    pub fn draw_player_score_on_screen(&mut self, hwvideo: &mut GameHwVideo, playing: &GamePlaying) {
        let c = if playing.current_player == 0 {
            ( 5, 1) // src:43fc
        } else {
            (24, 1) // src:43e9
        };
        self.draw_score_to_screen(hwvideo, self.score[playing.current_player].0, c);
    }

    // src:2abe
    pub fn draw_score_to_screen(&mut self, hwvideo: &mut GameHwVideo, score:u32, p:(u8, u8) ) {
        // Printed text format must be '00' not '0' for 0
        // let text = format!("{:5}{:1}", score / 10, score % 10);
        let text = format!("{:02}", score);
        Text::print(hwvideo, p, &text);
    }

}