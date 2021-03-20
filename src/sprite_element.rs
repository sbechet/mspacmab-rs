use embedded_graphics::{ geometry::Point };
use crate::sprite::{ SpriteId, Sprite };
use crate::palette::{ ColorE, PALETTE };
use crate::game_hw_video::SpriteElement;

impl SpriteElement {

    // pub fn new_unknown() -> SpriteElement {
        
    // }

    pub fn new_red_ghost() -> SpriteElement {
        SpriteElement {
            p: Point::new(0, 0),
            s: SpriteId::GhostRight1,
            c: ColorE::Red,
        }
    }
    
    pub fn new_pink_ghost() -> SpriteElement {
        SpriteElement {
            p: Point::new(0, 0),
            s: SpriteId::GhostRight1,
            c: ColorE::Pink,
        }
    }


    pub fn new_blue_ghost() -> SpriteElement {
        SpriteElement {
            p: Point::new(0, 0),
            s: SpriteId::GhostRight1,
            c: ColorE::Blue,
        }
    }


    pub fn new_orange_ghost() -> SpriteElement {
        SpriteElement {
            p: Point::new(0, 0),
            s: SpriteId::GhostRight1,
            c: ColorE::Orange,
        }
    }

    pub fn new_man() -> SpriteElement {
        SpriteElement {
            p: Point::new(0, 0),
            s: SpriteId::Stork0,
            c: ColorE::Yellow,
        }
    }

    pub fn new_fruit() -> SpriteElement {
        SpriteElement {
            p: Point::new(0, 0),
            s: SpriteId::FruitStart,
            c: ColorE::Black,
        }
    }
    
    // pub fn new_unknown2() -> SpriteElement {

    // }

}