use crate::game::Game;
use crate::game_counter::{CurrentTime, Counter60Hz};
use crate::game_task::{TaskCoreE};
use crate::text::{TextId};

const MAX_TASKTIMED: usize = 16;

#[derive(Copy, Clone)]
pub enum TaskTimedNameE {
    IncreaseSubroutinePlayingState,             // 0 src:0894
    IncreaseSubroutineCoinInsertedState,        // 1 src:06a3
    IncreaseSubroutineDemoState,                // 2 src:058e
    IncreaseKilledGhostAnimationState,          // 3 src:1272
    ClearFruitPoints,                           // 3 src:1000
    ClearFruitPosition,                         // 4 src:100b
    ClearReadyMessage,                          // 5 src:0263
    IncreaseStateIn1stCutescene,                // 6 src:212b
    IncreaseStateIn2ndCutescene,                // 7 src:21f0
    IncreaseStateIn3rdCutescene,                // 8 src:22b9
}

#[derive(Copy, Clone)]
pub struct TaskTimedE {
    unit:CurrentTime,
    counter:u8,
    task:TaskTimedNameE,
}

pub struct GameTaskTimed {
    /* src:4c90 */
    tasks: [TaskTimedE; MAX_TASKTIMED],
}

impl GameTaskTimed {
    pub fn new() -> Self {
        let empty_task = TaskTimedE {
            unit: CurrentTime::None,
            counter: 0,
            task: TaskTimedNameE::IncreaseSubroutinePlayingState,
        };

        GameTaskTimed {
            tasks: [empty_task; MAX_TASKTIMED],
        }
    }

    pub fn add(&mut self, unit: CurrentTime, counter: u8, t: TaskTimedNameE) {
        for tasktimed in self.tasks.iter_mut() {
            if tasktimed.unit == CurrentTime::None {
                tasktimed.unit = unit;
                tasktimed.counter = counter;
                tasktimed.task = t;
                break;
            }
        }
    }

    // src:0221
    fn execute_timed_task(&mut self, g: &mut Game) {
        for tasktimed in self.tasks.iter_mut() {
            if g.counter.get() == tasktimed.unit {
                tasktimed.counter -= 1;
                if tasktimed.counter == 0 {
                    tasktimed.unit = CurrentTime::None;
                    match tasktimed.task {
                        // 0 src:0894
                        TaskTimedNameE::IncreaseSubroutinePlayingState => {
                            g.subroutine_playing_state += 1;
                        },        
                        // 1 src:06a3
                        TaskTimedNameE::IncreaseSubroutineCoinInsertedState => {
                            g.subroutine_coin_inserted_state += 1;
                        },
                        // 2 src:058e
                        TaskTimedNameE::IncreaseSubroutineDemoState => {
                            g.subroutine_demo_state += 1;
                        },
                        // 3 src:1272
                        TaskTimedNameE::IncreaseKilledGhostAnimationState => {
                            g.killed_ghost_animation_state += 1;
                        },
                        // 3 src:1000
                        TaskTimedNameE::ClearFruitPoints => {
                            g.fruit_points = 0;
                        },
                        // 4 src:100b
                        TaskTimedNameE::ClearFruitPosition => {
                            g.fruit_coord = (0, 0);
                        },
                        // 5 src:0263
                        TaskTimedNameE::ClearReadyMessage => {
                            g.task.add_to_task_list(TaskCoreE::DrawTextOrGraphics(TextId::Ready, true) );
                        },
                        // 6 src:212b
                        TaskTimedNameE::IncreaseStateIn1stCutescene => {
                            g.state_in_first_cutscene += 1;
                        },
                        // 7 src:21f0
                        TaskTimedNameE::IncreaseStateIn2ndCutescene => {
                            g.state_in_second_cutscene += 1;
                        },
                        // 8 src:22b9
                        TaskTimedNameE::IncreaseStateIn3rdCutescene => {
                            g.state_in_third_cutscene += 1;
                        },
                    }
                }
            }
        }
    }

}