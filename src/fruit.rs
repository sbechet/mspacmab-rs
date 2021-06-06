pub struct FruitEntrance<'a> {
    start: (i16, i16),
    path: &'a str,
}

pub struct FruitExit<'a> {
    path: &'a str,
}

pub struct Fruit<'a> {
    // src:4c40 - index used for moving fruit positions
    current_place_in_fruit_path: usize,
    // src:4c41 - index used for bounce frames (see BOUNCE_FRAMES)
    current_place_within_bounce: usize,
    // src:4c42 - ref to the path the fruit is currently following
    p_current_fruit_path: &'a str,
}

impl Fruit<'_> {
    pub fn new() -> Fruit<'static> {
        Fruit {
            current_place_in_fruit_path: 0,
            current_place_within_bounce: 0,
            p_current_fruit_path: &FRUIT_PATH_ENTRY_FOR_MAZE1_1,
        }
    }
}

// src:87f8
pub const FRUITS_PATHS_ENTRY: [&'static [FruitEntrance; 4]; 4] = [
    &FRUIT_PATHS_ENTRY_FOR_MAZE_1,
    &FRUIT_PATHS_ENTRY_FOR_MAZE_2,
    &FRUIT_PATHS_ENTRY_FOR_MAZE_3,
    &FRUIT_PATHS_ENTRY_FOR_MAZE_4,
];

// src:8800
pub const FRUITS_PATHS_EXIT: [&'static [FruitExit; 4]; 4] = [
    &FRUIT_PATHS_EXIT_FOR_MAZE_1,
    &FRUIT_PATHS_EXIT_FOR_MAZE_2,
    &FRUIT_PATHS_EXIT_FOR_MAZE_3,
    &FRUIT_PATHS_EXIT_FOR_MAZE_4,
];

// src:8808
// start: (15, 20)
pub const GHOST_PEN_PATH: &str = "<<<<<<^^^^^^>>>>>>>>>vvvvvv<<";

// src:8841
// A 16-frame animation for moving 8 pixels either up, down, left, or right.
pub const BOUNCE_FRAMES: [[(i8, i8); 16]; 4] = [
    [
        (-1, -1),
        (-1, -1),
        (-1, -1),
        (-1, -1),
        (-1, -1),
        (-1, -1),
        (-1, -1),
        (-1, -1),
        (-1, -1),
        (0, 0),
        (-1, -1),
        (0, 0),
        (0, 0),
        (0, 1),
        (0, 0),
        (0, 1),
    ],
    [
        (0, 0),
        (-2, -1),
        (0, 0),
        (-1, 0),
        (0, 0),
        (-2, -1),
        (0, 0),
        (-1, 0),
        (0, 0),
        (-1, 0),
        (0, 0),
        (-1, 0),
        (0, 0),
        (-1, 1),
        (-1, 1),
        (0, 0),
    ],
    [
        (0, 0),
        (0, 0),
        (0, -1),
        (0, 0),
        (1, 0),
        (0, 0),
        (0, -1),
        (0, 0),
        (1, 0),
        (0, 0),
        (1, 0),
        (0, 0),
        (1, 0),
        (0, 0),
        (1, 1),
        (1, 1),
    ],
    [
        (0, 0),
        (0, 1),
        (0, 1),
        (0, 1),
        (0, 1),
        (0, 1),
        (0, 1),
        (0, 1),
        (0, 1),
        (0, 1),
        (0, 1),
        (0, 1),
        (-1, -1),
        (-1, -1),
        (0, 0),
        (-1, -1),
    ],
];

