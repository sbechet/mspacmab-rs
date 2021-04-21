use crate::score::Score;
use crate::hardware::{ HardwareInput, HardwareOutput };
use crate::game_hw_video::{ GameHwVideo, ScreenPart };
use crate::game_hw_sound::{ SoundChannels, Wave };
use crate::credits::Credits;
// use crate::test_mode::{ test_mode };
use crate::game_attract::GameAttract;
use crate::game_playing::GamePlaying;
use crate::game_task::{GameTask, TaskCoreE};
use crate::game_task_timed::GameTaskTimed;

#[derive(PartialEq, Copy, Clone)]
pub enum MainStateE {
    Init=0,
    Attract=1,
    CoinInserted=2,
    Playing=3,
}

pub struct Game {
    pub hwvideo: GameHwVideo,
    pub hwinput: HardwareInput,
    pub hwoutput: HardwareOutput,
    /* src:4cc0 src:4c80 src:4c82 */
    pub tasks: GameTask,
    // src:4c90
    pub timed_task: GameTaskTimed,
    // src:4d00
    pub playing: GamePlaying,
    // src:4e00
    pub main_state: MainStateE,
    // src:4e01
    pub main_state_init_done: bool,
    pub credits: Credits,

    pub score: Score,
    pub game_attract: GameAttract,

    // src:4ecc, src:4edc, src:4eec
    wave: [Wave; 3],
    // src:4e8c, src:4e92, src:4e97
    // src:4e9c, src:4eac, src:4ebc
    // src:4ecc, src:4edc, src:4eec
    // src:5041, src:5046, src:504b
    pub hwsound: SoundChannels,
}

impl Game {

    pub fn new() -> Self {
        Game {
            hwvideo: GameHwVideo::new(),
            hwsound: SoundChannels::new(),
            hwinput: HardwareInput::new(),
            hwoutput: HardwareOutput::new(),
            main_state: MainStateE::Init,   // src:4e00
            main_state_init_done: false,    // src:4e01
            credits: Credits::new(),
            score: Score::new(),
            tasks: GameTask::new(),
            timed_task: GameTaskTimed::new(),
            playing: GamePlaying::new(),
            game_attract: GameAttract::new(),
            wave: [Wave::new(); 3],

        }
    }

    /// push on "real" hardware
    pub fn update(&mut self) {
        for i in 0..6 {
            self.hwvideo.put_sprite(i+1, self.playing.sprite[i].p, self.playing.sprite[i].s, self.playing.sprite[i].c);
        }
        self.hwvideo.update();
    }

    /* MEMORY_MAP: program_rom1 **********************************************/



    /* src:03c8 */
    fn change_mode(&mut self) {
        match self.main_state {
            MainStateE::Init => {
                if ! self.main_state_init_done {
                    /* src:03dc */
                    println!("change_mode/Init");
                    self.tasks.add(TaskCoreE::ClearWholeScreenOrMaze(ScreenPart::All));
                    self.tasks.add(TaskCoreE::ClearColorRam);
                    self.tasks.add(TaskCoreE::SelectMazeColor(0));
                    self.tasks.add(TaskCoreE::SetupConfigFromDipSwitches);
                    self.tasks.add(TaskCoreE::ResetThenPrintPlayersScore);
                    self.tasks.add(TaskCoreE::ResetSpritesToDefaultValues(false));
                    self.tasks.add(TaskCoreE::ClearFruitAndPacmanPosition);
                    self.tasks.add(TaskCoreE::SetGameToAttractMode);
                    self.main_state_init_done = true;
                    self.hwoutput.sound_enabled = true;
                }
            },
            MainStateE::Attract => {
                /* src:03fe */
                println!("change_mode/Attract");
                self.credits.t1d_draw_credit_qty(&mut self.hwvideo);
                if self.credits.number_of_credits != 0 {
                    self.main_state = MainStateE::CoinInserted;   // +=1
                    self.game_attract.subroutine_attract_state = 0;
                    self.playing.subroutine_playing_state = 0;
                } else {
                    self.game_attract.execute_attract_task_state_patch(&mut self.timed_task, &mut self.tasks, &mut self.hwvideo, &mut self.hwsound, &self.hwinput, &mut self.hwoutput, &mut self.playing, &self.main_state);
                }
            },
            MainStateE::CoinInserted => {
                /* src:05e5 */
                println!("change_mode/CoinInserted/{}", self.credits.subroutine_coin_inserted_state);
                self.credits.state_machine(&mut self.timed_task, &mut self.tasks, &mut self.hwinput, &mut self.hwvideo, &mut self.game_attract, &mut self.playing, &mut self.wave, &mut self.main_state);
            },
            MainStateE::Playing => {
                /* src:06be */
                println!("change_mode/Playing");
                self.playing.execute_playing_task_state(&mut self.timed_task, &mut self.tasks, &mut self.hwvideo, &mut self.hwsound, &self.hwinput, &mut self.hwoutput, &self.main_state, self.game_attract.subroutine_attract_state);
            },
        }
    }

