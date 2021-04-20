# mspacmab-rs

long term goal: good mspacmab remake - a method as other to play rust-lang

see my `mspacmab` project for reverse part

WIP!

# Hardware

Main CPU : Z80

MASTER_CLOCK   18432000 Hz
Z80_CLOCK      3072000 Hz
PIXEL_CLOCK    6144000 Hz

screen : 264x384 but mspacmab really use 224x288

Sound Chip : Custom 3 channel 4-bit WSG

HBLANK = 6144000 / 384 = 16000 Hz
VBLANK = HBLANK / 264 = 60,60606060 Hz

Assume on average 4 clock cycles per instruction

# TODO

- [x] Tile
- [x] Sprites
- [x] DrawText
- [x] test_main_state
- [ ] Game { hardware_vars, game_vars }; impl Game { new(), run(), test_mode(), timed60Hz() }
- [ ] stm32f103 hardware using `stm32f103-vga-rs` project as video card (size ok: 296x364)
- [ ] split and cleanup code using more files (ghost, man, maze,...)


