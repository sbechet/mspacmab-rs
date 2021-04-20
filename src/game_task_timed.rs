use std::collections::VecDeque;
use embedded_graphics::prelude::*;

use crate::credits::Credits;
use crate::game_counter::{ Counter60Hz, CurrentTime };
use crate::game_attract::GameAttract;
use crate::game_playing::{ GamePlaying, SpriteName };
use crate::game_task::{GameTask, TaskCoreE };
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
}

#[derive(Copy, Clone, Debug)]
struct TaskTimedE {
    pub unit:CurrentTime,
    pub counter:u8,
    pub task:TaskTimedNameE,
}

pub struct GameTaskTimed {
    pub counter: Counter60Hz,
    timed_tasks: VecDeque<TaskTimedE>,
}

impl GameTaskTimed {
    pub fn new() -> GameTaskTimed {
        GameTaskTimed {
            counter: Counter60Hz::new(),
            timed_tasks: VecDeque::with_capacity(MAX_TASKTIMED),
        }
    }

    // src:0030
    pub fn add(&mut self, unit: CurrentTime, counter: u8, t: TaskTimedNameE) {
        let task = TaskTimedE {
            unit: unit,
            counter: counter,
            task: t,
        };
        self.timed_tasks.push_back(task);
    }

    // src:0221
    pub fn run(&mut self, tasks: &mut GameTask, playing: &mut GamePlaying, game_attract: &mut GameAttract, credits: &mut Credits) {
        for tasktimed in self.timed_tasks.iter_mut() {
            if tasktimed.unit != CurrentTime::None && tasktimed.unit <= self.counter.get() {
                tasktimed.counter -= 1;
                if tasktimed.counter == 0 {
                    println!("task_timed={:?}", tasktimed.task);
                    match tasktimed.task {
                        // 0 src:0894
                        TaskTimedNameE::IncreaseSubroutinePlayingState => {
                            playing.subroutine_playing_state += 1;
                        },        
                        // 1 src:06a3
                        TaskTimedNameE::IncreaseSubroutineCoinInsertedState => {
                            credits.subroutine_coin_inserted_state += 1;
                        },
                        // 2 src:058e
                        TaskTimedNameE::IncreaseSubroutineAttractState => {
                            game_attract.subroutine_attract_state += 1;
                        },
                        // 3 src:1272
                        TaskTimedNameE::IncreaseKilledGhostAnimationState => {
                            playing.killed_ghost_animation_state += 1;
                        },
                        // 4 src:1000
                        TaskTimedNameE::ClearFruitPoints => {
                            playing.fruit_points = 0;
                        },
                        // 5 src:100b
                        TaskTimedNameE::ClearFruitPosition => {
                            playing.sprite[SpriteName::Fruit as usize].p = Point::new(0,0);
                        },
                        // 6 src:0263
                        TaskTimedNameE::ClearReadyMessage => {
                            tasks.add(TaskCoreE::DrawTextOrGraphics(TextId::Ready, true) );
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