    /*
        1. SOUND
            copy freqs to hardware (channel1, channel2, channel3)
            for each channel :
                hardware wave_t.sel configuration
                if channel_X_wave.num == 0:
                    wave_select_X = channel_X_effect.table[0]
        2. SPRITES PREPARE FOR HARDWARE
            // 4C02..4C20 TO 4C22..4C40
                all sprites data to hardware_prepare (*8):
                    sprite_flip_id_color_t[7] and sprites_coord_t[8]
            // 4C22..4C2C :
                red, pink, blue, orange, man, fruit : rotate to put xy on lower part?

        3. KILLED GHOST ANIMATION
            if (killed_ghost_animation_state == 1) {
                ... action on 4C22..4C2C = BUFF PREPARE FOR HARDWARE
            }
        4. POWER PILL ANIMATION
            if (power_pill_effect) {
                ... action on 4C22..4C2C = BUFF PREPARE FOR HARDWARE
            }
        
        5. SPRITES PREPARED TO HARDWARE
            // COPY 4C22..4C2E TO 4FF2..5000 (flipx, flipy, spriteid, palette)
            // COPY 4C32..4C40 TO 5060..5070 (x, y)

    */
    // src:
    pub fn timed_60_hz(&mut self) {
        // SoundChannels::channel[0].set_wave 

        // 6. VARIABLE THINGS
        // src:018c
        self.timed_task.counter.update();
        self.timed_task.run(&mut self.tasks, &mut self.playing, &mut self.game_attract, &mut self.credits);
        self.change_mode();
        match self.main_state {
            MainStateE::Init => {
                // src:01b3
                // channel_2_effect.num = 0;
                // channel_3_effect.num = 0;
            },
            _ => {
                // src:019b
                // check_for_double_size_pacman(); TODO: NEVER USED
                // no_cocktail_mode_update_sprites();
                // cocktail_mode_update_sprites();
                self.credits.rack_input_add_credits(&mut self.hwinput, &mut self.hwoutput, &mut self.hwsound);
                self.credits.debounce_coin_input(&mut self.hwoutput, &mut self.hwsound);
                self.credits.blink_coin_lights(&mut self.hwoutput, &mut self.hwvideo, &mut self.playing, self.main_state);
            }
        }

        // 7. SOUND GAME
        // process_effect_all_voices();
        // process_waves_all_voices();

        // 8. MSPACMAN intermission
        //     see backup_sprites__then__check_cocktail_animation_end();
        //     if animation_enable {
        //         copy sprites informations to intermisson buffer
        //     }
        //     ..


    }




    // src:2b0b - not used


    /* MEMORY_MAP: /program_rom1 *********************************************/


    

    // pub fn hw_set_orientation(&self, left:bool, right: bool, up: bool, down: bool) {

    // }

    // pub fn hw_sound(&self, enable:bool) {

    // }


}