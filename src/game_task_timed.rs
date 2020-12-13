use std::collections::VecDeque;

use crate::game::Game;
use crate::game_task::GameTask;
use crate::game_counter::CurrentTime;
use crate::game_task::TaskCoreE;
use crate::text::TextId;

// as near as possible than original software
const MAX_TASKTIMED: usize = 16;

#[derive(Copy, Clone, Debug)]
pub enum TaskTimedNameE {
    IncreaseSubroutinePlayingState,             // 0 src:0894
    IncreaseSubroutineCoinInsertedState,        // 1 src:06a3
    IncreaseSubroutineAttractState,             // 2 src:058e
    IncreaseKilledGhostAnimationState,          // 3 src:1272
    ClearFruitPoints,                           // 4 src:1000
    ClearFruitPosition,                         // 5 src:100b
    ClearReadyMessage,                          // 6 src:0263
    IncreaseStateIn1stCutescene,                // 7 src:212b
    IncreaseStateIn2ndCutescene,                // 8 src:21f0
    IncreaseStateIn3rdCutescene,                // 9 src:22b9
}

#[derive(Copy, Clone, Debug)]
pub struct TaskTimedE {
    unit:CurrentTime,
    counter:u8,
    task:TaskTimedNameE,
}

pub trait GameTaskTimed {
    fn timed_task_new() -> VecDeque<TaskTimedE>;
    fn timed_task_add(&mut self, unit: CurrentTime, counter: u8, t: TaskTimedNameE);
    fn timed_task_execute(&mut self);
}

impl GameTaskTimed for Game {
    fn timed_task_new() -> VecDeque<TaskTimedE> {
        VecDeque::with_capacity(MAX_TASKTIMED)
    }

    // src:0030
    fn timed_task_add(&mut self, unit: CurrentTime, counter: u8, t: TaskTimedNameE) {
        let task = TaskTimedE {
            unit: unit,
            counter: counter,
            task: t,
        };
        self.timed_tasks.push_back(task);
    }

    // src:0221
    fn timed_task_execute(&mut self) {
        for tasktimed in self.timed_tasks.iter_mut() {
            if tasktimed.unit != CurrentTime::None && tasktimed.unit <= self.counter.get() {
                tasktimed.counter -= 1;
                if tasktimed.counter == 0 {
                    match tasktimed.task {
                        // 0 src:0894
                        TaskTimedNameE::IncreaseSubroutinePlayingState => {
                            self.subroutine_playing_state += 1;
                        },        
                        // 1 src:06a3
                        TaskTimedNameE::IncreaseSubroutineCoinInsertedState => {
                            self.subroutine_coin_inserted_state += 1;
                        },
                        // 2 src:058e
                        TaskTimedNameE::IncreaseSubroutineAttractState => {
                            self.subroutine_attract_state += 1;
                        },
                        // 3 src:1272
                        TaskTimedNameE::IncreaseKilledGhostAnimationState => {
                            self.killed_ghost_animation_state += 1;
                        },
                        // 4 src:1000
                        TaskTimedNameE::ClearFruitPoints => {
                            self.fruit_points = 0;
                        },
                        // 5 src:100b
                        TaskTimedNameE::ClearFruitPosition => {
                            self.fruit_coord = (0, 0);
                        },
                        // 6 src:0263
                        TaskTimedNameE::ClearReadyMessage => {
                            self.tasks.push_back(TaskCoreE::DrawTextOrGraphics(TextId::Ready, true) );
                        },
                        // 7 src:212b
                        TaskTimedNameE::IncreaseStateIn1stCutescene => {
                            self.state_in_first_cutscene += 1;
                        },
                        // 8 src:21f0
                        TaskTimedNameE::IncreaseStateIn2ndCutescene => {
                            self.state_in_second_cutscene += 1;
                        },
                        // 9 src:22b9
                        TaskTimedNameE::IncreaseStateIn3rdCutescene => {
                            self.state_in_third_cutscene += 1;
                        },
                    }
                }
            }
        }

        // to please borrow checker
        self.timed_tasks.retain(|tasktimed| {
            if tasktimed.counter == 0 {
                false
            } else {
                true
            }
        });

    }
}