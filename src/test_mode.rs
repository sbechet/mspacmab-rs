/*
testMode():
    clear hardware_floating_sprite_t *_sprite  ( * 8 )
    setmem(tileRam,0x40)
    setmem(colorRam,0x0F)
    DrawText(MEMORY_OK)
    hardware_coin_lockout_global = 1
    hardware_flipscreen = false;
    while service switch on:
        game_mode = INIT;
        subroutine_INIT_state = 1;
        channel_1_effect.num = 2 if credit
        channel_1_effect.num = 1 if player1 or player2 button
        channel_3_effect.num = 8 if joystick up
        channel_3_effect.num = 4 if joystick left
        channel_3_effect.num = 0x10 if joystick right
        channel_3_effect.num = 0x20 if joystick down
        form hardware_DSW1 : 
            drawText(FREEPLAY) or drawText(1_COIN_1_CREDIT) or drawText(1_COIN_2_CREDITS) or drawText(2_COINS_1_CREDIT)
            drawText(BONUS_NONE) or drawText(BONUS) + drawText(000) + tileRam( (13,12), 10|15|20)
            lives on (18, 14) = 1, 2, 3 or 5
        drawText(MS_PAC_MEN)
        if Cocktail cabinet:
            drawText(TABLE)
        else
            drawText(UPRIGHT)
    disable coin lockout
    disable players start lamps
    disable flip screen
    disable sound
    disable interrupts
    print_grid()
    sleep 0,5 sec
    // Here: press buttons and switch on
    wait service switch off
    if not player 1 and player 2 buttons : goto main()
    // Here: switch on
    sleep 0,5 sec
    if service switch off : goto main()
    4 times :
        sleep 0,069 sec while not Joystick Up
        sleep 0,069 sec while not Joystick w/o position
    4 times :
        sleep 0,069 sec while not Joystick Left
        sleep 0,069 sec while not Joystick w/o position
    4 times :
        sleep 0,069 sec while not Joystick Right
        sleep 0,069 sec while not Joystick w/o position
    4 times :
        sleep 0,069 sec while not Joystick Down
        sleep 0,069 sec while not Joystick w/o position
    setmem(tileRam,0x40)
    draw_easter_egg__Made_By_Namco()
    wait service switch off
    goto main()


*/

use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    prelude::*,
};

use embedded_graphics_simulator::{
    SimulatorDisplay, 
};

use crate::tile::{TileId, Tile};

fn print_grid(display: &mut SimulatorDisplay<Rgb888>, palette_id: u8) {
    let ur = TileId::get_tile(&TileId::GridUpperRight, palette_id);
    let ul = TileId::get_tile(&TileId::GridUpperLeft, palette_id);
    let lr = TileId::get_tile(&TileId::GridLowerRight, palette_id);
    let ll = TileId::get_tile(&TileId::GridLowerLeft, palette_id);

    for y in 0..18 {
        for x in 0..14 {
            let iul: Image<Tile, Rgb888> = Image::new(&ul, Point::new(16*x,16*y));
            let iur: Image<Tile, Rgb888> = Image::new(&ur, Point::new(16*x+8,16*y));
            let ilr: Image<Tile, Rgb888> = Image::new(&lr, Point::new(16*x+8,16*y+8));
            let ill: Image<Tile, Rgb888> = Image::new(&ll, Point::new(16*x,16*y+8));
            iul.draw(display).unwrap();
            iur.draw(display).unwrap();
            ilr.draw(display).unwrap();
            ill.draw(display).unwrap();
        }
    }
}

fn clear_screen(display: &mut SimulatorDisplay<Rgb888>) {
    display.clear(Rgb888::BLACK).unwrap();
}

pub fn test_mode(display: &mut SimulatorDisplay<Rgb888>) {
    clear_screen(display);
    print_grid(display, 15);
}