// ---------------------------------------------------------------------------
// src:8b4f
// entrance fruit paths for maze 1
pub const FRUIT_PATHS_ENTRY_FOR_MAZE_1: [FruitEntrance; 4] = [
    FruitEntrance {
        start: (228, 164),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE1_1,
    },
    FruitEntrance {
        start: (-4, 164),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE1_2,
    },
    FruitEntrance {
        start: (-4, 92),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE1_3,
    },
    FruitEntrance {
        start: (228, 92),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE1_4,
    },
];
// src:8b63
pub const FRUIT_PATH_ENTRY_FOR_MAZE1_1: &str = "<vvv>>>>>>>>>^^^>>>>";
// src:8b68
pub const FRUIT_PATH_ENTRY_FOR_MAZE1_2: &str = ">>>>vvvvvv>>>>>>>>>>>>>>>^^^<<<^^^";
// src:8b71
pub const FRUIT_PATH_ENTRY_FOR_MAZE1_3: &str = ">>>>^^^^>>>vvvv>>>vvv>>>>>>>>>vvvvvv<<<";
// src:8b7b
pub const FRUIT_PATH_ENTRY_FOR_MAZE1_4: &str = "<<<<vvvvvvvvv<<<^^^<<<vvv<<<";
// ---------------------------------------------------------------------------
// src:8b82
// exit fruit paths for maze 1
pub const FRUIT_PATHS_EXIT_FOR_MAZE_1: [FruitExit; 4] = [
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE1_1,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE1_2,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE1_3,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE1_4,
    },
];
// src:8b94
pub const FRUIT_PATH_EXIT_FOR_MAZE1_1: &str = "<vvv>>>>>>>>>^^^>>>>";
// src:8b99
pub const FRUIT_PATH_EXIT_FOR_MAZE1_2: &str = "<<<<vvv<<<<<<<<<^^^<<<<";
// src:8b9f
pub const FRUIT_PATH_EXIT_FOR_MAZE1_3: &str = "<<<<<<<^^^^^^<<<<<<^^^<<<<";
// src:8ba6
pub const FRUIT_PATH_EXIT_FOR_MAZE1_4: &str = "<vvv>>>>>>>>>^^^^^^^^^^^^>>>>";

// ---------------------------------------------------------------------------
// src:8e40
// entrance fruit paths for maze 2
pub const FRUIT_PATHS_ENTRY_FOR_MAZE_2: [FruitEntrance; 4] = [
    FruitEntrance {
        start: (228, 212),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE2_1,
    },
    FruitEntrance {
        start: (-4, 212),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE2_2,
    },
    FruitEntrance {
        start: (-4, 36),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE2_3,
    },
    FruitEntrance {
        start: (228, 36),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE2_4,
    },
];
// src:8e54
pub const FRUIT_PATH_ENTRY_FOR_MAZE2_1: &str = "<<<<^^^<<<<<<<<^^^<";
// src:8e59
pub const FRUIT_PATH_ENTRY_FOR_MAZE2_2: &str = ">>>>^^^>>>>>>>>vvv>>>>>^^^^^^<";
// src:8e61
pub const FRUIT_PATH_ENTRY_FOR_MAZE2_3: &str = ">>>>>>>vvv>>>vvvvvvv>>>>>>>>>vvvvvv<<<";
// src:8e6b
pub const FRUIT_PATH_ENTRY_FOR_MAZE2_4: &str = "<<<<<<<vvv<<<vvvvvvvvvvvvv<<<";
// ---------------------------------------------------------------------------
// src:8e73
// exit fruit paths for maze 2
pub const FRUIT_PATHS_EXIT_FOR_MAZE_2: [FruitExit; 4] = [
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE2_1,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE2_2,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE2_3,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE2_4,
    },
];
// src:8e87
pub const FRUIT_PATH_EXIT_FOR_MAZE2_1: &str = "vvv>>>>>>>>vvv>>>>";
// src:8e8c
pub const FRUIT_PATH_EXIT_FOR_MAZE2_2: &str = "vvvvvv<<<<<^^^<<<<<<<<vvv<<<<";
// src:8e94
pub const FRUIT_PATH_EXIT_FOR_MAZE2_3: &str = "<<<<<<<^^^^^^^^^^^^^<<<^^^<<<<<<<";
// src:8e9d
pub const FRUIT_PATH_EXIT_FOR_MAZE2_4: &str = "vvv>>>>>^^^^^^^^^^>>>>>^^^^^^<<<<<^^^>>>>>>>";

