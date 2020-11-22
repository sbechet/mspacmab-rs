use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

use crate::tile::{TileId};
use crate::palette::ColorE;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, FromPrimitive, ToPrimitive)]
pub enum FruitId {
    Cherry = 0,
    Strawberry = 1,
    Peach = 2,
    Pretzel = 3,
    Apple = 4,
    Pear = 5,
    Banana = 6,
    Key = 7,
}

// src:3b08
pub const FRUIT: [ (TileId, ColorE); 8] = [
    (TileId::CherryUR, ColorE::ColorMazeLevel6_7_8_9),
    (TileId::StrawberryUR, ColorE::ColorFruit),
    (TileId::PeachUR, ColorE::Brown),
    (TileId::PretzelUR, ColorE::Orange),
    (TileId::AppleUR, ColorE::ColorMazeLevel6_7_8_9),
    (TileId::PearUR, ColorE::ColorFruitPear),
    (TileId::BananaUR, ColorE::ColorMazeLevel3_4_5),
    (TileId::KeyUR, ColorE::ColorMazeLevel3_4_5),   // unused in ms.pac
];