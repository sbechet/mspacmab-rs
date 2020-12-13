use crate::game::Game;

pub trait GamePlaying {
    fn execute_playing_task_state(&mut self);
}

impl GamePlaying for Game {
    // src:06be
    fn execute_playing_task_state(&mut self) {
    }
}