// ---------------------------------------------------------------------------
// src:911a
// entrance fruit paths for maze 3
pub const FRUIT_PATHS_ENTRY_FOR_MAZE_3: [FruitEntrance; 4] = [
    FruitEntrance {
        start: (228, 100),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE3_1,
    },
    FruitEntrance {
        start: (-4, 100),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE3_2,
    },
    FruitEntrance {
        start: (-4, 100),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE3_2,
    },
    FruitEntrance {
        start: (228, 100),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE3_3,
    },
];
// src:912e
pub const FRUIT_PATH_ENTRY_FOR_MAZE3_1: &str = "<<<<<vv<<<<<vvvvvv<<<";
// src:9134
pub const FRUIT_PATH_ENTRY_FOR_MAZE3_2: &str = ">>>>>vv>>>>>>>>>>>>>>vvvvvv<<<";
// src:913c
pub const FRUIT_PATH_ENTRY_FOR_MAZE3_3: &str = "<<vvvvv<<<vvv<<<<<<<<";
// ---------------------------------------------------------------------------
// src:9142
// exit fruit paths for maze 3
pub const FRUIT_PATHS_EXIT_FOR_MAZE_3: [FruitExit; 4] = [
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE3_1,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE3_2,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE3_2,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE3_3,
    },
];
// src:9156
pub const FRUIT_PATH_EXIT_FOR_MAZE3_1: &str = "<vvv>>>vvv>>>^^^>>>>>^^^^^^^^^^^>>";
// src:915f
pub const FRUIT_PATH_EXIT_FOR_MAZE3_2: &str = "<<<<vvv<<<vvv<<<^^^<<<<<^^^^^^^^^^^<<";
// src:916f
pub const FRUIT_PATH_EXIT_FOR_MAZE3_3: &str = "<vvv>>>vvv>>>^^^^^^<<<^^^^^^>>>>>^^>>>>>";

// ---------------------------------------------------------------------------
// src:940a
// entrance fruit paths for maze 4
pub const FRUIT_PATHS_ENTRY_FOR_MAZE_4: [FruitEntrance; 4] = [
    FruitEntrance {
        start: (228, 156),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE4_1,
    },
    FruitEntrance {
        start: (-4, 156),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE4_2,
    },
    FruitEntrance {
        start: (-4, 132),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE4_2,
    },
    FruitEntrance {
        start: (228, 132),
        path: &FRUIT_PATH_ENTRY_FOR_MAZE4_3,
    },
];
// src:941e
pub const FRUIT_PATH_ENTRY_FOR_MAZE4_1: &str = "<<<<vv<<<vv<<<<<<^^^";
// src:9423
pub const FRUIT_PATH_ENTRY_FOR_MAZE4_2: &str = ">>>>vv>>>vv>>>>>>vvv>>>^^^^^^";
// src:942b
pub const FRUIT_PATH_ENTRY_FOR_MAZE4_3: &str = ">>>>^^^^^>>>^^^>>>vvv>>>vvv>>>>>>vvvvvv<<<";
// src:9436
pub const FRUIT_PATH_ENTRY_FOR_MAZE4_4: &str = "<<<<^^<<<vvv<<<vvv<<<";
// ---------------------------------------------------------------------------
// src:943c
// exit fruit paths for maze 4
pub const FRUIT_PATHS_EXIT_FOR_MAZE_4: [FruitExit; 4] = [
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE4_1,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE4_2,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE4_2,
    },
    FruitExit {
        path: &FRUIT_PATH_EXIT_FOR_MAZE4_3,
    },
];
// src:9450
pub const FRUIT_PATH_EXIT_FOR_MAZE4_1: &str = "<vvv>>>>>>^^>>>^^>>>>";
// src:9456
pub const FRUIT_PATH_EXIT_FOR_MAZE4_2: &str = "<<<<vvv<<<<<<^^<<<^^<<<<";
// src:945c
pub const FRUIT_PATH_EXIT_FOR_MAZE4_3: &str = "<<<<<<<^^^<<<^^^<<<vv<<<<";
// src:9463
pub const FRUIT_PATH_EXIT_FOR_MAZE4_4: &str = "<vvv>>>>>>^^^^^^^^^>>>vv>>>>";
// ---------------------------------------------------------------------------
