use std::collections::VecDeque;
use crate::text::{TextId};
use crate::palette::{ColorE};

pub enum ScreenPart {
    All=0,
    Maze=1,
}

/*
    Param(u8),
    Text(TextId),
    Color(u8),

*/
pub enum TaskCoreE {
    ClearWholeScreenOrMaze(ScreenPart),     //  0 src:23ed
    SelectMazeColor(u8),                    //  1 src:24d7 (u8 playing_state) Verify:0:off, 1:playing, 2:flashing (xref:3e2, 462, 5f9, 677, 98b, 9ec)
    DrawMaze,                               //  2 src:2419
    DrawPellets,                            //  3 src:2448
    ResetSpritesToDefaultValues(bool),      //  4 src:253d (bool game_start)
    ResetGhostHomeCounter,                  //  5 src:268b
    ClearColorRam,                          //  6 src:240d (void)
    SetGameToDemoMode,                      //  7 src:2698 (void)
    RedGhostAi,                             //  8 src:2730
    PinkGhostAi,                            //  9 src:276c
    BlueGhostAi,                            // 10 src:27a9
    OrangeGhostAi,                          // 11 src:27f1
    RedGhostMovementWhenPowerPill,          // 12 src:283b
    PinkGhostMovementWhenPowerPill,         // 13 src:2865
    BlueGhostMovementWhenPowerPill,         // 14 src:288f
    OrangeGhostMovementWhenPowerPill,       // 15 src:28b9
    SetupDifficulty,                        // 16 src:070e
    ClearFullDataGame,                      // 17 src:26a2
    ClearsPillsAndPowerPills,               // 18 src:24c9
    ClearsSprites,                          // 19 src:2a35
    SetupConfigFromDipSwitches,             // 20 src:26d0 (void)
    UpdateScreenPillConfigToVideoRam,       // 21 src:2487
    IncreaseMainSubroutineNumber,           // 22 src:23e8
    PacmanAiMovementWhenDemo,               // 23 src:28e3
    ResetThenPrintPlayersScore,             // 24 src:2ae0 (void)
    UpdateScoreThenDraw,                    // 25 src:2a5a
    DrawRemainingLivesBottomLeftScreen,     // 26 src:2b6a
    DrawFruitsBottomRightScreen,            // 27 src:2bea
    DrawTextOrGraphics(TextId, bool),       // 28 src:95e3 (TextId textid, bool clear)
    DrawDrawCreditQty,                      // 29 src:2ba1
    ClearFruitAndPacmanPosition,            // 30 src:2675 (void)
    DrawExtraLifePoints,                    // 31 src:26b2
}

pub struct GameTask {
    /* src:4cc0 src:4c80 src:4c82 */
    tasks: VecDeque<TaskCoreE>,
}

impl GameTask {
    pub fn new() -> Self {
        GameTask {
            tasks: VecDeque::new(),
        }
    }

    /* src:0028 src:0042 */
    pub fn add_to_task_list(&mut self, action:TaskCoreE) {
        self.tasks.push_back(action);
    }

    pub fn get_from_task_list(&mut self) -> Option<TaskCoreE> {
        self.tasks.pop_front()
    }

}